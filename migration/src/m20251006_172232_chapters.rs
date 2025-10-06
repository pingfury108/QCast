use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "chapters",
            &[
                ("id", ColType::PkAuto),
                ("title", ColType::StringNull),
                ("description", ColType::TextNull),
                ("sort_order", ColType::IntegerNull),
            ],
            &[("book", "")],
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "chapters").await
    }
}
