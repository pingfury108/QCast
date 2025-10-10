pub use super::_entities::user_group_members;
pub use super::_entities::user_groups::{ActiveModel, Entity, Model};
use loco_rs::prelude::*;
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveValue, QueryOrder};
pub type UserGroups = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, insert: bool) -> std::result::Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if !insert && self.updated_at.is_unchanged() {
            let mut this = self;
            this.updated_at = sea_orm::ActiveValue::Set(chrono::Utc::now().into());
            Ok(this)
        } else {
            Ok(self)
        }
    }
}

// implement your read-oriented logic here
impl Model {
    /// 创建用户组
    ///
    /// # Errors
    ///
    /// When database insert fails
    pub async fn create_group(
        db: &DatabaseConnection,
        name: String,
        description: Option<String>,
    ) -> ModelResult<Self> {
        let group = ActiveModel {
            name: ActiveValue::Set(name),
            description: ActiveValue::Set(description),
            ..Default::default()
        };

        Ok(group.insert(db).await?)
    }

    /// 列出所有用户组
    ///
    /// # Errors
    ///
    /// When database query fails
    pub async fn list_all(db: &DatabaseConnection) -> ModelResult<Vec<Self>> {
        Ok(Entity::find()
            .order_by_asc(super::_entities::user_groups::Column::Name)
            .all(db)
            .await?)
    }

    /// 根据 ID 查找用户组
    ///
    /// # Errors
    ///
    /// When group not found or database query fails
    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> ModelResult<Self> {
        Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)
    }

    /// 添加用户到组
    ///
    /// # Errors
    ///
    /// When database insert fails or unique constraint violation
    pub async fn add_user(
        db: &DatabaseConnection,
        group_id: i32,
        user_id: i32,
    ) -> ModelResult<user_group_members::Model> {
        let now = chrono::Local::now();
        let member = user_group_members::ActiveModel {
            group_id: ActiveValue::Set(group_id),
            user_id: ActiveValue::Set(user_id),
            created_at: ActiveValue::Set(now.into()),
            updated_at: ActiveValue::Set(now.into()),
            ..Default::default()
        };

        Ok(member.insert(db).await?)
    }

    /// 从组中移除用户
    ///
    /// # Errors
    ///
    /// When database delete fails
    pub async fn remove_user(
        db: &DatabaseConnection,
        group_id: i32,
        user_id: i32,
    ) -> ModelResult<()> {
        user_group_members::Entity::delete_many()
            .filter(user_group_members::Column::GroupId.eq(group_id))
            .filter(user_group_members::Column::UserId.eq(user_id))
            .exec(db)
            .await?;

        Ok(())
    }

    /// 获取组内所有用户
    ///
    /// # Errors
    ///
    /// When database query fails
    pub async fn get_members(
        db: &DatabaseConnection,
        group_id: i32,
    ) -> ModelResult<Vec<super::users::Model>> {
        use super::_entities::{user_group_members, users};

        let members = user_group_members::Entity::find()
            .filter(user_group_members::Column::GroupId.eq(group_id))
            .find_also_related(users::Entity)
            .all(db)
            .await?;

        Ok(members.into_iter().filter_map(|(_, user)| user).collect())
    }

    /// 获取用户所属的所有组
    ///
    /// # Errors
    ///
    /// When database query fails
    pub async fn get_user_groups(db: &DatabaseConnection, user_id: i32) -> ModelResult<Vec<Self>> {
        use super::_entities::user_group_members;

        let groups = user_group_members::Entity::find()
            .filter(user_group_members::Column::UserId.eq(user_id))
            .find_also_related(Entity)
            .all(db)
            .await?;

        Ok(groups.into_iter().filter_map(|(_, group)| group).collect())
    }

    /// 检查用户是否在组内
    ///
    /// # Errors
    ///
    /// When database query fails
    pub async fn is_user_in_group(
        db: &DatabaseConnection,
        group_id: i32,
        user_id: i32,
    ) -> ModelResult<bool> {
        let count = user_group_members::Entity::find()
            .filter(user_group_members::Column::GroupId.eq(group_id))
            .filter(user_group_members::Column::UserId.eq(user_id))
            .count(db)
            .await?;

        Ok(count > 0)
    }

    /// 统计组内用户数量
    ///
    /// # Errors
    ///
    /// When database query fails
    pub async fn count_members(db: &DatabaseConnection, group_id: i32) -> ModelResult<u64> {
        Ok(user_group_members::Entity::find()
            .filter(user_group_members::Column::GroupId.eq(group_id))
            .count(db)
            .await?)
    }
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}
