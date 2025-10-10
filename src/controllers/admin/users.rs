use crate::models::_entities::users as users_entity;
use crate::models::users;
use axum::extract::Query;
use axum::routing::method_routing::delete as axum_delete;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ListParams {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
    pub search: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRoleParams {
    pub is_staff: bool,
    pub is_superuser: bool,
}

#[derive(Debug, Serialize)]
pub struct UserListResponse {
    pub users: Vec<users::Model>,
    pub pagination: PaginationInfo,
}

#[derive(Debug, Serialize)]
pub struct PaginationInfo {
    pub page: u64,
    pub per_page: u64,
    pub total_pages: u64,
}

/// 列出所有用户（需要管理员权限）
pub async fn list(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Query(params): Query<ListParams>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    if !user.is_admin() {
        return unauthorized("需要管理员权限");
    }

    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(20);

    let (user_list, total_pages) = if let Some(keyword) = params.search {
        users::Model::search(&ctx.db, &keyword, page, per_page).await?
    } else {
        users::Model::list_all(&ctx.db, page, per_page).await?
    };

    format::json(UserListResponse {
        users: user_list,
        pagination: PaginationInfo {
            page,
            per_page,
            total_pages,
        },
    })
}

/// 更新用户角色（需要超级管理员权限）
pub async fn update_role(
    auth: auth::JWT,
    Path(user_id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<UpdateRoleParams>,
) -> Result<Response> {
    let admin = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    if !admin.is_super_admin() {
        return unauthorized("需要超级管理员权限");
    }

    let updated_user =
        users::Model::update_admin_status(&ctx.db, user_id, params.is_staff, params.is_superuser)
            .await?;

    format::json(updated_user)
}

/// 删除用户（需要超级管理员权限）
pub async fn delete(
    auth: auth::JWT,
    Path(user_id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let admin = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    if !admin.is_super_admin() {
        return unauthorized("需要超级管理员权限");
    }

    // 不允许删除自己
    if admin.id == user_id {
        return bad_request("不能删除自己");
    }

    users_entity::Entity::delete_by_id(user_id)
        .exec(&ctx.db)
        .await?;

    format::empty()
}

/// 获取统计信息
pub async fn stats(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    if !user.is_admin() {
        return unauthorized("需要管理员权限");
    }

    let total_users = users::Model::count_all(&ctx.db).await?;
    let total_admins = users::Model::count_admins(&ctx.db).await?;

    format::json(serde_json::json!({
        "total_users": total_users,
        "total_admins": total_admins,
    }))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/users")
        .add("/", get(list))
        .add("/stats", get(stats))
        .add("/{id}/role", put(update_role))
        .add("/{id}", axum_delete(delete))
}
