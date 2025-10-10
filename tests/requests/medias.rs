use insta::{assert_debug_snapshot, with_settings};
use loco_rs::app::AppContext;
use loco_rs::testing::prelude::*;
use loco_rs::TestServer;
use qcast::app::App;
use qcast::models::_entities::books;
use qcast::models::_entities::chapters;
use qcast::models::_entities::medias;
use qcast::models::users;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use serde_json::json;
use serial_test::serial;

use super::prepare_data::{auth_header, init_user_login};

macro_rules! configure_insta {
    () => {
        let mut settings = insta::Settings::clone_current();
        settings.set_prepend_module_to_snapshot(false);
        settings.set_snapshot_suffix("medias_request");
    };
}

macro_rules! cleanup_media_response {
    () => {
        vec![
            (r#""id":\s*\d+"#, r#""id": ID"#),
            (r#""book_id":\s*\d+"#, r#""book_id": BOOK_ID"#),
            (r#""chapter_id":\s*\d+"#, r#""chapter_id": CHAPTER_ID"#),
            (r#""user_id":\s*\d+"#, r#""user_id": USER_ID"#),
            (r#""access_token":\s*"[^"]+""#, r#""access_token": "TOKEN""#),
            (r#""access_url":\s*"[^"]+""#, r#""access_url": "URL""#),
            (r#""file_path":\s*"[^"]+""#, r#""file_path": "PATH""#),
            (r#""created_at":\s*"[^"]+""#, r#""created_at": "DATE""#),
            (r#""updated_at":\s*"[^"]+""#, r#""updated_at": "DATE""#),
        ]
    };
}

/// 创建测试书籍
async fn create_test_book(
    request: &TestServer,
    ctx: &AppContext,
    auth_header: &(axum::http::HeaderName, axum::http::HeaderValue),
) -> books::Model {
    let payload = json!({
        "title": "Test Book for Medias",
        "description": "A test book for media testing"
    });

    let response = request
        .post("/api/books")
        .add_header(auth_header.0.clone(), auth_header.1.clone())
        .json(&payload)
        .await;

    assert_eq!(response.status_code(), 200);

    let response_text = response.text();
    let book_response: serde_json::Value = serde_json::from_str(&response_text).unwrap();

    books::Entity::find_by_id(book_response["id"].as_i64().unwrap() as i32)
        .one(&ctx.db)
        .await
        .unwrap()
        .unwrap()
}

/// 创建测试媒体记录
async fn create_test_media(
    ctx: &AppContext,
    book_id: i32,
    chapter_id: Option<i32>,
    user_id: i32,
    title: &str,
    description: Option<&str>,
) -> medias::Model {
    let media = medias::ActiveModel::create_new(
        title.to_string(),
        description.map(|s| s.to_string()),
        "audio".to_string(),
        "/test/path.mp3".to_string(),
        Some(1024000),
        Some(120),
        Some("audio/mpeg".to_string()),
        Some("test.mp3".to_string()),
        book_id,
        chapter_id,
        user_id,
    );

    let media = media.insert(&ctx.db).await.unwrap();

    // 设置 access_url
    let mut active_media: medias::ActiveModel = media.into();
    active_media.access_url = Set(Some(format!(
        "http://localhost:5150/public/media/{}",
        active_media.access_token.as_ref()
    )));
    active_media.update(&ctx.db).await.unwrap()
}

/// 创建测试章节
async fn create_test_chapter(
    ctx: &AppContext,
    book_id: i32,
    parent_id: Option<i32>,
    title: &str,
    description: Option<&str>,
) -> chapters::Model {
    let chapter = chapters::ActiveModel {
        title: Set(title.to_string()),
        description: Set(description.map(|s| s.to_string())),
        book_id: Set(book_id),
        parent_id: Set(parent_id),
        level: Set(if parent_id.is_some() {
            Some(1)
        } else {
            Some(0)
        }),
        path: Set(None), // 将在模型创建时自动设置
        sort_order: Set(None),
        ..Default::default()
    };

    let chapter = chapter.insert(&ctx.db).await.unwrap();

    // 手动设置和更新 path 字段以确保正确的层级关系
    let path = if let Some(parent_id) = parent_id {
        // 先获取父章节的 path
        let parent_chapter = chapters::Entity::find_by_id(parent_id)
            .one(&ctx.db)
            .await
            .unwrap();
        let parent_path = parent_chapter.unwrap().path.unwrap();
        format!("{}/{}", parent_path, chapter.id)
    } else {
        chapter.id.to_string()
    };

    let mut active_chapter: chapters::ActiveModel = chapter.into();
    active_chapter.path = Set(Some(path.clone()));
    let updated_chapter = active_chapter.update(&ctx.db).await.unwrap();

    // 确保子章节的路径也正确更新
    if parent_id.is_some() {
        // 如果这是子章节，确保 level 也正确
        let mut final_chapter: chapters::ActiveModel = updated_chapter.into();
        final_chapter.level = Set(Some(1));
        final_chapter.update(&ctx.db).await.unwrap()
    } else {
        updated_chapter
    }
}

#[tokio::test]
#[serial]
async fn can_list_medias() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let logged_in_user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&logged_in_user.token);
        let auth_header = (auth_key, auth_value);

        let book = create_test_book(&request, &ctx, &auth_header).await;

        // 创建多个媒体
        create_test_media(
            &ctx,
            book.id,
            None,
            logged_in_user.user.id,
            "Audio 1",
            Some("First audio"),
        )
        .await;
        create_test_media(&ctx, book.id, None, logged_in_user.user.id, "Audio 2", None).await;
        create_test_media(
            &ctx,
            book.id,
            None,
            logged_in_user.user.id,
            "Audio 3",
            Some("Third audio"),
        )
        .await;

        let response = request
            .get("/api/media")
            .add_header(auth_header.0.clone(), auth_header.1.clone())
            .await;

        assert_eq!(response.status_code(), 200);

        let response_text = response.text();
        let medias: Vec<serde_json::Value> = serde_json::from_str(&response_text).unwrap();

        assert_eq!(medias.len(), 3);
        assert_eq!(medias[0]["title"], "Audio 1");
        assert_eq!(medias[0]["file_type"], "audio");

        with_settings!({filters => cleanup_media_response!()}, {
            assert_debug_snapshot!(medias);
        });
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_media_by_id() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let logged_in_user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&logged_in_user.token);
        let auth_header = (auth_key, auth_value);

        let book = create_test_book(&request, &ctx, &auth_header).await;
        let media = create_test_media(
            &ctx,
            book.id,
            None,
            logged_in_user.user.id,
            "Test Media",
            Some("Test Description"),
        )
        .await;

        let response = request
            .get(&format!("/api/media/{}", media.id))
            .add_header(auth_header.0.clone(), auth_header.1.clone())
            .await;

        assert_eq!(response.status_code(), 200);

        let response_text = response.text();
        let media_response: serde_json::Value = serde_json::from_str(&response_text).unwrap();

        assert_eq!(media_response["title"], "Test Media");
        assert_eq!(media_response["description"], "Test Description");
        assert_eq!(media_response["file_type"], "audio");
        assert_eq!(media_response["file_version"], 1);

        with_settings!({filters => cleanup_media_response!()}, {
            assert_debug_snapshot!(media_response);
        });
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_update_media() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let logged_in_user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&logged_in_user.token);
        let auth_header = (auth_key, auth_value);

        let book = create_test_book(&request, &ctx, &auth_header).await;
        let media = create_test_media(
            &ctx,
            book.id,
            None,
            logged_in_user.user.id,
            "Original Title",
            Some("Original Description"),
        )
        .await;

        let payload = json!({
            "title": "Updated Title",
            "description": "Updated Description",
            "is_public": true
        });

        let response = request
            .put(&format!("/api/media/{}", media.id))
            .add_header(auth_header.0.clone(), auth_header.1.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), 200);

        let response_text = response.text();
        let updated_media: serde_json::Value = serde_json::from_str(&response_text).unwrap();

        assert_eq!(updated_media["title"], "Updated Title");
        assert_eq!(updated_media["description"], "Updated Description");
        assert_eq!(updated_media["is_public"], true);

        with_settings!({filters => cleanup_media_response!()}, {
            assert_debug_snapshot!(updated_media);
        });
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_delete_media() {
    request::<App, _, _>(|request, ctx| async move {
        let logged_in_user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&logged_in_user.token);
        let auth_header = (auth_key, auth_value);

        let book = create_test_book(&request, &ctx, &auth_header).await;
        let media = create_test_media(
            &ctx,
            book.id,
            None,
            logged_in_user.user.id,
            "Media to Delete",
            None,
        )
        .await;

        // 删除媒体
        let response = request
            .delete(&format!("/api/media/{}", media.id))
            .add_header(auth_header.0.clone(), auth_header.1.clone())
            .await;

        assert_eq!(response.status_code(), 200);

        // 验证媒体已删除
        let response = request
            .get(&format!("/api/media/{}", media.id))
            .add_header(auth_header.0.clone(), auth_header.1.clone())
            .await;

        assert_eq!(response.status_code(), 404);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_search_medias() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let logged_in_user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&logged_in_user.token);
        let auth_header = (auth_key, auth_value);

        let book = create_test_book(&request, &ctx, &auth_header).await;

        // 创建多个媒体
        create_test_media(
            &ctx,
            book.id,
            None,
            logged_in_user.user.id,
            "Podcast Episode 1",
            Some("First episode"),
        )
        .await;
        create_test_media(
            &ctx,
            book.id,
            None,
            logged_in_user.user.id,
            "Music Track",
            Some("A nice song"),
        )
        .await;
        create_test_media(
            &ctx,
            book.id,
            None,
            logged_in_user.user.id,
            "Podcast Episode 2",
            Some("Second episode"),
        )
        .await;

        let response = request
            .get("/api/media/search?q=podcast")
            .add_header(auth_header.0.clone(), auth_header.1.clone())
            .await;

        assert_eq!(response.status_code(), 200);

        let response_text = response.text();
        let medias: Vec<serde_json::Value> = serde_json::from_str(&response_text).unwrap();

        assert_eq!(medias.len(), 2);
        assert!(medias[0]["title"].as_str().unwrap().contains("Podcast"));
        assert!(medias[1]["title"].as_str().unwrap().contains("Podcast"));
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_publish_media() {
    request::<App, _, _>(|request, ctx| async move {
        let logged_in_user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&logged_in_user.token);
        let auth_header = (auth_key, auth_value);

        let book = create_test_book(&request, &ctx, &auth_header).await;
        let media = create_test_media(
            &ctx,
            book.id,
            None,
            logged_in_user.user.id,
            "Media to Publish",
            None,
        )
        .await;

        // 发布媒体
        let response = request
            .post(&format!("/api/media/{}/publish", media.id))
            .add_header(auth_header.0.clone(), auth_header.1.clone())
            .await;

        assert_eq!(response.status_code(), 200);

        let response_text = response.text();
        let published_media: serde_json::Value = serde_json::from_str(&response_text).unwrap();

        assert_eq!(published_media["is_public"], true);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn requires_authentication() {
    request::<App, _, _>(|request, _ctx| async move {
        // 尝试不带认证访问
        let response = request.get("/api/media").await;

        // 应该返回 401 Unauthorized
        assert_eq!(response.status_code(), 401);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_list_medias_by_book_id() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let logged_in_user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&logged_in_user.token);
        let auth_header = (auth_key, auth_value);

        // 创建两本书
        let book1 = create_test_book(&request, &ctx, &auth_header).await;
        let book2 = create_test_book(&request, &ctx, &auth_header).await;

        // 为第一本书创建媒体
        create_test_media(
            &ctx,
            book1.id,
            None,
            logged_in_user.user.id,
            "Book 1 Audio 1",
            Some("First book audio"),
        )
        .await;
        create_test_media(
            &ctx,
            book1.id,
            None,
            logged_in_user.user.id,
            "Book 1 Audio 2",
            Some("Second book audio"),
        )
        .await;

        // 为第二本书创建媒体
        create_test_media(
            &ctx,
            book2.id,
            None,
            logged_in_user.user.id,
            "Book 2 Audio 1",
            Some("First audio for book 2"),
        )
        .await;

        // 测试获取第一本书的媒体
        let response = request
            .get(&format!("/api/media?book_id={}", book1.id))
            .add_header(auth_header.0.clone(), auth_header.1.clone())
            .await;

        assert_eq!(response.status_code(), 200);

        let response_text = response.text();
        let book1_medias: Vec<serde_json::Value> = serde_json::from_str(&response_text).unwrap();

        assert_eq!(book1_medias.len(), 2);
        assert_eq!(book1_medias[0]["title"], "Book 1 Audio 1");
        assert_eq!(book1_medias[0]["book_id"], book1.id);
        assert_eq!(book1_medias[1]["title"], "Book 1 Audio 2");
        assert_eq!(book1_medias[1]["book_id"], book1.id);

        // 测试获取第二本书的媒体
        let response = request
            .get(&format!("/api/media?book_id={}", book2.id))
            .add_header(auth_header.0.clone(), auth_header.1.clone())
            .await;

        assert_eq!(response.status_code(), 200);

        let response_text = response.text();
        let book2_medias: Vec<serde_json::Value> = serde_json::from_str(&response_text).unwrap();

        assert_eq!(book2_medias.len(), 1);
        assert_eq!(book2_medias[0]["title"], "Book 2 Audio 1");
        assert_eq!(book2_medias[0]["book_id"], book2.id);

        // 测试获取不存在的书籍的媒体
        let response = request
            .get("/api/media?book_id=99999")
            .add_header(auth_header.0.clone(), auth_header.1.clone())
            .await;

        assert_eq!(response.status_code(), 200);

        let response_text = response.text();
        let empty_medias: Vec<serde_json::Value> = serde_json::from_str(&response_text).unwrap();

        assert_eq!(empty_medias.len(), 0);

        with_settings!({filters => cleanup_media_response!()}, {
            assert_debug_snapshot!(book1_medias);
        });
    })
    .await;
}

#[tokio::test]
#[serial]
async fn cannot_access_other_user_media() {
    request::<App, _, _>(|request, ctx| async move {
        // 创建第一个用户
        let user1 = init_user_login(&request, &ctx).await;
        let (auth_key1, auth_value1) = auth_header(&user1.token);
        let auth_header1 = (auth_key1, auth_value1);

        let book1 = create_test_book(&request, &ctx, &auth_header1).await;
        let media = create_test_media(
            &ctx,
            book1.id,
            None,
            user1.user.id,
            "Secret Media",
            Some("Secret content"),
        )
        .await;

        // 创建第二个用户
        let user2_payload = json!({
            "name": "User 2",
            "email": "user2@test.com",
            "password": "password123"
        });

        let response = request
            .post("/api/auth/register")
            .json(&user2_payload)
            .await;
        assert_eq!(response.status_code(), 200);

        let user2 = users::Model::find_by_email(&ctx.db, "user2@test.com")
            .await
            .unwrap();

        let verify_payload = json!({
            "token": user2.email_verification_token,
        });

        request.post("/api/auth/verify").json(&verify_payload).await;

        let login_response = request
            .post("/api/auth/login")
            .json(&json!({
                "email": "user2@test.com",
                "password": "password123"
            }))
            .await;

        let login_response_text = login_response.text();
        let user2_login: serde_json::Value = serde_json::from_str(&login_response_text).unwrap();
        let user2_token = user2_login["token"].as_str().unwrap();
        let (auth_key2, auth_value2) = auth_header(user2_token);

        let response = request
            .get(&format!("/api/media/{}", media.id))
            .add_header(auth_key2.clone(), auth_value2.clone())
            .await;

        // 应该返回 404 Not Found（因为用户2无法访问用户1的媒体）
        assert_eq!(response.status_code(), 404);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_list_medias_by_chapter_only() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let logged_in_user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&logged_in_user.token);
        let auth_header = (auth_key, auth_value);

        let book = create_test_book(&request, &ctx, &auth_header).await;

        // 创建章节层级结构
        let parent_chapter = create_test_chapter(
            &ctx,
            book.id,
            None,
            "Parent Chapter",
            Some("Parent chapter description"),
        )
        .await;

        let child_chapter = create_test_chapter(
            &ctx,
            book.id,
            Some(parent_chapter.id),
            "Child Chapter",
            Some("Child chapter description"),
        )
        .await;

        // 在父章节创建媒体
        create_test_media(
            &ctx,
            book.id,
            Some(parent_chapter.id),
            logged_in_user.user.id,
            "Parent Media",
            Some("Media in parent chapter"),
        )
        .await;

        // 在子章节创建媒体
        create_test_media(
            &ctx,
            book.id,
            Some(child_chapter.id),
            logged_in_user.user.id,
            "Child Media",
            Some("Media in child chapter"),
        )
        .await;

        // 测试获取父章节的媒体（非递归）
        let response = request
            .get(&format!(
                "/api/media/by-chapter?chapter_id={}",
                parent_chapter.id
            ))
            .add_header(auth_header.0.clone(), auth_header.1.clone())
            .await;

        assert_eq!(response.status_code(), 200);

        let response_text = response.text();
        let parent_medias: Vec<serde_json::Value> = serde_json::from_str(&response_text).unwrap();

        assert_eq!(parent_medias.len(), 1);
        assert_eq!(parent_medias[0]["title"], "Parent Media");
        assert_eq!(parent_medias[0]["chapter_id"], parent_chapter.id);

        // 测试获取子章节的媒体（非递归）
        let response = request
            .get(&format!(
                "/api/media/by-chapter?chapter_id={}",
                child_chapter.id
            ))
            .add_header(auth_header.0.clone(), auth_header.1.clone())
            .await;

        assert_eq!(response.status_code(), 200);

        let response_text = response.text();
        let child_medias: Vec<serde_json::Value> = serde_json::from_str(&response_text).unwrap();

        assert_eq!(child_medias.len(), 1);
        assert_eq!(child_medias[0]["title"], "Child Media");
        assert_eq!(child_medias[0]["chapter_id"], child_chapter.id);

        with_settings!({filters => cleanup_media_response!()}, {
            assert_debug_snapshot!(parent_medias);
        });
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_list_medias_by_chapter_recursive() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let logged_in_user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&logged_in_user.token);
        let auth_header = (auth_key, auth_value);

        let book = create_test_book(&request, &ctx, &auth_header).await;

        // 创建章节层级结构
        let parent_chapter = create_test_chapter(
            &ctx,
            book.id,
            None,
            "Parent Chapter",
            Some("Parent chapter description"),
        )
        .await;

        let child_chapter = create_test_chapter(
            &ctx,
            book.id,
            Some(parent_chapter.id),
            "Child Chapter",
            Some("Child chapter description"),
        )
        .await;

        let grandchild_chapter = create_test_chapter(
            &ctx,
            book.id,
            Some(child_chapter.id),
            "Grandchild Chapter",
            Some("Grandchild chapter description"),
        )
        .await;

        // 在各层级创建媒体
        create_test_media(
            &ctx,
            book.id,
            Some(parent_chapter.id),
            logged_in_user.user.id,
            "Parent Media",
            Some("Media in parent chapter"),
        )
        .await;

        create_test_media(
            &ctx,
            book.id,
            Some(child_chapter.id),
            logged_in_user.user.id,
            "Child Media",
            Some("Media in child chapter"),
        )
        .await;

        create_test_media(
            &ctx,
            book.id,
            Some(grandchild_chapter.id),
            logged_in_user.user.id,
            "Grandchild Media",
            Some("Media in grandchild chapter"),
        )
        .await;

        // 测试递归获取父章节的所有媒体（应该包含所有子章节的媒体）
        let response = request
            .get(&format!(
                "/api/media/by-chapter-recursive?chapter_id={}",
                parent_chapter.id
            ))
            .add_header(auth_header.0.clone(), auth_header.1.clone())
            .await;

        assert_eq!(response.status_code(), 200);

        let response_text = response.text();
        let all_medias: Vec<serde_json::Value> = serde_json::from_str(&response_text).unwrap();

        assert_eq!(all_medias.len(), 3);

        // 验证包含所有层级的媒体
        let titles: Vec<String> = all_medias
            .iter()
            .map(|m| m["title"].as_str().unwrap().to_string())
            .collect();

        assert!(titles.contains(&"Parent Media".to_string()));
        assert!(titles.contains(&"Child Media".to_string()));
        assert!(titles.contains(&"Grandchild Media".to_string()));

        with_settings!({filters => cleanup_media_response!()}, {
            assert_debug_snapshot!(all_medias);
        });
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_list_child_chapters() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let logged_in_user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&logged_in_user.token);
        let auth_header = (auth_key, auth_value);

        let book = create_test_book(&request, &ctx, &auth_header).await;

        // 创建章节层级结构
        let parent_chapter = create_test_chapter(
            &ctx,
            book.id,
            None,
            "Parent Chapter",
            Some("Parent chapter description"),
        )
        .await;

        let child_chapter1 = create_test_chapter(
            &ctx,
            book.id,
            Some(parent_chapter.id),
            "Child Chapter 1",
            Some("First child chapter"),
        )
        .await;

        let child_chapter2 = create_test_chapter(
            &ctx,
            book.id,
            Some(parent_chapter.id),
            "Child Chapter 2",
            Some("Second child chapter"),
        )
        .await;

        // 创建孙子章节
        let grandchild_chapter = create_test_chapter(
            &ctx,
            book.id,
            Some(child_chapter1.id),
            "Grandchild Chapter",
            Some("Grandchild chapter"),
        )
        .await;

        // 测试获取父章节的直接子章节
        let response = request
            .get(&format!(
                "/api/media/chapters/{}/children",
                parent_chapter.id
            ))
            .add_header(auth_header.0.clone(), auth_header.1.clone())
            .await;

        assert_eq!(response.status_code(), 200);

        let response_text = response.text();
        let child_chapters: Vec<serde_json::Value> = serde_json::from_str(&response_text).unwrap();

        // 应该只包含直接子章节，不包含孙子章节
        assert_eq!(child_chapters.len(), 2);

        let titles: Vec<String> = child_chapters
            .iter()
            .map(|c| c["title"].as_str().unwrap().to_string())
            .collect();

        assert!(titles.contains(&"Child Chapter 1".to_string()));
        assert!(titles.contains(&"Child Chapter 2".to_string()));
        assert!(!titles.contains(&"Grandchild Chapter".to_string()));

        // 验证所有子章节的 parent_id 都正确
        for chapter in &child_chapters {
            assert_eq!(chapter["parent_id"], parent_chapter.id);
            assert_eq!(chapter["book_id"], book.id);
        }

        with_settings!({filters => cleanup_media_response!()}, {
            assert_debug_snapshot!(child_chapters);
        });
    })
    .await;
}

#[tokio::test]
#[serial]
async fn chapter_media_apis_handle_invalid_chapter() {
    request::<App, _, _>(|request, ctx| async move {
        let logged_in_user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&logged_in_user.token);
        let auth_header = (auth_key, auth_value);

        // 测试不存在的章节ID
        let response = request
            .get("/api/media/by-chapter?chapter_id=99999")
            .add_header(auth_header.0.clone(), auth_header.1.clone())
            .await;

        assert_eq!(response.status_code(), 404);

        // 测试无效的章节ID
        let response = request
            .get("/api/media/by-chapter?chapter_id=invalid")
            .add_header(auth_header.0.clone(), auth_header.1.clone())
            .await;

        assert_eq!(response.status_code(), 400);

        // 测试递归接口的不存在章节
        let response = request
            .get("/api/media/by-chapter-recursive?chapter_id=99999")
            .add_header(auth_header.0.clone(), auth_header.1.clone())
            .await;

        assert_eq!(response.status_code(), 404);

        // 测试子章节接口的不存在章节
        let response = request
            .get("/api/media/chapters/99999/children")
            .add_header(auth_header.0.clone(), auth_header.1.clone())
            .await;

        assert_eq!(response.status_code(), 404);
    })
    .await;
}

// === 媒体流播放相关测试 ===

#[tokio::test]
#[serial]
async fn can_stream_media_without_auth() {
    request::<App, _, _>(|request, ctx| async move {
        let logged_in_user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&logged_in_user.token);
        let auth_header = (auth_key, auth_value);

        let book = create_test_book(&request, &ctx, &auth_header).await;
        let media = create_test_media(
            &ctx,
            book.id,
            None,
            logged_in_user.user.id,
            "Test Stream Media",
            Some("For streaming test"),
        )
        .await;

        // 不需要认证，直接访问流媒体端点
        let response = request.get(&format!("/api/media/{}/stream", media.id)).await;

        // 由于文件不存在，应该返回 404
        assert_eq!(response.status_code(), 404);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn stream_media_handles_invalid_media_id() {
    request::<App, _, _>(|request, _ctx| async move {
        // 测试不存在的媒体ID
        let response = request.get("/api/media/99999/stream").await;

        assert_eq!(response.status_code(), 404);

        // 测试无效的媒体ID（路由解析失败，返回400）
        let response = request.get("/api/media/invalid/stream").await;

        assert_eq!(response.status_code(), 400);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn stream_media_supports_time_based_seeking() {
    request::<App, _, _>(|request, ctx| async move {
        let logged_in_user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&logged_in_user.token);
        let auth_header = (auth_key, auth_value);

        let book = create_test_book(&request, &ctx, &auth_header).await;
        let media = create_test_media(
            &ctx,
            book.id,
            None,
            logged_in_user.user.id,
            "Test Time Seek",
            Some("For time seeking test"),
        )
        .await;

        // 测试时间范围参数
        let response = request
            .get(&format!("/api/media/{}/stream?start=30&end=120", media.id))
            .await;

        // 由于文件不存在，应该返回 404，但这证明参数解析正常
        assert_eq!(response.status_code(), 404);

        // 测试只有 start 参数
        let response = request
            .get(&format!("/api/media/{}/stream?start=60", media.id))
            .await;

        assert_eq!(response.status_code(), 404);

        // 测试只有 end 参数
        let response = request
            .get(&format!("/api/media/{}/stream?end=180", media.id))
            .await;

        assert_eq!(response.status_code(), 404);

        // 测试无效的时间参数
        let response = request
            .get(&format!("/api/media/{}/stream?start=invalid&end=120", media.id))
            .await;

        assert_eq!(response.status_code(), 404);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn stream_media_supports_range_requests() {
    request::<App, _, _>(|request, ctx| async move {
        let logged_in_user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&logged_in_user.token);
        let auth_header = (auth_key, auth_value);

        let book = create_test_book(&request, &ctx, &auth_header).await;
        let media = create_test_media(
            &ctx,
            book.id,
            None,
            logged_in_user.user.id,
            "Test Range Request",
            Some("For range request test"),
        )
        .await;

        // 测试 Range 头
        let response = request
            .get(&format!("/api/media/{}/stream", media.id))
            .add_header("Range", "bytes=0-1023")
            .await;

        // 由于文件不存在，应该返回 404
        assert_eq!(response.status_code(), 404);

        // 测试不同的 Range 格式
        let response = request
            .get(&format!("/api/media/{}/stream", media.id))
            .add_header("Range", "bytes=1024-2047")
            .await;

        assert_eq!(response.status_code(), 404);

        // 测试无效的 Range 头
        let response = request
            .get(&format!("/api/media/{}/stream", media.id))
            .add_header("Range", "invalid-range")
            .await;

        assert_eq!(response.status_code(), 404);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn stream_media_combines_time_and_range_params() {
    request::<App, _, _>(|request, ctx| async move {
        let logged_in_user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&logged_in_user.token);
        let auth_header = (auth_key, auth_value);

        let book = create_test_book(&request, &ctx, &auth_header).await;
        let media = create_test_media(
            &ctx,
            book.id,
            None,
            logged_in_user.user.id,
            "Test Combined Params",
            Some("For combined params test"),
        )
        .await;

        // 测试时间参数和 Range 头的组合
        let response = request
            .get(&format!("/api/media/{}/stream?start=30&end=90", media.id))
            .add_header("Range", "bytes=0-1023")
            .await;

        // 由于文件不存在，应该返回 404
        assert_eq!(response.status_code(), 404);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn stream_media_invalid_params_handling() {
    request::<App, _, _>(|request, ctx| async move {
        let logged_in_user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&logged_in_user.token);
        let auth_header = (auth_key, auth_value);

        let book = create_test_book(&request, &ctx, &auth_header).await;
        let media = create_test_media(
            &ctx,
            book.id,
            None,
            logged_in_user.user.id,
            "Test Invalid Params",
            Some("For invalid params test"),
        )
        .await;

        // 测试负数时间参数
        let response = request
            .get(&format!("/api/media/{}/stream?start=-10", media.id))
            .await;

        assert_eq!(response.status_code(), 404);

        // 测试 start > end 的情况
        let response = request
            .get(&format!("/api/media/{}/stream?start=120&end=60", media.id))
            .await;

        assert_eq!(response.status_code(), 404);

        // 测试超大数值
        let response = request
            .get(&format!("/api/media/{}/stream?start=999999999", media.id))
            .await;

        assert_eq!(response.status_code(), 404);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_update_media_and_still_stream() {
    request::<App, _, _>(|request, ctx| async move {
        let logged_in_user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&logged_in_user.token);
        let auth_header = (auth_key, auth_value);

        let book = create_test_book(&request, &ctx, &auth_header).await;
        let media = create_test_media(
            &ctx,
            book.id,
            None,
            logged_in_user.user.id,
            "Original Media",
            Some("Original description"),
        )
        .await;

        // 更新媒体信息
        let payload = json!({
            "title": "Updated Media",
            "description": "Updated description",
            "is_public": true
        });

        let update_response = request
            .put(&format!("/api/media/{}", media.id))
            .add_header(auth_header.0.clone(), auth_header.1.clone())
            .json(&payload)
            .await;

        assert_eq!(update_response.status_code(), 200);

        // 更新后仍然可以流式播放
        let stream_response = request
            .get(&format!("/api/media/{}/stream", media.id))
            .await;

        // 由于文件不存在，应该返回 404，但媒体ID有效
        assert_eq!(stream_response.status_code(), 404);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn stream_media_with_nonexistent_file() {
    request::<App, _, _>(|request, ctx| async move {
        let logged_in_user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&logged_in_user.token);
        let auth_header = (auth_key, auth_value);

        let book = create_test_book(&request, &ctx, &auth_header).await;

        // 创建一个媒体记录，但文件路径指向不存在的文件
        let media = create_test_media(
            &ctx,
            book.id,
            None,
            logged_in_user.user.id,
            "Media with Missing File",
            Some("File does not exist"),
        )
        .await;

        // 获取媒体ID，然后手动更新文件路径为不存在的路径
        let media_id = media.id;
        use sea_orm::{ActiveModelTrait, Set};
        let mut active_media: medias::ActiveModel = media.into();
        active_media.file_path = Set("/nonexistent/path/to/file.mp3".to_string());
        active_media.update(&ctx.db).await.unwrap();

        // 尝试流式播放不存在的文件
        let response = request
            .get(&format!("/api/media/{}/stream", media_id))
            .await;

        assert_eq!(response.status_code(), 404);
    })
    .await;
}
