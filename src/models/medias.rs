pub use super::_entities::books;
pub use super::_entities::medias::{ActiveModel, Column, Entity, Model};
pub use super::_entities::users;
use sea_orm::entity::prelude::*;
use sea_orm::QueryOrder;
use sea_orm::Set;
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
            // 生成访问令牌
            if this.access_token.is_unchanged() {
                this.access_token = sea_orm::ActiveValue::Set(Uuid::new_v4().to_string());
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
impl ActiveModel {
    /// 创建新的媒体记录
    pub fn create_new(
        title: String,
        description: Option<String>,
        file_type: String,
        file_path: String,
        file_size: Option<i64>,
        duration: Option<i32>,
        mime_type: Option<String>,
        original_filename: Option<String>,
        book_id: i32,
        chapter_id: Option<i32>,
        user_id: i32,
    ) -> Self {
        Self {
            title: Set(title),
            description: Set(description),
            file_type: Set(file_type),
            file_path: Set(file_path),
            file_size: Set(file_size),
            duration: Set(duration),
            mime_type: Set(mime_type),
            access_token: Set(Uuid::new_v4().to_string()),
            access_url: Set(None),   // 将在 service 中生成
            qr_code_path: Set(None), // 将在 worker 中生成
            file_version: Set(1),
            original_filename: Set(original_filename),
            play_count: Set(0),
            is_public: Set(false),
            chapter_id: Set(chapter_id),
            book_id: Set(book_id),
            user_id: Set(user_id),
            ..Default::default()
        }
    }

    /// 替换媒体文件（保持 access_token 不变）
    pub async fn replace_file(
        db: &DatabaseConnection,
        media_id: i32,
        user_id: i32,
        new_file_path: String,
        new_file_size: Option<i64>,
        new_duration: Option<i32>,
        new_mime_type: Option<String>,
        new_original_filename: Option<String>,
    ) -> Result<Model, DbErr> {
        // 查找现有媒体记录
        let media = Entity::find()
            .filter(Column::Id.eq(media_id))
            .filter(Column::UserId.eq(user_id))
            .one(db)
            .await?
            .ok_or(DbErr::Custom(
                "Media not found or permission denied".to_string(),
            ))?;

        let old_file_version = media.file_version;
        let mut active_model: ActiveModel = media.into();
        active_model.file_path = Set(new_file_path);
        active_model.file_size = Set(new_file_size);
        active_model.duration = Set(new_duration);
        active_model.mime_type = Set(new_mime_type);
        active_model.file_version = Set(old_file_version + 1);
        active_model.original_filename = Set(new_original_filename);
        // access_token 保持不变

        let updated_media = active_model.update(db).await?;
        Ok(updated_media)
    }

    /// 发布/取消发布媒体
    pub async fn toggle_public(
        db: &DatabaseConnection,
        media_id: i32,
        user_id: i32,
    ) -> Result<Model, DbErr> {
        let media = Entity::find()
            .filter(Column::Id.eq(media_id))
            .filter(Column::UserId.eq(user_id))
            .one(db)
            .await?
            .ok_or(DbErr::Custom(
                "Media not found or permission denied".to_string(),
            ))?;

        let old_is_public = media.is_public;
        let mut active_model: ActiveModel = media.into();
        active_model.is_public = Set(!old_is_public);

        let updated_media = active_model.update(db).await?;
        Ok(updated_media)
    }
}

// implement your custom finders, selectors oriented logic here
impl Entity {}
