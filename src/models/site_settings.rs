use loco_rs::prelude::*;
use sea_orm::ActiveValue;
use serde::{Deserialize, Serialize};

pub use super::_entities::site_settings::{self, ActiveModel, Entity, Model};

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateSiteUrlParams {
    pub site_url: String,
}

impl Model {
    /// 获取站点设置，如果不存在则创建默认设置
    ///
    /// # Errors
    ///
    /// When database query fails
    pub async fn get_or_create(db: &DatabaseConnection, default_url: &str) -> ModelResult<Self> {
        // 尝试获取第一条记录（应该只有一条）
        if let Some(settings) = site_settings::Entity::find_by_id(1).one(db).await? {
            return Ok(settings);
        }

        // 如果不存在，创建默认设置
        let now = chrono::Utc::now();
        let settings = site_settings::ActiveModel {
            id: ActiveValue::Set(1),
            site_url: ActiveValue::Set(default_url.to_string()),
            created_at: ActiveValue::Set(now.into()),
            updated_at: ActiveValue::Set(now.into()),
        }
        .insert(db)
        .await?;

        Ok(settings)
    }

    /// 更新站点URL
    ///
    /// # Errors
    ///
    /// When database query fails or URL validation fails
    pub async fn update_url(db: &DatabaseConnection, new_url: String) -> ModelResult<Self> {
        // 验证URL格式
        validate_site_url(&new_url).map_err(|e| ModelError::Any(e.into()))?;

        // 获取或创建设置
        let settings = Self::get_or_create(db, &new_url).await?;

        // 更新URL
        let mut active: ActiveModel = settings.into();
        active.site_url = ActiveValue::Set(normalize_url(&new_url));
        active.update(db).await.map_err(ModelError::from)
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for super::_entities::site_settings::ActiveModel {
    async fn before_save<C>(self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        // 验证 site_url
        if let ActiveValue::Set(ref url) = self.site_url {
            validate_site_url(url).map_err(DbErr::Custom)?;
        }
        Ok(self)
    }
}

/// 验证站点URL格式
fn validate_site_url(url: &str) -> Result<(), String> {
    if url.trim().is_empty() {
        return Err("Site URL cannot be empty".to_string());
    }

    // 必须以 http:// 或 https:// 开头
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err("Site URL must start with http:// or https://".to_string());
    }

    // 验证是否是有效的URL
    url::Url::parse(url).map_err(|_| "Invalid URL format".to_string())?;

    Ok(())
}

/// 标准化URL（去除尾部斜杠）
fn normalize_url(url: &str) -> String {
    url.trim_end_matches('/').to_string()
}
