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

async fn load_item(ctx: &AppContext, id: i32, user_id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;

    if let Some(ref chapter) = item {
        // 验证书籍是否属于当前用户
        let book = books::Entity::find_by_id(chapter.book_id)
            .filter(books::Column::UserIdId.eq(user_id))
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
        .filter(books::Column::UserIdId.eq(user.id))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    let chapters = Model::find_by_book(&ctx.db, book_id).await?;
    let responses: Vec<ChapterResponse> = chapters.into_iter().map(ChapterResponse::from).collect();

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
        .filter(books::Column::UserIdId.eq(user.id))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    let item = if let Some(sort_order) = params.sort_order {
        // 使用指定的排序号
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
        // 自动获取下一个排序号
        let item =
            ActiveModel::create_with_order(&ctx.db, book_id, params.title, params.description)
                .await?;
        item.insert(&ctx.db).await?
    };

    format::json(ChapterResponse::from(item))
}

/// 获取章节详情
#[debug_handler]
pub async fn show(
    auth: auth::JWT,
    Path(id): Path<i32>,
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
    Path(id): Path<i32>,
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
    Path(id): Path<i32>,
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
    Path(id): Path<i32>,
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
        .filter(books::Column::UserIdId.eq(user.id))
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
    Path(id): Path<i32>,
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
    Path(id): Path<i32>,
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

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/books/{book_id}/chapters/")
        .add("/", get(list))
        .add("/", post(create))
        .add("/batch-reorder", post(batch_reorder))
        .add("/{id}", get(show))
        .add("/{id}", put(update))
        .add("/{id}", patch(update))
        .add("/{id}", axum_delete(delete))
        .add("/{id}/reorder", post(reorder))
        .add("/{id}/move-up", post(move_up))
        .add("/{id}/move-down", post(move_down))
}
