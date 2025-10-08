#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use axum::routing::method_routing::delete as axum_delete;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::_entities::chapters::{ActiveModel, Entity, Model};
use crate::models::books;
use crate::models::users;
use crate::views::chapters::ChapterResponse;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateParams {
    pub title: String,
    pub description: Option<String>,
    pub sort_order: Option<i32>,
    pub parent_id: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateParams {
    pub title: Option<String>,
    pub description: Option<String>,
    pub sort_order: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReorderParams {
    pub sort_order: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BatchReorderParams {
    pub chapter_ids: Vec<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateChildParams {
    pub title: String,
    pub description: Option<String>,
    pub sort_order: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MoveChapterParams {
    pub new_parent_id: Option<i32>,
    pub new_sort_order: Option<i32>,
}

async fn load_item(ctx: &AppContext, id: i32, user_id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;

    if let Some(ref chapter) = item {
        // 验证书籍是否属于当前用户
        let book = books::Entity::find_by_id(chapter.book_id)
            .filter(books::Column::UserId.eq(user_id))
            .one(&ctx.db)
            .await?;

        if book.is_none() {
            return Err(Error::NotFound);
        }
    }

    item.ok_or_else(|| Error::NotFound)
}


/// 获取书籍的章节列表
#[debug_handler]
pub async fn list(
    auth: auth::JWT,
    Path(book_id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    // 验证用户是否有权限访问该书籍
    let _book = books::Entity::find_by_id(book_id)
        .filter(books::Column::UserId.eq(user.id))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    let chapters_with_media = Model::find_by_book_with_media_count(&ctx.db, book_id).await?;
    let responses: Vec<ChapterResponse> = chapters_with_media.into_iter().map(|(chapter, media_count)| {
        ChapterResponse::from_with_media_count(chapter, media_count)
    }).collect();

    format::json(responses)
}

/// 创建章节
#[debug_handler]
pub async fn create(
    auth: auth::JWT,
    Path(book_id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<CreateParams>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    // 验证用户是否有权限访问该书籍
    let _book = books::Entity::find_by_id(book_id)
        .filter(books::Column::UserId.eq(user.id))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    let item = if let Some(parent_chapter_id) = params.parent_id {
        // 创建子章节
        if let Some(sort_order) = params.sort_order {
            // 使用指定的排序号
            ActiveModel {
                book_id: Set(book_id),
                parent_id: Set(Some(parent_chapter_id)),
                title: Set(params.title),
                description: Set(params.description),
                sort_order: Set(Some(sort_order)),
                ..Default::default()
            }
            .insert(&ctx.db)
            .await?
        } else {
            // 自动获取同级下一个排序号
            let item = ActiveModel::create_child(&ctx.db, book_id, parent_chapter_id, params.title, params.description)
                .await?;
            item.insert(&ctx.db).await?
        }
    } else if let Some(sort_order) = params.sort_order {
        // 创建顶级章节，使用指定的排序号
        ActiveModel {
            book_id: Set(book_id),
            title: Set(params.title),
            description: Set(params.description),
            sort_order: Set(Some(sort_order)),
            ..Default::default()
        }
        .insert(&ctx.db)
        .await?
    } else {
        // 创建顶级章节，自动获取下一个排序号
        let item =
            ActiveModel::create_with_order(&ctx.db, book_id, params.title, params.description)
                .await?;
        item.insert(&ctx.db).await?
    };

    // 更新层级和路径信息
    crate::models::chapters::Model::update_level_and_path(&ctx.db, item.id, params.parent_id).await?;

    // 重新加载更新的数据
    let updated_item = crate::models::_entities::chapters::Entity::find_by_id(item.id)
        .one(&ctx.db)
        .await?
        .ok_or(Error::NotFound)?;

    format::json(ChapterResponse::from(updated_item))
}

/// 获取章节详情
#[debug_handler]
pub async fn show(
    auth: auth::JWT,
    Path((_book_id, id)): Path<(i32, i32)>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let item = load_item(&ctx, id, user.id).await?;

    format::json(ChapterResponse::from(item))
}

/// 更新章节
#[debug_handler]
pub async fn update(
    auth: auth::JWT,
    Path((_book_id, id)): Path<(i32, i32)>,
    State(ctx): State<AppContext>,
    Json(params): Json<UpdateParams>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let item = load_item(&ctx, id, user.id).await?;

    let mut item = item.into_active_model();

    if let Some(title) = params.title {
        item.title = Set(title);
    }
    if let Some(description) = params.description {
        item.description = Set(Some(description));
    }
    if let Some(sort_order) = params.sort_order {
        item.sort_order = Set(Some(sort_order));
    }

    let item = item.update(&ctx.db).await?;

    format::json(ChapterResponse::from(item))
}

/// 删除章节
#[debug_handler]
pub async fn delete(
    auth: auth::JWT,
    Path((_book_id, id)): Path<(i32, i32)>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let item = load_item(&ctx, id, user.id).await?;

    item.delete(&ctx.db).await?;
    format::empty()
}

/// 调整章节顺序
#[debug_handler]
pub async fn reorder(
    auth: auth::JWT,
    Path((_book_id, id)): Path<(i32, i32)>,
    State(ctx): State<AppContext>,
    Json(params): Json<ReorderParams>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let item = load_item(&ctx, id, user.id).await?;

    let mut item = item.into_active_model();
    item.sort_order = Set(Some(params.sort_order));
    let item = item.update(&ctx.db).await?;

    format::json(ChapterResponse::from(item))
}

/// 批量重排序章节
#[debug_handler]
pub async fn batch_reorder(
    auth: auth::JWT,
    Path(book_id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<BatchReorderParams>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    // 验证用户是否有权限访问该书籍
    let _book = books::Entity::find_by_id(book_id)
        .filter(books::Column::UserId.eq(user.id))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    Model::reorder_all(&ctx.db, book_id, &params.chapter_ids).await?;

    let chapters = Model::find_by_book(&ctx.db, book_id).await?;
    let responses: Vec<ChapterResponse> = chapters.into_iter().map(ChapterResponse::from).collect();

    format::json(responses)
}

/// 章节上移
#[debug_handler]
pub async fn move_up(
    auth: auth::JWT,
    Path((_book_id, id)): Path<(i32, i32)>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let _item = load_item(&ctx, id, user.id).await?;

    let moved = Model::move_up(&ctx.db, id).await?;

    if !moved {
        return Err(Error::Message("Chapter is already at the top".to_string()));
    }

    let updated_item = Entity::find_by_id(id).one(&ctx.db).await?.unwrap();
    format::json(ChapterResponse::from(updated_item))
}

/// 章节下移
#[debug_handler]
pub async fn move_down(
    auth: auth::JWT,
    Path((_book_id, id)): Path<(i32, i32)>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let _item = load_item(&ctx, id, user.id).await?;

    let moved = Model::move_down(&ctx.db, id).await?;

    if !moved {
        return Err(Error::Message(
            "Chapter is already at the bottom".to_string(),
        ));
    }

    let updated_item = Entity::find_by_id(id).one(&ctx.db).await?.unwrap();
    format::json(ChapterResponse::from(updated_item))
}

/// 获取书籍的章节树状结构
#[debug_handler]
pub async fn tree(
    auth: auth::JWT,
    Path(book_id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    // 验证用户是否有权限访问该书籍
    let _book = books::Entity::find_by_id(book_id)
        .filter(books::Column::UserId.eq(user.id))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    let chapter_tree = Model::get_book_tree(&ctx.db, book_id).await?;
    format::json(chapter_tree)
}

/// 获取章节的扁平列表（包含层级信息）
#[debug_handler]
pub async fn flat_list(
    auth: auth::JWT,
    Path(book_id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    // 验证用户是否有权限访问该书籍
    let _book = books::Entity::find_by_id(book_id)
        .filter(books::Column::UserId.eq(user.id))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    let flat_list = Model::get_flat_list_with_level(&ctx.db, book_id).await?;
    format::json(flat_list)
}

/// 获取章节的子章节
#[debug_handler]
pub async fn children(
    auth: auth::JWT,
    Path((_book_id, id)): Path<(i32, i32)>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let _chapter = load_item(&ctx, id, user.id).await?;

    let children = Model::find_children(&ctx.db, id).await?;
    let responses: Vec<ChapterResponse> = children.into_iter().map(ChapterResponse::from).collect();

    format::json(responses)
}

/// 创建子章节
#[debug_handler]
pub async fn create_child(
    auth: auth::JWT,
    Path((_book_id, id)): Path<(i32, i32)>,
    State(ctx): State<AppContext>,
    Json(params): Json<CreateChildParams>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let parent_chapter = load_item(&ctx, id, user.id).await?;

    let item = if let Some(sort_order) = params.sort_order {
        // 使用指定的排序号
        ActiveModel {
            book_id: Set(parent_chapter.book_id),
            parent_id: Set(Some(id)),
            title: Set(params.title),
            description: Set(params.description),
            sort_order: Set(Some(sort_order)),
            ..Default::default()
        }
        .insert(&ctx.db)
        .await?
    } else {
        // 自动获取同级下一个排序号
        let item = ActiveModel::create_child(&ctx.db, parent_chapter.book_id, id, params.title, params.description)
            .await?;
        item.insert(&ctx.db).await?
    };

    // 更新层级和路径信息
    Model::update_level_and_path(&ctx.db, item.id, Some(id)).await?;

    // 重新加载更新的数据
    let updated_item = crate::models::_entities::chapters::Entity::find_by_id(item.id)
        .one(&ctx.db)
        .await?
        .ok_or(Error::NotFound)?;

    format::json(ChapterResponse::from(updated_item))
}

/// 移动章节到新的父级
#[debug_handler]
pub async fn move_chapter(
    auth: auth::JWT,
    Path((_book_id, id)): Path<(i32, i32)>,
    State(ctx): State<AppContext>,
    Json(params): Json<MoveChapterParams>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let _chapter = load_item(&ctx, id, user.id).await?;

    // 如果指定了新的父级，验证父级章节是否存在且属于同一本书
    if let Some(new_parent_id) = params.new_parent_id {
        let parent_chapter = Entity::find_by_id(new_parent_id).one(&ctx.db).await?;
        if let Some(parent_chapter) = parent_chapter {
            // 验证书籍是否属于当前用户
            let _book = books::Entity::find_by_id(parent_chapter.book_id)
                .filter(books::Column::UserId.eq(user.id))
                .one(&ctx.db)
                .await?
                .ok_or_else(|| Error::NotFound)?;
        } else {
            return Err(Error::NotFound);
        }
    }

      if let Err(e) = Model::move_to_parent(&ctx.db, id, params.new_parent_id, params.new_sort_order).await {
        // 处理循环引用错误
        match e {
            sea_orm::DbErr::Custom(ref msg) if msg.contains("cycle") => {
                return Err(Error::BadRequest("Moving chapter would create a cycle".to_string()));
            }
            _ => return Err(e.into()),
        }
    }

    let updated_item = Entity::find_by_id(id).one(&ctx.db).await?.unwrap();
    format::json(ChapterResponse::from(updated_item))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/books")
        .add("/{book_id}/chapters", get(list))
        .add("/{book_id}/chapters", post(create))
        .add("/{book_id}/chapters/batch-reorder", post(batch_reorder))
        .add("/{book_id}/chapters/tree", get(tree))
        .add("/{book_id}/chapters/flat", get(flat_list))
        .add("/{book_id}/chapters/{id}/reorder", post(reorder))
        .add("/{book_id}/chapters/{id}/move-up", post(move_up))
        .add("/{book_id}/chapters/{id}/move-down", post(move_down))
        .add("/{book_id}/chapters/{id}/children", get(children))
        .add("/{book_id}/chapters/{id}/children", post(create_child))
        .add("/{book_id}/chapters/{id}/move", post(move_chapter))
        .add("/{book_id}/chapters/{id}", get(show))
        .add("/{book_id}/chapters/{id}", put(update))
        .add("/{book_id}/chapters/{id}", patch(update))
        .add("/{book_id}/chapters/{id}", axum_delete(delete))
}
