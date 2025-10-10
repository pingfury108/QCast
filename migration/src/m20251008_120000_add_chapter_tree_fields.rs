use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        // 添加树状结构字段到 chapters 表
        m.alter_table(
            Table::alter()
                .table(Chapters::Table)
                .add_column(ColumnDef::new(Chapters::ParentId).integer().null())
                .to_owned(),
        )
        .await?;

        m.alter_table(
            Table::alter()
                .table(Chapters::Table)
                .add_column(ColumnDef::new(Chapters::Level).integer().null())
                .to_owned(),
        )
        .await?;

        m.alter_table(
            Table::alter()
                .table(Chapters::Table)
                .add_column(ColumnDef::new(Chapters::Path).text().null())
                .to_owned(),
        )
        .await?;

        // 添加索引以提升查询性能
        m.create_index(
            Index::create()
                .name("idx_chapters_book_id")
                .table(Chapters::Table)
                .col(Chapters::BookId)
                .to_owned(),
        )
        .await?;

        m.create_index(
            Index::create()
                .name("idx_chapters_parent_id")
                .table(Chapters::Table)
                .col(Chapters::ParentId)
                .to_owned(),
        )
        .await?;

        m.create_index(
            Index::create()
                .name("idx_chapters_path")
                .table(Chapters::Table)
                .col(Chapters::Path)
                .to_owned(),
        )
        .await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        // 删除索引
        m.drop_index(Index::drop().name("idx_chapters_path").to_owned())
            .await?;
        m.drop_index(Index::drop().name("idx_chapters_parent_id").to_owned())
            .await?;
        m.drop_index(Index::drop().name("idx_chapters_book_id").to_owned())
            .await?;

        // 删除添加的字段
        m.alter_table(
            Table::alter()
                .table(Chapters::Table)
                .drop_column(Chapters::Path)
                .to_owned(),
        )
        .await?;

        m.alter_table(
            Table::alter()
                .table(Chapters::Table)
                .drop_column(Chapters::Level)
                .to_owned(),
        )
        .await?;

        m.alter_table(
            Table::alter()
                .table(Chapters::Table)
                .drop_column(Chapters::ParentId)
                .to_owned(),
        )
        .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Chapters {
    Table,
    BookId,
    ParentId,
    Level,
    Path,
}
