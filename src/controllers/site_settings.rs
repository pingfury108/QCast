#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::{site_settings, users};

const DEFAULT_SITE_URL: &str = "http://localhost:5150";

#[derive(Debug, Serialize, Deserialize)]
pub struct SiteSettingsResponse {
    pub id: i32,
    pub site_url: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSiteSettingsParams {
    pub site_url: String,
}

/// 获取站点设置
#[debug_handler]
pub async fn get_settings(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    // 验证管理员权限
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    if !user.is_admin() {
        return unauthorized("Admin permission required");
    }

    // 获取或创建站点设置
    let settings = site_settings::Model::get_or_create(&ctx.db, DEFAULT_SITE_URL).await?;

    let response = SiteSettingsResponse {
        id: settings.id,
        site_url: settings.site_url,
        created_at: settings.created_at.into(),
        updated_at: settings.updated_at.into(),
    };

    format::json(response)
}

/// 更新站点设置
#[debug_handler]
pub async fn update_settings(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<UpdateSiteSettingsParams>,
) -> Result<Response> {
    // 验证管理员权限
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    if !user.is_admin() {
        return unauthorized("Admin permission required");
    }

    // 更新站点URL
    let settings = site_settings::Model::update_url(&ctx.db, params.site_url).await?;

    let response = SiteSettingsResponse {
        id: settings.id,
        site_url: settings.site_url,
        created_at: settings.created_at.into(),
        updated_at: settings.updated_at.into(),
    };

    format::json(response)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/site-settings")
        .add("", get(get_settings))
        .add("", put(update_settings))
}
