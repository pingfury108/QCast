use serde::{Deserialize, Serialize};
use crate::models::_entities::chapters::Model;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChapterResponse {
    pub id: i32,
    pub title: Option<String>,
    pub description: Option<String>,
    pub sort_order: Option<i32>,
    pub book_id: i32,
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
            created_at: chapter.created_at.into(),
            updated_at: chapter.updated_at.into(),
        }
    }
}