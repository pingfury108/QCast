use insta::{assert_debug_snapshot, with_settings};
use loco_rs::testing::prelude::*;
use qcast::app::App;
use qcast::models::_entities::chapters::ActiveModel;
use qcast::models::chapters;
use sea_orm::{ActiveModelTrait, Set};
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
async fn can_search_chapters_with_parents() {
    configure_insta!();

    let boot = boot_test::<App>().await.unwrap();
    seed::<App>(&boot.app_context).await.unwrap();

    // 创建一个测试书籍
    let book = qcast::models::_entities::books::ActiveModel {
        title: Set("测试书籍".to_string()),
        user_id: Set(1),
        ..Default::default()
    }
    .insert(&boot.app_context.db)
    .await
    .unwrap();

    // 创建章节层级结构
    let root_chapter = ActiveModel {
        title: Set("根章节".to_string()),
        book_id: Set(book.id),
        sort_order: Set(Some(1)),
        ..Default::default()
    }
    .insert(&boot.app_context.db)
    .await
    .unwrap();

    let child_chapter = ActiveModel {
        title: Set("子章节".to_string()),
        book_id: Set(book.id),
        parent_id: Set(Some(root_chapter.id)),
        sort_order: Set(Some(1)),
        ..Default::default()
    }
    .insert(&boot.app_context.db)
    .await
    .unwrap();

    let grandchild_chapter = ActiveModel {
        title: Set("孙子章节目标".to_string()),
        description: Set(Some("这是目标章节".to_string())),
        book_id: Set(book.id),
        parent_id: Set(Some(child_chapter.id)),
        sort_order: Set(Some(1)),
        ..Default::default()
    }
    .insert(&boot.app_context.db)
    .await
    .unwrap();

    // 创建一个无关的章节
    let _other_chapter = ActiveModel {
        title: Set("无关章节".to_string()),
        book_id: Set(book.id),
        ..Default::default()
    }
    .insert(&boot.app_context.db)
    .await
    .unwrap();

    // 使用 search_with_parents 搜索包含"目标"的章节
    let search_results =
        chapters::Model::search_with_parents(&boot.app_context.db, book.id, "目标")
            .await
            .unwrap();

    // 应该返回孙子章节、子章节和根章节（3个）
    assert_eq!(search_results.len(), 3);

    // 验证包含所有祖先
    let ids: Vec<i32> = search_results.iter().map(|c| c.id).collect();
    assert!(ids.contains(&root_chapter.id));
    assert!(ids.contains(&child_chapter.id));
    assert!(ids.contains(&grandchild_chapter.id));

    // 验证不包含无关章节
    assert!(!ids.contains(&_other_chapter.id));

    with_settings!({
        filters => cleanup_chapter_model()
    }, {
        assert_debug_snapshot!(search_results);
    });
}

fn cleanup_chapter_model() -> Vec<(&'static str, &'static str)> {
    vec![
        (r"id: \d+,", "id: ID"),
        (r"book_id: \d+,", "book_id: BOOK_ID"),
        (r"parent_id: Some\(\d+\)", "parent_id: Some(PARENT_ID)"),
        (
            r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(\.\d+)?\+\d{2}:\d{2}",
            "DATE",
        ),
    ]
}
