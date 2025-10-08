use crate::models::_entities::chapters::Model;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChapterResponse {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub sort_order: Option<i32>,
    pub book_id: i32,
    pub media_count: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<Model> for ChapterResponse {
    fn from(chapter: Model) -> Self {
        Self {
            id: chapter.id,
            title: chapter.title,
            description: chapter.description,
            sort_order: chapter.sort_order,
            book_id: chapter.book_id,
            media_count: 0, // 默认值，需要从其他地方计算
            created_at: chapter.created_at.naive_utc().and_utc(),
            updated_at: chapter.updated_at.naive_utc().and_utc(),
        }
    }
}

impl From<(Model, i64)> for ChapterResponse {
    fn from((chapter, media_count): (Model, i64)) -> Self {
        Self {
            id: chapter.id,
            title: chapter.title,
            description: chapter.description,
            sort_order: chapter.sort_order,
            book_id: chapter.book_id,
            media_count,
            created_at: chapter.created_at.naive_utc().and_utc(),
            updated_at: chapter.updated_at.naive_utc().and_utc(),
        }
    }
}

impl ChapterResponse {
    /// 从带媒体计数的章节创建响应
    pub fn from_with_media_count(chapter: Model, media_count: i64) -> Self {
        Self {
            id: chapter.id,
            title: chapter.title,
            description: chapter.description,
            sort_order: chapter.sort_order,
            book_id: chapter.book_id,
            media_count,
            created_at: chapter.created_at.naive_utc().and_utc(),
            updated_at: chapter.updated_at.naive_utc().and_utc(),
        }
    }
}
