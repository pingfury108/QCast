use crate::models::_entities::{user_groups as user_groups_entity, users as users_entity};
use crate::models::{user_groups, users};
use axum::routing::method_routing::delete as axum_delete;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateGroupParams {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateGroupParams {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AddMemberParams {
    pub user_id: i32,
}

#[derive(Debug, Serialize)]
pub struct GroupWithMembers {
    #[serde(flatten)]
    pub group: user_groups::Model,
    pub member_count: u64,
}

/// 创建用户组（需要管理员权限）
pub async fn create(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<CreateGroupParams>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    if !user.is_admin() {
        return unauthorized("需要管理员权限");
    }

    let group = user_groups::Model::create_group(&ctx.db, params.name, params.description).await?;

    format::json(group)
}

/// 列出所有用户组
pub async fn list(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    if !user.is_admin() {
        return unauthorized("需要管理员权限");
    }

    let groups = user_groups::Model::list_all(&ctx.db).await?;

    // 获取每个组的成员数量
    let mut groups_with_count = Vec::new();
    for group in groups {
        let member_count = user_groups::Model::count_members(&ctx.db, group.id).await?;
        groups_with_count.push(GroupWithMembers {
            group,
            member_count,
        });
    }

    format::json(groups_with_count)
}

/// 获取用户组详情
pub async fn show(
    auth: auth::JWT,
    Path(group_id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    if !user.is_admin() {
        return unauthorized("需要管理员权限");
    }

    let group = user_groups::Model::find_by_id(&ctx.db, group_id).await?;
    let members = user_groups::Model::get_members(&ctx.db, group_id).await?;
    let member_count = members.len() as u64;

    format::json(serde_json::json!({
        "group": group,
        "members": members,
        "member_count": member_count,
    }))
}

/// 添加用户到组
pub async fn add_member(
    auth: auth::JWT,
    Path(group_id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<AddMemberParams>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    if !user.is_admin() {
        return unauthorized("需要管理员权限");
    }

    // 检查用户是否存在
    users_entity::Entity::find_by_id(params.user_id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    // 检查组是否存在
    user_groups::Model::find_by_id(&ctx.db, group_id).await?;

    // 检查用户是否已在组内
    if user_groups::Model::is_user_in_group(&ctx.db, group_id, params.user_id).await? {
        return bad_request("用户已在该组内");
    }

    user_groups::Model::add_user(&ctx.db, group_id, params.user_id).await?;

    format::empty()
}

/// 从组中移除用户
pub async fn remove_member(
    auth: auth::JWT,
    Path((group_id, user_id)): Path<(i32, i32)>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let admin = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    if !admin.is_admin() {
        return unauthorized("需要管理员权限");
    }

    user_groups::Model::remove_user(&ctx.db, group_id, user_id).await?;

    format::empty()
}

/// 删除用户组
pub async fn delete(
    auth: auth::JWT,
    Path(group_id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    if !user.is_super_admin() {
        return unauthorized("需要超级管理员权限");
    }

    user_groups_entity::Entity::delete_by_id(group_id)
        .exec(&ctx.db)
        .await?;

    format::empty()
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/groups")
        .add("/", get(list))
        .add("/", post(create))
        .add("/{id}", get(show))
        .add("/{id}", axum_delete(delete))
        .add("/{id}/members", post(add_member))
        .add("/{id}/members/{user_id}", axum_delete(remove_member))
}
