use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        // 添加 is_staff 字段
        m.alter_table(
            Table::alter()
                .table(Users::Table)
                .add_column(
                    ColumnDef::new(Users::IsStaff)
                        .boolean()
                        .not_null()
                        .default(false),
                )
                .to_owned(),
        )
        .await?;

        // 添加 is_superuser 字段
        m.alter_table(
            Table::alter()
                .table(Users::Table)
                .add_column(
                    ColumnDef::new(Users::IsSuperuser)
                        .boolean()
                        .not_null()
                        .default(false),
                )
                .to_owned(),
        )
        .await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        // 删除 is_superuser 字段
        m.alter_table(
            Table::alter()
                .table(Users::Table)
                .drop_column(Users::IsSuperuser)
                .to_owned(),
        )
        .await?;

        // 删除 is_staff 字段
        m.alter_table(
            Table::alter()
                .table(Users::Table)
                .drop_column(Users::IsStaff)
                .to_owned(),
        )
        .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    IsStaff,
    IsSuperuser,
}
