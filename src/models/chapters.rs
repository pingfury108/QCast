pub use super::_entities::books;
pub use super::_entities::chapters::{ActiveModel, Column, Entity, Model};
use sea_orm::entity::prelude::*;
use sea_orm::QueryOrder;
use sea_orm::QuerySelect;
pub type Chapters = Entity;

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
        }

        if !insert && this.updated_at.is_unchanged() {
            this.updated_at = sea_orm::ActiveValue::Set(chrono::Utc::now().into());
        }

        Ok(this)
    }
}

// implement your read-oriented logic here
impl Model {
    /// 获取书籍的所有章节
    pub async fn find_by_book(db: &DatabaseConnection, book_id: i32) -> Result<Vec<Model>, DbErr> {
        Entity::find()
            .filter(Column::BookId.eq(book_id))
            .order_by_asc(Column::SortOrder)
            .order_by_asc(Column::CreatedAt)
            .all(db)
            .await
    }

    /// 搜索章节
    pub async fn search(
        db: &DatabaseConnection,
        book_id: i32,
        query: &str,
    ) -> Result<Vec<Model>, DbErr> {
        Entity::find()
            .filter(Column::BookId.eq(book_id))
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

    /// 获取下一个排序号
    pub async fn get_next_sort_order(db: &DatabaseConnection, book_id: i32) -> Result<i32, DbErr> {
        let max_order = Entity::find()
            .filter(Column::BookId.eq(book_id))
            .select_only()
            .column(Column::SortOrder)
            .order_by_desc(Column::SortOrder)
            .into_tuple::<Option<i32>>()
            .one(db)
            .await?;

        Ok(max_order.flatten().unwrap_or(-1) + 1)
    }
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}
