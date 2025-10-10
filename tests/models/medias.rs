use insta::{assert_debug_snapshot, with_settings};
use loco_rs::testing::prelude::*;
use qcast::app::App;
use qcast::models::_entities::medias::ActiveModel;
use qcast::models::medias;
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
async fn can_search_medias_by_book() {
    configure_insta!();

    let boot = boot_test::<App>().await.unwrap();
    seed::<App>(&boot.app_context).await.unwrap();

    // 创建两个测试书籍
    let book1 = qcast::models::_entities::books::ActiveModel {
        title: Set("书籍1".to_string()),
        user_id: Set(1),
        ..Default::default()
    }
    .insert(&boot.app_context.db)
    .await
    .unwrap();

    let book2 = qcast::models::_entities::books::ActiveModel {
        title: Set("书籍2".to_string()),
        user_id: Set(1),
        ..Default::default()
    }
    .insert(&boot.app_context.db)
    .await
    .unwrap();

    // 在书籍1中创建媒体
    let media1 = ActiveModel {
        title: Set("目标媒体1".to_string()),
        description: Set(Some("这是第一个目标媒体".to_string())),
        book_id: Set(book1.id),
        user_id: Set(1),
        file_path: Set("/test/media1.mp3".to_string()),
        file_type: Set("audio".to_string()),
        file_size: Set(Some(1024)),
        original_filename: Set(Some("media1.mp3".to_string())),
        access_token: Set("test_token_1".to_string()),
        file_version: Set(1),
        play_count: Set(0),
        is_public: Set(false),
        ..Default::default()
    }
    .insert(&boot.app_context.db)
    .await
    .unwrap();

    let media2 = ActiveModel {
        title: Set("普通媒体".to_string()),
        description: Set(Some("这是一个普通媒体".to_string())),
        book_id: Set(book1.id),
        user_id: Set(1),
        file_path: Set("/test/media2.mp3".to_string()),
        file_type: Set("audio".to_string()),
        file_size: Set(Some(2048)),
        original_filename: Set(Some("media2.mp3".to_string())),
        access_token: Set("test_token_2".to_string()),
        file_version: Set(1),
        play_count: Set(0),
        is_public: Set(false),
        ..Default::default()
    }
    .insert(&boot.app_context.db)
    .await
    .unwrap();

    let media3 = ActiveModel {
        title: Set("目标媒体2".to_string()),
        description: Set(Some("这是第二个目标媒体".to_string())),
        book_id: Set(book1.id),
        user_id: Set(1),
        file_path: Set("/test/media3.mp3".to_string()),
        file_type: Set("audio".to_string()),
        file_size: Set(Some(3072)),
        original_filename: Set(Some("media3.mp3".to_string())),
        access_token: Set("test_token_3".to_string()),
        file_version: Set(1),
        play_count: Set(0),
        is_public: Set(false),
        ..Default::default()
    }
    .insert(&boot.app_context.db)
    .await
    .unwrap();

    // 在书籍2中创建一个包含"目标"关键词的媒体
    let _media4 = ActiveModel {
        title: Set("书籍2的目标媒体".to_string()),
        description: Set(Some("这是书籍2的媒体".to_string())),
        book_id: Set(book2.id),
        user_id: Set(1),
        file_path: Set("/test/media4.mp3".to_string()),
        file_type: Set("audio".to_string()),
        file_size: Set(Some(4096)),
        original_filename: Set(Some("media4.mp3".to_string())),
        access_token: Set("test_token_4".to_string()),
        file_version: Set(1),
        play_count: Set(0),
        is_public: Set(false),
        ..Default::default()
    }
    .insert(&boot.app_context.db)
    .await
    .unwrap();

    // 使用 search_by_book 搜索书籍1中包含"目标"的媒体
    let search_results = medias::Model::search_by_book(&boot.app_context.db, 1, book1.id, "目标")
        .await
        .unwrap();

    // 应该返回书籍1中的2个目标媒体，不包含书籍2的媒体
    assert_eq!(search_results.len(), 2);

    // 验证只包含书籍1的媒体
    let ids: Vec<i32> = search_results.iter().map(|m| m.id).collect();
    assert!(ids.contains(&media1.id));
    assert!(ids.contains(&media3.id));
    assert!(!ids.contains(&media2.id)); // 普通媒体不应该在结果中
    assert!(!ids.contains(&_media4.id)); // 书籍2的媒体不应该在结果中

    with_settings!({
        filters => cleanup_media_model()
    }, {
        assert_debug_snapshot!(search_results);
    });
}

fn cleanup_media_model() -> Vec<(&'static str, &'static str)> {
    vec![
        (r"id: \d+,", "id: ID"),
        (r"book_id: \d+,", "book_id: BOOK_ID"),
        (r"user_id: \d+,", "user_id: USER_ID"),
        (r"chapter_id: Some\(\d+\)", "chapter_id: Some(CHAPTER_ID)"),
        (
            r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(\.\d+)?\+\d{2}:\d{2}",
            "DATE",
        ),
    ]
}
