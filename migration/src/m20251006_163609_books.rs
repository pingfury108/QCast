use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "books",
            &[
                ("id", ColType::PkAuto),
                ("title", ColType::String),
                ("description", ColType::TextNull),
                ("cover_image", ColType::StringNull),
                ("parent_id", ColType::IntegerNull),
                ("sort_order", ColType::IntegerNull),
                ("is_public", ColType::BooleanNull),
            ],
            &[("user_id", "users")],
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "books").await
    }
}
