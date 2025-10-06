pub use super::_entities::books::{ActiveModel, Column, Entity, Model};
pub use super::_entities::users;
use sea_orm::entity::prelude::*;
use sea_orm::QueryOrder;
use serde::{Deserialize, Serialize};
pub type Books = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, insert: bool) -> std::result::Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let mut this = self;

        if insert {
            // 设置默认值
            if this.sort_order.is_unchanged() {
                this.sort_order = sea_orm::ActiveValue::Set(Some(0));
            }
            if this.is_public.is_unchanged() {
                this.is_public = sea_orm::ActiveValue::Set(Some(false));
            }
        }

        if !insert && this.updated_at.is_unchanged() {
            this.updated_at = sea_orm::ActiveValue::Set(chrono::Utc::now().into());
        }

        Ok(this)
    }
}

// implement your read-oriented logic here
impl Model {
    /// 获取用户的所有书籍
    pub async fn find_by_user(db: &DatabaseConnection, user_id: i32) -> Result<Vec<Model>, DbErr> {
        Entity::find()
            .filter(Column::UserIdId.eq(user_id))
            .all(db)
            .await
    }

    /// 获取顶级书籍（parent_id 为 null）
    pub async fn find_root_books(
        db: &DatabaseConnection,
        user_id: i32,
    ) -> Result<Vec<Model>, DbErr> {
        Entity::find()
            .filter(Column::UserIdId.eq(user_id))
            .filter(Column::ParentId.is_null())
            .order_by_asc(Column::SortOrder)
            .order_by_asc(Column::CreatedAt)
            .all(db)
            .await
    }

    /// 获取子书籍
    pub async fn find_children(
        db: &DatabaseConnection,
        parent_id: i32,
    ) -> Result<Vec<Model>, DbErr> {
        Entity::find()
            .filter(Column::ParentId.eq(parent_id))
            .order_by_asc(Column::SortOrder)
            .order_by_asc(Column::CreatedAt)
            .all(db)
            .await
    }

    /// 搜索书籍
    pub async fn search(
        db: &DatabaseConnection,
        user_id: i32,
        query: &str,
    ) -> Result<Vec<Model>, DbErr> {
        Entity::find()
            .filter(Column::UserIdId.eq(user_id))
            .filter(
                Column::Title
                    .contains(query)
                    .or(Column::Description.contains(query)),
            )
            .order_by_asc(Column::SortOrder)
            .order_by_asc(Column::CreatedAt)
            .all(db)
            .await
    }

    /// 获取书籍的完整树形结构（递归获取所有子书籍）
    pub async fn get_tree(db: &DatabaseConnection, book_id: i32) -> Result<BookTree, DbErr> {
        let book = Entity::find_by_id(book_id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!("Book {} not found", book_id)))?;

        let children = Self::find_children(db, book_id).await?;
        let mut tree_children = Vec::new();

        // 递归获取子书籍的树形结构
        for child in children {
            // 使用 Box::pin 来避免递归 async fn 问题
            let child_tree_future = Box::pin(Self::get_tree(db, child.id));
            let child_tree = child_tree_future.await?;
            tree_children.push(child_tree);
        }

        Ok(BookTree {
            book,
            children: tree_children,
        })
    }

    /// 获取用户的所有书籍的树形结构
    pub async fn get_user_tree(
        db: &DatabaseConnection,
        user_id: i32,
    ) -> Result<Vec<BookTree>, DbErr> {
        let root_books = Self::find_root_books(db, user_id).await?;
        let mut tree = Vec::new();

        for book in root_books {
            let book_tree = Self::get_tree(db, book.id).await?;
            tree.push(book_tree);
        }

        Ok(tree)
    }
}

/// 书籍树形结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookTree {
    pub book: Model,
    pub children: Vec<BookTree>,
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}
