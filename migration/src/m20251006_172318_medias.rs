use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "medias",
            &[
                ("id", ColType::PkAuto),
                ("title", ColType::String),
                ("description", ColType::TextNull),
                ("file_type", ColType::String),
                ("file_path", ColType::String),
                ("file_size", ColType::BigIntegerNull),
                ("duration", ColType::IntegerNull),
                ("mime_type", ColType::StringNull),
                ("access_token", ColType::String),
                ("access_url", ColType::StringNull),
                ("qr_code_path", ColType::StringNull),
                ("file_version", ColType::Integer),
                ("original_filename", ColType::StringNull),
                ("play_count", ColType::Integer),
                ("is_public", ColType::Boolean),
                ("chapter_id", ColType::IntegerNull),
                ("book_id", ColType::Integer),
                ("user_id", ColType::Integer),
            ],
            &[],
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "medias").await
    }
}
