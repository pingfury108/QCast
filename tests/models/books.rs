use insta::{assert_debug_snapshot, with_settings};
use loco_rs::testing::prelude::*;
use qcast::app::App;
use qcast::models::_entities::books::{ActiveModel, Entity};
use qcast::models::books;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
};
use serial_test::serial;

macro_rules! configure_insta {
    ($($expr:expr),*) => {
        let mut settings = insta::Settings::clone_current();
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
    };
}

#[tokio::test]
#[serial]
async fn can_create_book() {
    configure_insta!();

    let boot = boot_test::<App>().await.unwrap();
    seed::<App>(&boot.app_context).await.unwrap();

    let book_data = ActiveModel {
        title: Set("测试书籍".to_string()),
        description: Set(Some("这是一本测试书籍".to_string())),
        user_id: Set(1), // 假设用户ID为1
        ..Default::default()
    };

    let created_book = book_data.insert(&boot.app_context.db).await.unwrap();

    assert!(created_book.id > 0);
    assert_eq!(created_book.title, "测试书籍");
    assert_eq!(
        created_book.description,
        Some("这是一本测试书籍".to_string())
    );
    assert_eq!(created_book.user_id, 1);

    with_settings!({
        filters => cleanup_book_model()
    }, {
        assert_debug_snapshot!(created_book);
    });
}

#[tokio::test]
#[serial]
async fn can_find_books_by_user() {
    configure_insta!();

    let boot = boot_test::<App>().await.unwrap();
    seed::<App>(&boot.app_context).await.unwrap();

    // 创建两本书，一本属于用户1，一本属于用户2
    let _book1 = ActiveModel {
        title: Set("用户1的书".to_string()),
        user_id: Set(1),
        ..Default::default()
    }
    .insert(&boot.app_context.db)
    .await
    .unwrap();

    let _book2 = ActiveModel {
        title: Set("用户2的书".to_string()),
        user_id: Set(2),
        ..Default::default()
    }
    .insert(&boot.app_context.db)
    .await
    .unwrap();

    // 查询用户1的书籍
    let user1_books = Entity::find()
        .filter(books::Column::UserId.eq(1))
        .all(&boot.app_context.db)
        .await
        .unwrap();

    assert_eq!(user1_books.len(), 1);
    assert_eq!(user1_books[0].title, "用户1的书");

    with_settings!({
        filters => cleanup_book_model()
    }, {
        assert_debug_snapshot!(user1_books);
    });
}

#[tokio::test]
#[serial]
async fn can_get_book_tree() {
    configure_insta!();

    let boot = boot_test::<App>().await.unwrap();
    seed::<App>(&boot.app_context).await.unwrap();

    // 创建根书籍
    let root_book = ActiveModel {
        title: Set("根书籍".to_string()),
        user_id: Set(1),
        sort_order: Set(Some(1)),
        ..Default::default()
    }
    .insert(&boot.app_context.db)
    .await
    .unwrap();

    // 创建子书籍
    let child_book = ActiveModel {
        title: Set("子书籍".to_string()),
        user_id: Set(1),
        parent_id: Set(Some(root_book.id)),
        sort_order: Set(Some(1)),
        ..Default::default()
    }
    .insert(&boot.app_context.db)
    .await
    .unwrap();

    // 创建孙子书籍
    let grandchild_book = ActiveModel {
        title: Set("孙子书籍".to_string()),
        user_id: Set(1),
        parent_id: Set(Some(child_book.id)),
        sort_order: Set(Some(1)),
        ..Default::default()
    }
    .insert(&boot.app_context.db)
    .await
    .unwrap();

    // 查询所有书籍并验证树结构
    let all_books = Entity::find()
        .filter(books::Column::UserId.eq(1))
        .order_by_asc(books::Column::SortOrder)
        .all(&boot.app_context.db)
        .await
        .unwrap();

    assert_eq!(all_books.len(), 3);

    // 验证父子关系
    let child = all_books.iter().find(|b| b.id == child_book.id).unwrap();
    assert_eq!(child.parent_id, Some(root_book.id));

    let grandchild = all_books
        .iter()
        .find(|b| b.id == grandchild_book.id)
        .unwrap();
    assert_eq!(grandchild.parent_id, Some(child_book.id));

    with_settings!({
        filters => cleanup_book_model()
    }, {
        assert_debug_snapshot!(all_books);
    });
}

#[tokio::test]
#[serial]
async fn can_search_books() {
    configure_insta!();

    let boot = boot_test::<App>().await.unwrap();
    seed::<App>(&boot.app_context).await.unwrap();

    // 创建多本书籍
    ActiveModel {
        title: Set("编程指南".to_string()),
        description: Set(Some("学习编程的完整指南".to_string())),
        user_id: Set(1),
        ..Default::default()
    }
    .insert(&boot.app_context.db)
    .await
    .unwrap();

    ActiveModel {
        title: Set("设计模式".to_string()),
        description: Set(Some("软件设计模式详解".to_string())),
        user_id: Set(1),
        ..Default::default()
    }
    .insert(&boot.app_context.db)
    .await
    .unwrap();

    ActiveModel {
        title: Set("算法导论".to_string()),
        description: Set(Some("算法与数据结构".to_string())),
        user_id: Set(1),
        ..Default::default()
    }
    .insert(&boot.app_context.db)
    .await
    .unwrap();

    // 搜索包含"设计"的书籍
    let search_results = Entity::find()
        .filter(
            books::Column::Title
                .contains("设计")
                .or(books::Column::Description.contains("设计")),
        )
        .all(&boot.app_context.db)
        .await
        .unwrap();

    assert_eq!(search_results.len(), 1);
    assert_eq!(search_results[0].title, "设计模式");

    with_settings!({
        filters => cleanup_book_model()
    }, {
        assert_debug_snapshot!(search_results);
    });
}

#[tokio::test]
#[serial]
async fn can_search_books_with_parents() {
    configure_insta!();

    let boot = boot_test::<App>().await.unwrap();
    seed::<App>(&boot.app_context).await.unwrap();

    // 创建书籍层级结构
    let root_book = ActiveModel {
        title: Set("根书籍".to_string()),
        user_id: Set(1),
        sort_order: Set(Some(1)),
        ..Default::default()
    }
    .insert(&boot.app_context.db)
    .await
    .unwrap();

    let child_book = ActiveModel {
        title: Set("子书籍".to_string()),
        user_id: Set(1),
        parent_id: Set(Some(root_book.id)),
        sort_order: Set(Some(1)),
        ..Default::default()
    }
    .insert(&boot.app_context.db)
    .await
    .unwrap();

    let grandchild_book = ActiveModel {
        title: Set("孙子书籍目标".to_string()),
        description: Set(Some("这是目标书籍".to_string())),
        user_id: Set(1),
        parent_id: Set(Some(child_book.id)),
        sort_order: Set(Some(1)),
        ..Default::default()
    }
    .insert(&boot.app_context.db)
    .await
    .unwrap();

    // 创建一个无关的书籍
    let _other_book = ActiveModel {
        title: Set("无关书籍".to_string()),
        user_id: Set(1),
        ..Default::default()
    }
    .insert(&boot.app_context.db)
    .await
    .unwrap();

    // 使用 search_with_parents 搜索包含"目标"的书籍
    let search_results = books::Model::search_with_parents(&boot.app_context.db, 1, "目标")
        .await
        .unwrap();

    // 应该返回孙子书籍、子书籍和根书籍（3个）
    assert_eq!(search_results.len(), 3);

    // 验证包含所有祖先
    let ids: Vec<i32> = search_results.iter().map(|b| b.id).collect();
    assert!(ids.contains(&root_book.id));
    assert!(ids.contains(&child_book.id));
    assert!(ids.contains(&grandchild_book.id));

    // 验证不包含无关书籍
    assert!(!ids.contains(&_other_book.id));

    with_settings!({
        filters => cleanup_book_model()
    }, {
        assert_debug_snapshot!(search_results);
    });
}

#[tokio::test]
#[serial]
async fn can_update_book() {
    configure_insta!();

    let boot = boot_test::<App>().await.unwrap();
    seed::<App>(&boot.app_context).await.unwrap();

    // 创建书籍
    let book = ActiveModel {
        title: Set("原始标题".to_string()),
        description: Set(Some("原始描述".to_string())),
        user_id: Set(1),
        ..Default::default()
    }
    .insert(&boot.app_context.db)
    .await
    .unwrap();

    // 更新书籍
    let mut book_to_update: ActiveModel = book.into();
    book_to_update.title = Set("更新后的标题".to_string());
    book_to_update.description = Set(Some("更新后的描述".to_string()));

    let updated_book = book_to_update.update(&boot.app_context.db).await.unwrap();

    assert_eq!(updated_book.title, "更新后的标题");
    assert_eq!(updated_book.description, Some("更新后的描述".to_string()));
    assert!(updated_book.updated_at > updated_book.created_at);

    with_settings!({
        filters => cleanup_book_model()
    }, {
        assert_debug_snapshot!(updated_book);
    });
}

#[tokio::test]
#[serial]
async fn can_delete_book() {
    configure_insta!();

    let boot = boot_test::<App>().await.unwrap();
    seed::<App>(&boot.app_context).await.unwrap();

    // 创建书籍
    let book = ActiveModel {
        title: Set("待删除的书籍".to_string()),
        user_id: Set(1),
        ..Default::default()
    }
    .insert(&boot.app_context.db)
    .await
    .unwrap();

    let book_id = book.id;

    // 删除书籍
    Entity::delete_by_id(book_id)
        .exec(&boot.app_context.db)
        .await
        .unwrap();

    // 验证书籍已被删除
    let deleted_book = Entity::find_by_id(book_id)
        .one(&boot.app_context.db)
        .await
        .unwrap();
    assert!(deleted_book.is_none());
}

#[tokio::test]
#[serial]
async fn can_handle_book_hierarchy() {
    configure_insta!();

    let boot = boot_test::<App>().await.unwrap();
    seed::<App>(&boot.app_context).await.unwrap();

    // 创建多层级的书籍结构
    let level1 = ActiveModel {
        title: Set("第一级".to_string()),
        user_id: Set(1),
        sort_order: Set(Some(1)),
        ..Default::default()
    }
    .insert(&boot.app_context.db)
    .await
    .unwrap();

    let _level2_1 = ActiveModel {
        title: Set("第二级-第一章".to_string()),
        user_id: Set(1),
        parent_id: Set(Some(level1.id)),
        sort_order: Set(Some(1)),
        ..Default::default()
    }
    .insert(&boot.app_context.db)
    .await
    .unwrap();

    let _level2_2 = ActiveModel {
        title: Set("第二级-第二章".to_string()),
        user_id: Set(1),
        parent_id: Set(Some(level1.id)),
        sort_order: Set(Some(2)),
        ..Default::default()
    }
    .insert(&boot.app_context.db)
    .await
    .unwrap();

    // 查询第一级书籍的所有子书籍
    let children = Entity::find()
        .filter(books::Column::ParentId.eq(Some(level1.id)))
        .order_by_asc(books::Column::SortOrder)
        .all(&boot.app_context.db)
        .await
        .unwrap();

    assert_eq!(children.len(), 2);
    assert_eq!(children[0].title, "第二级-第一章");
    assert_eq!(children[1].title, "第二级-第二章");

    with_settings!({
        filters => cleanup_book_model()
    }, {
        assert_debug_snapshot!(children);
    });
}

#[tokio::test]
#[serial]
async fn can_paginate_books() {
    configure_insta!();

    let boot = boot_test::<App>().await.unwrap();
    seed::<App>(&boot.app_context).await.unwrap();

    // 创建多本书籍用于分页测试
    for i in 1..=10 {
        ActiveModel {
            title: Set(format!("书籍 {:02}", i)),
            user_id: Set(1),
            ..Default::default()
        }
        .insert(&boot.app_context.db)
        .await
        .unwrap();
    }

    // 分页查询：第1页，每页3条
    let page1 = Entity::find()
        .filter(books::Column::UserId.eq(1))
        .order_by_asc(books::Column::Title)
        .paginate(&boot.app_context.db, 3);

    let page1_results = page1.fetch_page(0).await.unwrap();
    assert_eq!(page1_results.len(), 3);

    // 分页查询：第2页，每页3条
    let page2_results = page1.fetch_page(1).await.unwrap();
    assert_eq!(page2_results.len(), 3);

    // 验证排序正确
    assert_eq!(page1_results[0].title, "书籍 01");
    assert_eq!(page1_results[1].title, "书籍 02");
    assert_eq!(page1_results[2].title, "书籍 03");
    assert_eq!(page2_results[0].title, "书籍 04");

    with_settings!({
        filters => cleanup_book_model()
    }, {
        assert_debug_snapshot!(page1_results);
    });
}

fn cleanup_book_model() -> Vec<(&'static str, &'static str)> {
    vec![
        (r"id: \d+,", "id: ID"),
        (r"user_id: \d+,", "user_id: USER_ID"),
        (r"parent_id: Some\(\d+\)", "parent_id: Some(PARENT_ID)"),
        (
            r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(\.\d+)?\+\d{2}:\d{2}",
            "DATE",
        ),
    ]
}
