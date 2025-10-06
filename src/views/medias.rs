use serde::{Deserialize, Serialize};
use crate::models::_entities::medias::Model;

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaResponse {
    pub id: i32,
    pub title: Option<String>,
    pub description: Option<String>,
    pub file_type: Option<String>,
    pub file_path: Option<String>,
    pub file_size: Option<i64>,
    pub duration: Option<i32>,
    pub mime_type: Option<String>,
    pub access_token: Option<String>,
    pub access_url: Option<String>,
    pub qr_code_path: Option<String>,
    pub file_version: Option<i32>,
    pub original_filename: Option<String>,
    pub play_count: Option<i64>,
    pub is_public: Option<bool>,
    pub chapter_id: Option<i32>,
    pub book_id: i32,
    pub user_id: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<Model> for MediaResponse {
    fn from(media: Model) -> Self {
        Self {
            id: media.id,
            title: media.title,
            description: media.description,
            file_type: media.file_type,
            file_path: media.file_path,
            file_size: media.file_size,
            duration: media.duration,
            mime_type: media.mime_type,
            access_token: media.access_token,
            access_url: media.access_url,
            qr_code_path: media.qr_code_path,
            file_version: media.file_version,
            original_filename: media.original_filename,
            play_count: media.play_count,
            is_public: media.is_public,
            chapter_id: media.chapter_id,
            book_id: media.book_id,
            user_id: media.user_id,
            created_at: media.created_at.into(),
            updated_at: media.updated_at.into(),
        }
    }
}

/// 公开访问的媒体响应（不包含敏感信息）
#[derive(Debug, Serialize, Deserialize)]
pub struct PublicMediaResponse {
    pub id: i32,
    pub title: Option<String>,
    pub description: Option<String>,
    pub file_type: Option<String>,
    pub duration: Option<i32>,
    pub mime_type: Option<String>,
    pub original_filename: Option<String>,
    pub play_count: Option<i64>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<Model> for PublicMediaResponse {
    fn from(media: Model) -> Self {
        Self {
            id: media.id,
            title: media.title,
            description: media.description,
            file_type: media.file_type,
            duration: media.duration,
            mime_type: media.mime_type,
            original_filename: media.original_filename,
            play_count: media.play_count,
            created_at: media.created_at.into(),
        }
    }
}