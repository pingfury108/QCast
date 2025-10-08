pub use super::_entities::books;
pub use super::_entities::chapters::{ActiveModel, Column, Entity, Model};
use sea_orm::entity::prelude::*;
use sea_orm::IntoActiveModel;
use sea_orm::QueryOrder;
use sea_orm::QuerySelect;
use sea_orm::Set;
use sea_orm::TransactionTrait;
use sea_orm::DatabaseTransaction;
use serde::{Serialize, Deserialize};
pub type Chapters = Entity;

/// 章节树状结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterTree {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub sort_order: Option<i32>,
    pub book_id: i32,
    pub parent_id: Option<i32>,
    pub level: Option<i32>,
    pub path: Option<String>,
    pub media_count: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub children: Vec<ChapterTree>,
}

/// 章节移动参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterMoveParams {
    pub new_parent_id: Option<i32>,
    pub new_sort_order: Option<i32>,
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, insert: bool) -> std::result::Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let mut this = self;

        if insert {
            // 设置默认值 - 只对真正未设置的值设置默认值
            if this.sort_order.is_unchanged() {
                this.sort_order = sea_orm::ActiveValue::Set(Some(0));
            }
            // parent_id 不需要在这里设置默认值，因为数据库字段本身就是 NULL
            if this.level.is_unchanged() {
                this.level = sea_orm::ActiveValue::Set(None);
            }
            if this.path.is_unchanged() {
                this.path = sea_orm::ActiveValue::Set(None);
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

    /// 获取书籍的顶级章节（parent_id 为 null）
    pub async fn find_root_chapters(
        db: &DatabaseConnection,
        book_id: i32,
    ) -> Result<Vec<Model>, DbErr> {
        Entity::find()
            .filter(Column::BookId.eq(book_id))
            .filter(Column::ParentId.is_null())
            .order_by_asc(Column::SortOrder)
            .order_by_asc(Column::CreatedAt)
            .all(db)
            .await
    }

    /// 获取章节的直接子章节
    pub async fn find_children(
        db: &DatabaseConnection,
        parent_id: i32,
    ) -> Result<Vec<Model>, DbErr> {
        Entity::find()
            .filter(Column::ParentId.eq(parent_id))
            .order_by_asc(Column::SortOrder)
            .order_by_asc(Column::CreatedAt)
            .all(db)
            .await
    }

    /// 获取章节的完整树形结构（递归获取所有子章节）
    pub async fn get_tree(db: &DatabaseConnection, chapter_id: i32) -> Result<ChapterTree, DbErr> {
        let chapter = Entity::find_by_id(chapter_id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!("Chapter {} not found", chapter_id)))?;

        // 获取媒体计数
        let media_count = crate::models::_entities::medias::Entity::find()
            .filter(crate::models::_entities::medias::Column::ChapterId.eq(chapter_id))
            .count(db)
            .await? as i64;

        let children = Self::find_children(db, chapter_id).await?;
        let mut tree_children = Vec::new();

        // 递归获取子章节的树形结构
        for child in children {
            // 使用 Box::pin 来避免递归 async fn 问题
            let child_tree_future = Box::pin(Self::get_tree(db, child.id));
            let child_tree = child_tree_future.await?;
            tree_children.push(child_tree);
        }

        Ok(ChapterTree {
            id: chapter.id,
            title: chapter.title,
            description: chapter.description,
            sort_order: chapter.sort_order,
            book_id: chapter.book_id,
            parent_id: chapter.parent_id,
            level: chapter.level,
            path: chapter.path,
            media_count,
            created_at: chapter.created_at.naive_utc().and_utc(),
            updated_at: chapter.updated_at.naive_utc().and_utc(),
            children: tree_children,
        })
    }

    /// 获取书籍的所有章节的树形结构
    pub async fn get_book_tree(
        db: &DatabaseConnection,
        book_id: i32,
    ) -> Result<Vec<ChapterTree>, DbErr> {
        let root_chapters = Self::find_root_chapters(db, book_id).await?;
        let mut tree = Vec::new();

        for chapter in root_chapters {
            let chapter_tree = Self::get_tree(db, chapter.id).await?;
            tree.push(chapter_tree);
        }

        Ok(tree)
    }

    /// 获取书籍的扁平章节列表（包含层级信息）
    pub async fn get_flat_list_with_level(
        db: &DatabaseConnection,
        book_id: i32,
    ) -> Result<Vec<ChapterTree>, DbErr> {
        let chapters = Entity::find()
            .filter(Column::BookId.eq(book_id))
            .order_by_asc(Column::Path)
            .order_by_asc(Column::SortOrder)
            .order_by_asc(Column::CreatedAt)
            .all(db)
            .await?;

        let mut result = Vec::new();
        for chapter in chapters {
            let media_count = crate::models::_entities::medias::Entity::find()
                .filter(crate::models::_entities::medias::Column::ChapterId.eq(chapter.id))
                .count(db)
                .await? as i64;

            result.push(ChapterTree {
                id: chapter.id,
                title: chapter.title,
                description: chapter.description,
                sort_order: chapter.sort_order,
                book_id: chapter.book_id,
                parent_id: chapter.parent_id,
                level: chapter.level,
                path: chapter.path,
                media_count,
                created_at: chapter.created_at.naive_utc().and_utc(),
                updated_at: chapter.updated_at.naive_utc().and_utc(),
                children: Vec::new(), // 扁平列表不包含子节点
            });
        }

        Ok(result)
    }

    /// 计算并更新章节的层级和路径
    pub async fn update_level_and_path(
        db: &DatabaseConnection,
        chapter_id: i32,
        parent_id: Option<i32>,
    ) -> Result<(), DbErr> {
        let txn = db.begin().await?;

        Self::update_level_and_path_txn(&txn, chapter_id, parent_id).await?;

        txn.commit().await?;
        Ok(())
    }

    /// 在事务中计算并更新章节的层级和路径
    pub async fn update_level_and_path_txn(
        db: &DatabaseTransaction,
        chapter_id: i32,
        parent_id: Option<i32>,
    ) -> Result<(), DbErr> {
        if let Some(parent_id) = parent_id {
            // 获取父章节信息
            let parent = Entity::find_by_id(parent_id)
                .one(db)
                .await?
                .ok_or(DbErr::RecordNotFound(format!("Parent chapter {} not found", parent_id)))?;

            let level = parent.level.unwrap_or(0) + 1;
            let path = match parent.path {
                Some(parent_path) => format!("{}/{}", parent_path, chapter_id),
                None => format!("{}", chapter_id),
            };

            // 更新当前章节
            let chapter = Entity::find_by_id(chapter_id)
                .one(db)
                .await?
                .ok_or(DbErr::RecordNotFound(format!("Chapter {} not found", chapter_id)))?;

            let mut active_chapter = chapter.into_active_model();
            active_chapter.level = Set(Some(level));
            active_chapter.path = Set(Some(path.clone()));
            active_chapter.update(db).await?;

            // 递归更新所有子章节的层级和路径
            Self::update_children_level_and_path(db, chapter_id, level, &path).await?;
        } else {
            // 顶级章节
            let chapter = Entity::find_by_id(chapter_id)
                .one(db)
                .await?
                .ok_or(DbErr::RecordNotFound(format!("Chapter {} not found", chapter_id)))?;

            let mut active_chapter = chapter.into_active_model();
            active_chapter.level = Set(Some(0));
            active_chapter.path = Set(Some(chapter_id.to_string()));
            active_chapter.update(db).await?;

            // 递归更新所有子章节的层级和路径
            Self::update_children_level_and_path(db, chapter_id, 0, &chapter_id.to_string()).await?;
        }

        Ok(())
    }

    /// 递归更新子章节的层级和路径
    async fn update_children_level_and_path(
        db: &DatabaseTransaction,
        parent_id: i32,
        parent_level: i32,
        parent_path: &str,
    ) -> Result<(), DbErr> {
        let children = Entity::find()
            .filter(Column::ParentId.eq(parent_id))
            .all(db)
            .await?;

        for child in children {
            let child_id = child.id;
            let level = parent_level + 1;
            let path = format!("{}/{}", parent_path, child_id);

            let mut active_child = child.into_active_model();
            active_child.level = Set(Some(level));
            active_child.path = Set(Some(path.clone()));
            active_child.update(db).await?;

            // 递归更新子章节的子章节
            Box::pin(Self::update_children_level_and_path(db, child_id, level, &path)).await?;
        }

        Ok(())
    }

    /// 移动章节到新的父级
    pub async fn move_to_parent(
        db: &DatabaseConnection,
        chapter_id: i32,
        new_parent_id: Option<i32>,
        new_sort_order: Option<i32>,
    ) -> Result<(), DbErr> {
        let chapter = Entity::find_by_id(chapter_id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!("Chapter {} not found", chapter_id)))?;

        // 检查是否会形成循环引用
        if let Some(new_parent_id) = new_parent_id {
            if Self::would_create_cycle(db, chapter_id, new_parent_id).await? {
                return Err(DbErr::Custom("Moving chapter would create a cycle".to_string()));
            }
        }

        let txn = db.begin().await?;

        // 更新父级和排序
        let mut active_chapter = chapter.into_active_model();
        active_chapter.parent_id = Set(new_parent_id);
        if let Some(sort_order) = new_sort_order {
            active_chapter.sort_order = Set(Some(sort_order));
        }
        active_chapter.update(&txn).await?;

        // 更新层级和路径
        Self::update_level_and_path_txn(&txn, chapter_id, new_parent_id).await?;

        txn.commit().await?;
        Ok(())
    }

    /// 检查移动是否会形成循环引用
    async fn would_create_cycle(
        db: &DatabaseConnection,
        chapter_id: i32,
        new_parent_id: i32,
    ) -> Result<bool, DbErr> {
        let mut current_parent = new_parent_id;

        // 最多检查100层，防止无限循环
        for _ in 0..100 {
            if current_parent == chapter_id {
                return Ok(true);
            }

            let parent = Entity::find_by_id(current_parent).one(db).await?;
            if let Some(parent) = parent {
                if let Some(parent_id) = parent.parent_id {
                    current_parent = parent_id;
                } else {
                    break; // 到达顶级章节
                }
            } else {
                break; // 父章节不存在
            }
        }

        Ok(false)
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

    /// 创建子章节
    pub async fn create_child(
        db: &DatabaseConnection,
        book_id: i32,
        parent_id: i32,
        title: String,
        description: Option<String>,
    ) -> Result<Self, DbErr> {
        // 获取同级章节的最大排序号
        let next_order = Entity::find()
            .filter(Column::BookId.eq(book_id))
            .filter(Column::ParentId.eq(parent_id))
            .select_only()
            .column(Column::SortOrder)
            .order_by_desc(Column::SortOrder)
            .into_tuple::<Option<i32>>()
            .one(db)
            .await?
            .flatten()
            .unwrap_or(0) + 1;

        Ok(Self {
            book_id: Set(book_id),
            parent_id: Set(Some(parent_id)),
            title: Set(title),
            description: Set(description),
            sort_order: Set(Some(next_order)),
            ..Default::default()
        })
    }
}

// implement your custom finders, selectors oriented logic here
impl Entity {}
