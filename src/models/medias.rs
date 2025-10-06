pub use super::_entities::books;
pub use super::_entities::medias::{ActiveModel, Column, Entity, Model};
pub use super::_entities::users;
use sea_orm::entity::prelude::*;
use sea_orm::QueryOrder;
use uuid::Uuid;
pub type Medias = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, insert: bool) -> std::result::Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let mut this = self;

        if insert {
            // 设置默认值
            if this.file_version.is_unchanged() {
                this.file_version = sea_orm::ActiveValue::Set(Some(1));
            }
            if this.play_count.is_unchanged() {
                this.play_count = sea_orm::ActiveValue::Set(Some(0));
            }
            if this.is_public.is_unchanged() {
                this.is_public = sea_orm::ActiveValue::Set(Some(false));
            }
            // 生成访问令牌
            if this.access_token.is_unchanged() {
                this.access_token = sea_orm::ActiveValue::Set(Some(Uuid::new_v4().to_string()));
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
    /// 获取用户的所有媒体文件
    pub async fn find_by_user(db: &DatabaseConnection, user_id: i32) -> Result<Vec<Model>, DbErr> {
        Entity::find()
            .filter(Column::UserId.eq(user_id))
            .order_by_desc(Column::CreatedAt)
            .all(db)
            .await
    }

    /// 获取书籍的所有媒体文件
    pub async fn find_by_book(db: &DatabaseConnection, book_id: i32) -> Result<Vec<Model>, DbErr> {
        Entity::find()
            .filter(Column::BookId.eq(book_id))
            .order_by_asc(Column::CreatedAt)
            .all(db)
            .await
    }

    /// 获取章节的所有媒体文件
    pub async fn find_by_chapter(
        db: &DatabaseConnection,
        chapter_id: i32,
    ) -> Result<Vec<Model>, DbErr> {
        Entity::find()
            .filter(Column::ChapterId.eq(chapter_id))
            .order_by_asc(Column::CreatedAt)
            .all(db)
            .await
    }

    /// 搜索媒体文件
    pub async fn search(
        db: &DatabaseConnection,
        user_id: i32,
        query: &str,
    ) -> Result<Vec<Model>, DbErr> {
        Entity::find()
            .filter(Column::UserId.eq(user_id))
            .filter(
                Column::Title
                    .contains(query)
                    .or(Column::Description.contains(query))
                    .or(Column::OriginalFilename.contains(query)),
            )
            .order_by_desc(Column::CreatedAt)
            .all(db)
            .await
    }

    /// 根据访问令牌查找媒体文件
    pub async fn find_by_access_token(
        db: &DatabaseConnection,
        access_token: &str,
    ) -> Result<Option<Model>, DbErr> {
        Entity::find()
            .filter(Column::AccessToken.eq(access_token))
            .one(db)
            .await
    }

    /// 获取公开的媒体文件
    pub async fn find_public(db: &DatabaseConnection) -> Result<Vec<Model>, DbErr> {
        Entity::find()
            .filter(Column::IsPublic.eq(true))
            .order_by_desc(Column::CreatedAt)
            .all(db)
            .await
    }

    /// 增加播放次数
    pub async fn increment_play_count(db: &DatabaseConnection, media_id: i32) -> Result<(), DbErr> {
        Entity::update_many()
            .filter(Column::Id.eq(media_id))
            .col_expr(Column::PlayCount, Expr::col(Column::PlayCount).add(1))
            .exec(db)
            .await?;
        Ok(())
    }
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}
