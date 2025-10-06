use crate::models::_entities::books::Model;
use crate::models::books::BookTree;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BookResponse {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub cover_image: Option<String>,
    pub parent_id: Option<i32>,
    pub sort_order: Option<i32>,
    pub is_public: Option<bool>,
    pub user_id: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<Model> for BookResponse {
    fn from(book: Model) -> Self {
        Self {
            id: book.id,
            title: book.title,
            description: book.description,
            cover_image: book.cover_image,
            parent_id: book.parent_id,
            sort_order: book.sort_order,
            is_public: book.is_public,
            user_id: book.user_id_id,
            created_at: book.created_at.into(),
            updated_at: book.updated_at.into(),
        }
    }
}

/// 书籍树形结构响应
#[derive(Debug, Serialize, Deserialize)]
pub struct BookTreeResponse {
    pub book: BookResponse,
    pub children: Vec<BookTreeResponse>,
}

impl From<BookTree> for BookTreeResponse {
    fn from(tree: BookTree) -> Self {
        Self {
            book: BookResponse::from(tree.book),
            children: tree
                .children
                .into_iter()
                .map(BookTreeResponse::from)
                .collect(),
        }
    }
}
