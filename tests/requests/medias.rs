use insta::{assert_debug_snapshot, with_settings};
use loco_rs::app::AppContext;
use loco_rs::testing::prelude::*;
use loco_rs::TestServer;
use qcast::app::App;
use qcast::models::_entities::books;
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
