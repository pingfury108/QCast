pub use super::_entities::books;
pub use super::_entities::chapters::{ActiveModel, Column, Entity, Model};
use sea_orm::entity::prelude::*;
use sea_orm::IntoActiveModel;
use sea_orm::QueryOrder;
use sea_orm::QuerySelect;
use sea_orm::Set;
use sea_orm::TransactionTrait;
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

    /// 获取带媒体计数的章节
    pub async fn find_by_book_with_media_count(db: &DatabaseConnection, book_id: i32) -> Result<Vec<(Model, i64)>, DbErr> {
        use crate::models::_entities::medias;
        use sea_orm::JoinType;

        let chapters = Entity::find()
            .filter(Column::BookId.eq(book_id))
            .order_by_asc(Column::SortOrder)
            .order_by_asc(Column::CreatedAt)
            .all(db)
            .await?;

        let mut result = Vec::new();
        for chapter in chapters {
            let media_count = medias::Entity::find()
                .filter(medias::Column::ChapterId.eq(chapter.id))
                .count(db)
                .await?;
            result.push((chapter, media_count as i64));
        }

        Ok(result)
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

        Ok(max_order.flatten().unwrap_or(0) + 1)
    }

    /// 重新排序书籍的所有章节
    pub async fn reorder_all(
        db: &DatabaseConnection,
        book_id: i32,
        chapter_ids: &[i32],
    ) -> Result<(), DbErr> {
        let txn = db.begin().await?;

        for (new_order, &chapter_id) in chapter_ids.iter().enumerate() {
            if let Some(chapter) = Entity::find_by_id(chapter_id).one(&txn).await? {
                if chapter.book_id != book_id {
                    txn.rollback().await?;
                    return Err(DbErr::Custom(
                        "Chapter does not belong to this book".to_string(),
                    ));
                }

                let mut active_chapter = chapter.into_active_model();
                active_chapter.sort_order = Set(Some((new_order + 1) as i32));
                active_chapter.update(&txn).await?;
            } else {
                txn.rollback().await?;
                return Err(DbErr::Custom("Chapter not found".to_string()));
            }
        }

        txn.commit().await?;
        Ok(())
    }

    /// 向上移动章节（减小 sort_order）
    pub async fn move_up(db: &DatabaseConnection, chapter_id: i32) -> Result<bool, DbErr> {
        let chapter = Entity::find_by_id(chapter_id).one(db).await?;
        if let Some(chapter) = chapter {
            let current_order = chapter.sort_order.unwrap_or(0);

            // 找到当前章节前一个章节
            let prev_chapter = Entity::find()
                .filter(Column::BookId.eq(chapter.book_id))
                .filter(Column::SortOrder.lt(current_order))
                .order_by_desc(Column::SortOrder)
                .one(db)
                .await?;

            if let Some(prev_chapter) = prev_chapter {
                let txn = db.begin().await?;

                // 保存原来的排序号
                let prev_order = prev_chapter.sort_order;

                // 交换排序号
                let mut active_current = chapter.into_active_model();
                let mut active_prev = prev_chapter.into_active_model();

                active_current.sort_order = Set(prev_order);
                active_prev.sort_order = Set(Some(current_order));

                active_current.update(&txn).await?;
                active_prev.update(&txn).await?;

                txn.commit().await?;
                Ok(true)
            } else {
                Ok(false) // 已经是第一个章节
            }
        } else {
            Err(DbErr::Custom("Chapter not found".to_string()))
        }
    }

    /// 向下移动章节（增加 sort_order）
    pub async fn move_down(db: &DatabaseConnection, chapter_id: i32) -> Result<bool, DbErr> {
        let chapter = Entity::find_by_id(chapter_id).one(db).await?;
        if let Some(chapter) = chapter {
            let current_order = chapter.sort_order.unwrap_or(0);

            // 找到当前章节后一个章节
            let next_chapter = Entity::find()
                .filter(Column::BookId.eq(chapter.book_id))
                .filter(Column::SortOrder.gt(current_order))
                .order_by_asc(Column::SortOrder)
                .one(db)
                .await?;

            if let Some(next_chapter) = next_chapter {
                let txn = db.begin().await?;

                // 保存原来的排序号
                let next_order = next_chapter.sort_order;

                // 交换排序号
                let mut active_current = chapter.into_active_model();
                let mut active_next = next_chapter.into_active_model();

                active_current.sort_order = Set(next_order);
                active_next.sort_order = Set(Some(current_order));

                active_current.update(&txn).await?;
                active_next.update(&txn).await?;

                txn.commit().await?;
                Ok(true)
            } else {
                Ok(false) // 已经是最后一个章节
            }
        } else {
            Err(DbErr::Custom("Chapter not found".to_string()))
        }
    }
}

// implement your write-oriented logic here
impl ActiveModel {
    /// 创建新章节时自动设置排序号
    pub async fn create_with_order(
        db: &DatabaseConnection,
        book_id: i32,
        title: String,
        description: Option<String>,
    ) -> Result<Self, DbErr> {
        let next_order = Model::get_next_sort_order(db, book_id).await?;

        Ok(Self {
            book_id: Set(book_id),
            title: Set(title),
            description: Set(description),
            sort_order: Set(Some(next_order)),
            ..Default::default()
        })
    }
}

// implement your custom finders, selectors oriented logic here
impl Entity {}
