#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use axum::extract::Query;
use axum::routing::method_routing::delete as axum_delete;
use loco_rs::prelude::*;

use crate::models::_entities::medias::{ActiveModel, Column, Entity, Model};
use crate::models::users;
use crate::views::medias::{MediaResponse, UpdateMediaParams};

async fn load_item(ctx: &AppContext, id: i32, user_id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id)
        .filter(Column::UserId.eq(user_id))
        .one(&ctx.db)
        .await?;
    item.ok_or_else(|| Error::NotFound)
}

/// 获取当前用户的所有媒体
#[debug_handler]
pub async fn list(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let medias = Entity::find()
        .filter(Column::UserId.eq(user.id))
        .all(&ctx.db)
        .await?;

    let responses: Vec<MediaResponse> = medias.into_iter().map(MediaResponse::from).collect();

    format::json(responses)
}

/// 获取媒体详情
#[debug_handler]
pub async fn show(
    auth: auth::JWT,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let item = load_item(&ctx, id, user.id).await?;

    format::json(MediaResponse::from(item))
}

/// 更新媒体
#[debug_handler]
pub async fn update(
    auth: auth::JWT,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<UpdateMediaParams>,
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
    if let Some(is_public) = params.is_public {
        item.is_public = Set(is_public);
    }

    let item = item.update(&ctx.db).await?;

    format::json(MediaResponse::from(item))
}

/// 删除媒体
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

/// 搜索媒体
#[debug_handler]
pub async fn search(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let query = params.get("q").map(|q| q.as_str()).unwrap_or("");

    if query.is_empty() {
        return format::json(Vec::<MediaResponse>::new());
    }

    let medias = Model::search(&ctx.db, user.id, query).await?;
    let responses: Vec<MediaResponse> = medias.into_iter().map(MediaResponse::from).collect();

    format::json(responses)
}

/// 发布/取消发布媒体
#[debug_handler]
pub async fn publish(
    auth: auth::JWT,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let _ = load_item(&ctx, id, user.id).await?;

    let media = ActiveModel::toggle_public(&ctx.db, id, user.id).await?;

    format::json(MediaResponse::from(media))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/media")
        .add("/", get(list))
        .add("/search", get(search))
        .add("/{id}", get(show))
        .add("/{id}", put(update))
        .add("/{id}", patch(update))
        .add("/{id}", axum_delete(delete))
        .add("/{id}/publish", post(publish))
}
