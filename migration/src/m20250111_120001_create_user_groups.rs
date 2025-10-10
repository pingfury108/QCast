use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        // 创建 user_groups 表
        create_table(
            m,
            "user_groups",
            &[
                ("id", ColType::PkAuto),
                ("name", ColType::String),
                ("description", ColType::TextNull),
            ],
            &[],
        )
        .await?;

        // 创建 user_group_members 表，直接在表定义中包含外键
        m.create_table(
            Table::create()
                .table(UserGroupMembers::Table)
                .col(pk_auto(UserGroupMembers::Id))
                .col(integer(UserGroupMembers::UserId))
                .col(integer(UserGroupMembers::GroupId))
                .col(timestamp_with_time_zone(UserGroupMembers::CreatedAt))
                .col(timestamp_with_time_zone(UserGroupMembers::UpdatedAt))
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_user_group_members_user_id")
                        .from(UserGroupMembers::Table, UserGroupMembers::UserId)
                        .to(Users::Table, Users::Id)
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_user_group_members_group_id")
                        .from(UserGroupMembers::Table, UserGroupMembers::GroupId)
                        .to(UserGroups::Table, UserGroups::Id)
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .to_owned(),
        )
        .await?;

        // 添加唯一索引，防止重复添加用户到同一组
        m.create_index(
            Index::create()
                .name("idx_user_group_members_unique")
                .table(UserGroupMembers::Table)
                .col(UserGroupMembers::UserId)
                .col(UserGroupMembers::GroupId)
                .unique()
                .to_owned(),
        )
        .await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        // 删除表（外键会自动删除）
        drop_table(m, "user_group_members").await?;
        drop_table(m, "user_groups").await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum UserGroups {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum UserGroupMembers {
    Table,
    Id,
    UserId,
    GroupId,
    CreatedAt,
    UpdatedAt,
}
