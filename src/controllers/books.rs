#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use axum::extract::Query;
use axum::routing::method_routing::delete as axum_delete;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::_entities::books::{ActiveModel, Column, Entity, Model};
use crate::models::users;
use crate::views::books::{BookResponse, BookTreeResponse};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateParams {
    pub title: String,
    pub description: Option<String>,
    pub cover_image: Option<String>,
    pub parent_id: Option<i32>,
    pub sort_order: Option<i32>,
    pub is_public: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateParams {
    pub title: Option<String>,
    pub description: Option<String>,
    pub cover_image: Option<String>,
    pub parent_id: Option<i32>,
    pub sort_order: Option<i32>,
    pub is_public: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReorderParams {
    pub sort_order: i32,
}

async fn load_item(ctx: &AppContext, id: i32, user_id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id)
        .filter(Column::UserId.eq(user_id))
        .one(&ctx.db)
        .await?;
    item.ok_or_else(|| Error::NotFound)
}

/// 获取当前用户的所有书籍
#[debug_handler]
pub async fn list(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let books = Entity::find()
        .filter(Column::UserId.eq(user.id))
        .all(&ctx.db)
        .await?;

    let responses: Vec<BookResponse> = books.into_iter().map(BookResponse::from).collect();

    format::json(responses)
}

/// 创建书籍
#[debug_handler]
pub async fn create(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<CreateParams>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let item = ActiveModel {
        user_id: Set(user.id),
        title: Set(params.title),
        description: Set(params.description),
        cover_image: Set(params.cover_image),
        parent_id: Set(params.parent_id),
        sort_order: Set(params.sort_order),
        is_public: Set(params.is_public),
        ..Default::default()
    };

    let item = item.insert(&ctx.db).await?;

    format::json(BookResponse::from(item))
}

/// 获取书籍详情
#[debug_handler]
pub async fn show(
    auth: auth::JWT,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let item = load_item(&ctx, id, user.id).await?;

    format::json(BookResponse::from(item))
}

/// 更新书籍
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
    if let Some(cover_image) = params.cover_image {
        item.cover_image = Set(Some(cover_image));
    }
    if let Some(parent_id) = params.parent_id {
        item.parent_id = Set(Some(parent_id));
    }
    if let Some(sort_order) = params.sort_order {
        item.sort_order = Set(Some(sort_order));
    }
    if let Some(is_public) = params.is_public {
        item.is_public = Set(Some(is_public));
    }

    let item = item.update(&ctx.db).await?;

    format::json(BookResponse::from(item))
}

/// 删除书籍
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

/// 搜索书籍
#[debug_handler]
pub async fn search(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let query = params.get("q").map(|q| q.as_str()).unwrap_or("");

    if query.is_empty() {
        return format::json(Vec::<BookResponse>::new());
    }

    let books = Model::search(&ctx.db, user.id, query).await?;
    let responses: Vec<BookResponse> = books.into_iter().map(BookResponse::from).collect();

    format::json(responses)
}

/// 获取书籍的完整树形结构
#[debug_handler]
pub async fn tree(
    auth: auth::JWT,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    // 验证用户是否有权限访问该书籍
    let _ = load_item(&ctx, id, user.id).await?;

    let book_tree = Model::get_tree(&ctx.db, id).await?;
    let tree_response = BookTreeResponse::from(book_tree);
    format::json(tree_response)
}

/// 调整书籍顺序
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

    format::json(BookResponse::from(item))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/books/")
        .add("/", get(list))
        .add("/", post(create))
        .add("/search", get(search))
        .add("/{id}", get(show))
        .add("/{id}", put(update))
        .add("/{id}", patch(update))
        .add("/{id}", axum_delete(delete))
        .add("/{id}/tree", get(tree))
        .add("/{id}/reorder", post(reorder))
}
