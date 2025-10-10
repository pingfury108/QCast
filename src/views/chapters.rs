use crate::models::_entities::chapters::Model;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChapterResponse {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub sort_order: Option<i32>,
    pub book_id: i32,
    pub parent_id: Option<i32>,
    pub level: Option<i32>,
    pub path: Option<String>,
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
            parent_id: chapter.parent_id,
            level: chapter.level,
            path: chapter.path,
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
            parent_id: chapter.parent_id,
            level: chapter.level,
            path: chapter.path,
            media_count,
            created_at: chapter.created_at.naive_utc().and_utc(),
            updated_at: chapter.updated_at.naive_utc().and_utc(),
        }
    }
}

impl ChapterResponse {
    /// 从带媒体计数的章节创建响应
    #[must_use]
    pub fn from_with_media_count(chapter: Model, media_count: i64) -> Self {
        Self {
            id: chapter.id,
            title: chapter.title,
            description: chapter.description,
            sort_order: chapter.sort_order,
            book_id: chapter.book_id,
            parent_id: chapter.parent_id,
            level: chapter.level,
            path: chapter.path,
            media_count,
            created_at: chapter.created_at.naive_utc().and_utc(),
            updated_at: chapter.updated_at.naive_utc().and_utc(),
        }
    }

    /// 从章节树结构创建响应
    #[must_use]
    pub fn from_chapter_tree(chapter_tree: crate::models::chapters::ChapterTree) -> Self {
        Self {
            id: chapter_tree.id,
            title: chapter_tree.title,
            description: chapter_tree.description,
            sort_order: chapter_tree.sort_order,
            book_id: chapter_tree.book_id,
            parent_id: chapter_tree.parent_id,
            level: chapter_tree.level,
            path: chapter_tree.path,
            media_count: chapter_tree.media_count,
            created_at: chapter_tree.created_at,
            updated_at: chapter_tree.updated_at,
        }
    }
}
