use insta::{assert_debug_snapshot, with_settings};
use loco_rs::app::AppContext;
use loco_rs::testing::prelude::*;
use loco_rs::TestServer;
use qcast::app::App;
use qcast::models::_entities::books;
use qcast::models::users;
use qcast::views::chapters::ChapterResponse;
use sea_orm::EntityTrait;
use serde_json::json;
use serial_test::serial;

use super::prepare_data::{auth_header, init_user_login};

macro_rules! configure_insta {
    ($($expr:expr),*) => {
        let mut settings = insta::Settings::clone_current();
        settings.set_prepend_module_to_snapshot(false);
        settings.set_snapshot_suffix("chapters_request");
        let _guard = settings.bind_to_scope();
    };
}

fn cleanup_chapter_response() -> Vec<(&'static str, &'static str)> {
    vec![
        (r#""id":\s*\d+"#, r#""id": ID"#),
        (r#""book_id":\s*\d+"#, r#""book_id": BOOK_ID"#),
        (r#""parent_id":\s*\d+"#, r#""parent_id": PARENT_ID"#),
        (r#""level":\s*\d+"#, r#""level": LEVEL"#),
        (r#""sort_order":\s*\d+"#, r#""sort_order": ORDER"#),
        (r#""path":\s*"[^"]+""#, r#""path": "PATH""#),
        (r#""created_at":\s*"[^"]+""#, r#""created_at": "DATE""#),
        (r#""updated_at":\s*"[^"]+""#, r#""updated_at": "DATE""#),
    ]
}

/// 创建测试书籍
async fn create_test_book(
    request: &TestServer,
    ctx: &AppContext,
    auth_header: &(axum::http::HeaderName, axum::http::HeaderValue),
) -> books::Model {
    let payload = json!({
        "title": "Test Book for Chapters",
        "description": "A test book"
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

/// 创建测试章节
async fn create_test_chapter(
    request: &TestServer,
    book_id: i32,
    auth_header: &(axum::http::HeaderName, axum::http::HeaderValue),
    title: &str,
    description: Option<&str>,
) -> ChapterResponse {
    let payload = if let Some(desc) = description {
        json!({
            "title": title,
            "description": desc
        })
    } else {
        json!({
            "title": title
        })
    };

    let response = request
        .post(&format!("/api/books/{}/chapters", book_id))
        .add_header(auth_header.0.clone(), auth_header.1.clone())
        .json(&payload)
        .await;

    assert_eq!(response.status_code(), 200);

    let response_text = response.text();
    serde_json::from_str(&response_text).unwrap()
}

#[tokio::test]
#[serial]
async fn can_create_chapter() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let logged_in_user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&logged_in_user.token);
        let auth_header = (auth_key, auth_value);

        let book = create_test_book(&request, &ctx, &auth_header).await;

        let payload = json!({
            "title": "Chapter 1",
            "description": "First chapter"
        });

        let response = request
            .post(&format!("/api/books/{}/chapters", book.id))
            .add_header(auth_header.0.clone(), auth_header.1.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), 200);

        let response_text = response.text();
        let chapter: serde_json::Value = serde_json::from_str(&response_text).unwrap();

        assert_eq!(chapter["title"], "Chapter 1");
        assert_eq!(chapter["description"], "First chapter");
        assert_eq!(chapter["sort_order"], 1);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_create_chapter_with_sort_order() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let logged_in_user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&logged_in_user.token);
        let auth_header = (auth_key, auth_value);

        let book = create_test_book(&request, &ctx, &auth_header).await;

        let payload = json!({
            "title": "Chapter 5",
            "description": "Fifth chapter",
            "sort_order": 5
        });

        let response = request
            .post(&format!("/api/books/{}/chapters", book.id))
            .add_header(auth_header.0.clone(), auth_header.1.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), 200);

        let response_text = response.text();
        let chapter: serde_json::Value = serde_json::from_str(&response_text).unwrap();

        assert_eq!(chapter["title"], "Chapter 5");
        assert_eq!(chapter["sort_order"], 5);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_list_chapters() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let logged_in_user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&logged_in_user.token);
        let auth_header = (auth_key, auth_value);

        let book = create_test_book(&request, &ctx, &auth_header).await;

        // 创建多个章节
        create_test_chapter(
            &request,
            book.id,
            &auth_header,
            "Chapter 1",
            Some("First chapter"),
        )
        .await;
        create_test_chapter(
            &request,
            book.id,
            &auth_header,
            "Chapter 2",
            Some("Second chapter"),
        )
        .await;
        create_test_chapter(&request, book.id, &auth_header, "Chapter 3", None).await;

        let response = request
            .get(&format!("/api/books/{}/chapters", book.id))
            .add_header(auth_header.0.clone(), auth_header.1.clone())
            .await;

        assert_eq!(response.status_code(), 200);

        let response_text = response.text();
        let chapters: Vec<serde_json::Value> = serde_json::from_str(&response_text).unwrap();

        assert_eq!(chapters.len(), 3);
        assert_eq!(chapters[0]["title"], "Chapter 1");
        assert_eq!(chapters[1]["title"], "Chapter 2");
        assert_eq!(chapters[2]["title"], "Chapter 3");

        with_settings!({filters => cleanup_chapter_response()}, {
            assert_debug_snapshot!(chapters);
        });
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_chapter_by_id() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let logged_in_user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&logged_in_user.token);
        let auth_header = (auth_key, auth_value);

        let book = create_test_book(&request, &ctx, &auth_header).await;
        let chapter = create_test_chapter(
            &request,
            book.id,
            &auth_header,
            "Test Chapter",
            Some("Test Description"),
        )
        .await;

        let response = request
            .get(&format!("/api/books/{}/chapters/{}", book.id, chapter.id))
            .add_header(auth_header.0.clone(), auth_header.1.clone())
            .await;

        assert_eq!(response.status_code(), 200);

        let response_text = response.text();
        let response_chapter: serde_json::Value = serde_json::from_str(&response_text).unwrap();

        assert_eq!(response_chapter["title"], "Test Chapter");
        assert_eq!(response_chapter["description"], "Test Description");

        with_settings!({filters => cleanup_chapter_response()}, {
            assert_debug_snapshot!(response_chapter);
        });
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_update_chapter() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let logged_in_user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&logged_in_user.token);
        let auth_header = (auth_key, auth_value);

        let book = create_test_book(&request, &ctx, &auth_header).await;
        let chapter = create_test_chapter(
            &request,
            book.id,
            &auth_header,
            "Original Title",
            Some("Original Description"),
        )
        .await;

        let payload = json!({
            "title": "Updated Title",
            "description": "Updated Description",
            "sort_order": 10
        });

        let response = request
            .put(&format!("/api/books/{}/chapters/{}", book.id, chapter.id))
            .add_header(auth_header.0.clone(), auth_header.1.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), 200);

        let response_text = response.text();
        let updated_chapter: serde_json::Value = serde_json::from_str(&response_text).unwrap();

        assert_eq!(updated_chapter["title"], "Updated Title");
        assert_eq!(updated_chapter["description"], "Updated Description");
        assert_eq!(updated_chapter["sort_order"], 10);

        with_settings!({filters => cleanup_chapter_response()}, {
            assert_debug_snapshot!(updated_chapter);
        });
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_delete_chapter() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let logged_in_user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&logged_in_user.token);
        let auth_header = (auth_key, auth_value);

        let book = create_test_book(&request, &ctx, &auth_header).await;
        let chapter =
            create_test_chapter(&request, book.id, &auth_header, "Chapter to Delete", None).await;

        // 删除章节
        let response = request
            .delete(&format!("/api/books/{}/chapters/{}", book.id, chapter.id))
            .add_header(auth_header.0.clone(), auth_header.1.clone())
            .await;

        assert_eq!(response.status_code(), 200);

        // 验证章节已删除
        let response = request
            .get(&format!("/api/books/{}/chapters/{}", book.id, chapter.id))
            .add_header(auth_header.0.clone(), auth_header.1.clone())
            .await;

        assert_eq!(response.status_code(), 404);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_reorder_chapter() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let logged_in_user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&logged_in_user.token);
        let auth_header = (auth_key, auth_value);

        let book = create_test_book(&request, &ctx, &auth_header).await;
        let chapter =
            create_test_chapter(&request, book.id, &auth_header, "Chapter to Reorder", None).await;

        let payload = json!({
            "sort_order": 5
        });

        let response = request
            .post(&format!(
                "/api/books/{}/chapters/{}/reorder",
                book.id, chapter.id
            ))
            .add_header(auth_header.0.clone(), auth_header.1.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), 200);

        let response_text = response.text();
        let reordered_chapter: serde_json::Value = serde_json::from_str(&response_text).unwrap();

        assert_eq!(reordered_chapter["sort_order"], 5);

        with_settings!({filters => cleanup_chapter_response()}, {
            assert_debug_snapshot!(reordered_chapter);
        });
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_batch_reorder_chapters() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let logged_in_user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&logged_in_user.token);
        let auth_header = (auth_key, auth_value);

        let book = create_test_book(&request, &ctx, &auth_header).await;

        // 创建多个章节
        let chapter1 =
            create_test_chapter(&request, book.id, &auth_header, "Chapter 1", None).await;
        let chapter2 =
            create_test_chapter(&request, book.id, &auth_header, "Chapter 2", None).await;
        let chapter3 =
            create_test_chapter(&request, book.id, &auth_header, "Chapter 3", None).await;

        let payload = json!({
            "chapter_ids": [chapter3.id, chapter1.id, chapter2.id]
        });

        let response = request
            .post(&format!("/api/books/{}/chapters/batch-reorder", book.id))
            .add_header(auth_header.0.clone(), auth_header.1.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), 200);

        let response_text = response.text();
        let chapters: Vec<serde_json::Value> = serde_json::from_str(&response_text).unwrap();

        assert_eq!(chapters.len(), 3);
        assert_eq!(chapters[0]["id"], chapter3.id);
        assert_eq!(chapters[0]["sort_order"], 1);
        assert_eq!(chapters[1]["id"], chapter1.id);
        assert_eq!(chapters[1]["sort_order"], 2);
        assert_eq!(chapters[2]["id"], chapter2.id);
        assert_eq!(chapters[2]["sort_order"], 3);

        with_settings!({filters => cleanup_chapter_response()}, {
            assert_debug_snapshot!(chapters);
        });
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_move_chapter_up() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let logged_in_user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&logged_in_user.token);
        let auth_header = (auth_key, auth_value);

        let book = create_test_book(&request, &ctx, &auth_header).await;

        // 创建多个章节
        let _chapter1 =
            create_test_chapter(&request, book.id, &auth_header, "Chapter 1", None).await;
        let chapter2 =
            create_test_chapter(&request, book.id, &auth_header, "Chapter 2", None).await;

        // 手动设置 chapter2 的顺序为 2
        let payload = json!({
            "sort_order": 2
        });

        let response = request
            .post(&format!(
                "/api/books/{}/chapters/{}/reorder",
                book.id, chapter2.id
            ))
            .add_header(auth_header.0.clone(), auth_header.1.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), 200);

        // 现在尝试将 chapter2 上移
        let response = request
            .post(&format!(
                "/api/books/{}/chapters/{}/move-up",
                book.id, chapter2.id
            ))
            .add_header(auth_header.0.clone(), auth_header.1.clone())
            .await;

        assert_eq!(response.status_code(), 200);

        let response_text = response.text();
        let moved_chapter: serde_json::Value = serde_json::from_str(&response_text).unwrap();

        // chapter2 现在应该在位置 1
        assert_eq!(moved_chapter["id"], chapter2.id);
        assert_eq!(moved_chapter["sort_order"], 1);

        with_settings!({filters => cleanup_chapter_response()}, {
            assert_debug_snapshot!(moved_chapter);
        });
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_move_chapter_down() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let logged_in_user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&logged_in_user.token);
        let auth_header = (auth_key, auth_value);

        let book = create_test_book(&request, &ctx, &auth_header).await;

        // 创建多个章节
        let chapter1 =
            create_test_chapter(&request, book.id, &auth_header, "Chapter 1", None).await;
        let _chapter2 =
            create_test_chapter(&request, book.id, &auth_header, "Chapter 2", None).await;

        // 尝试将 chapter1 下移
        let response = request
            .post(&format!(
                "/api/books/{}/chapters/{}/move-down",
                book.id, chapter1.id
            ))
            .add_header(auth_header.0.clone(), auth_header.1.clone())
            .await;

        assert_eq!(response.status_code(), 200);

        let response_text = response.text();
        let moved_chapter: serde_json::Value = serde_json::from_str(&response_text).unwrap();

        // chapter1 现在应该在位置 2
        assert_eq!(moved_chapter["id"], chapter1.id);
        assert_eq!(moved_chapter["sort_order"], 2);

        with_settings!({filters => cleanup_chapter_response()}, {
            assert_debug_snapshot!(moved_chapter);
        });
    })
    .await;
}

#[tokio::test]
#[serial]
async fn requires_authentication() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let logged_in_user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&logged_in_user.token);
        let auth_header = (auth_key, auth_value);

        let book = create_test_book(&request, &ctx, &auth_header).await;

        // 尝试不带认证访问
        let response = request
            .get(&format!("/api/books/{}/chapters", book.id))
            .await;

        // 应该返回 401 Unauthorized
        assert_eq!(response.status_code(), 401);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn cannot_access_other_user_chapter() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        // 创建第一个用户
        let user1 = init_user_login(&request, &ctx).await;
        let (auth_key1, auth_value1) = auth_header(&user1.token);
        let auth_header1 = (auth_key1, auth_value1);

        let book1 = create_test_book(&request, &ctx, &auth_header1).await;
        let chapter = create_test_chapter(
            &request,
            book1.id,
            &auth_header1,
            "Secret Chapter",
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
            .json(&serde_json::json!({
                "email": "user2@test.com",
                "password": "password123"
            }))
            .await;

        let login_response_text = login_response.text();
        let user2_login: serde_json::Value = serde_json::from_str(&login_response_text).unwrap();
        let user2_token = user2_login["token"].as_str().unwrap();
        let (auth_key2, auth_value2) = auth_header(user2_token);

        // 用户2尝试访问用户1的章节
        let response = request
            .get(&format!("/api/books/{}/chapters/{}", book1.id, chapter.id))
            .add_header(auth_key2.clone(), auth_value2.clone())
            .await;

        // 应该返回 404 Not Found（因为用户2无法访问用户1的书籍）
        assert_eq!(response.status_code(), 404);
    })
    .await;
}

// === 树状结构测试 ===

#[tokio::test]
#[serial]
async fn can_create_child_chapter() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let logged_in_user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&logged_in_user.token);
        let auth_header = (auth_key, auth_value);

        let book = create_test_book(&request, &ctx, &auth_header).await;

        // 先创建父章节
        let parent_chapter = create_test_chapter(
            &request,
            book.id,
            &auth_header,
            "Parent Chapter",
            Some("Parent description"),
        )
        .await;

        // 创建子章节
        let payload = json!({
            "title": "Child Chapter",
            "description": "Child description",
            "parent_id": parent_chapter.id
        });

        let response = request
            .post(&format!("/api/books/{}/chapters", book.id))
            .add_header(auth_header.0.clone(), auth_header.1.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), 200);

        let response_text = response.text();
        let child_chapter: serde_json::Value = serde_json::from_str(&response_text).unwrap();

        assert_eq!(child_chapter["title"], "Child Chapter");
        assert_eq!(child_chapter["parent_id"], parent_chapter.id);
        assert_eq!(child_chapter["level"], 1);

        with_settings!({filters => cleanup_chapter_response()}, {
            assert_debug_snapshot!(child_chapter);
        });
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_create_child_chapter_directly() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let logged_in_user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&logged_in_user.token);
        let auth_header = (auth_key, auth_value);

        let book = create_test_book(&request, &ctx, &auth_header).await;

        // 先创建父章节
        let parent_chapter = create_test_chapter(
            &request,
            book.id,
            &auth_header,
            "Parent Chapter",
            Some("Parent description"),
        )
        .await;

        // 直接创建子章节
        let payload = json!({
            "title": "Direct Child Chapter",
            "description": "Direct child description"
        });

        let response = request
            .post(&format!(
                "/api/books/{}/chapters/{}/children",
                book.id, parent_chapter.id
            ))
            .add_header(auth_header.0.clone(), auth_header.1.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), 200);

        let response_text = response.text();
        let child_chapter: serde_json::Value = serde_json::from_str(&response_text).unwrap();

        assert_eq!(child_chapter["title"], "Direct Child Chapter");
        assert_eq!(child_chapter["parent_id"], parent_chapter.id);

        with_settings!({filters => cleanup_chapter_response()}, {
            assert_debug_snapshot!(child_chapter);
        });
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_chapter_tree() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let logged_in_user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&logged_in_user.token);
        let auth_header = (auth_key, auth_value);

        let book = create_test_book(&request, &ctx, &auth_header).await;

        // 创建父章节
        let parent1 = create_test_chapter(&request, book.id, &auth_header, "Parent 1", None).await;

        let _parent2 = create_test_chapter(&request, book.id, &auth_header, "Parent 2", None).await;

        // 为 parent1 创建子章节
        let payload = json!({
            "title": "Child 1.1",
            "parent_id": parent1.id
        });
        let response = request
            .post(&format!("/api/books/{}/chapters", book.id))
            .add_header(auth_header.0.clone(), auth_header.1.clone())
            .json(&payload)
            .await;
        assert_eq!(response.status_code(), 200);

        // 获取章节树
        let response = request
            .get(&format!("/api/books/{}/chapters/tree", book.id))
            .add_header(auth_header.0.clone(), auth_header.1.clone())
            .await;

        assert_eq!(response.status_code(), 200);

        let response_text = response.text();
        let tree: serde_json::Value = serde_json::from_str(&response_text).unwrap();

        assert_eq!(tree.as_array().unwrap().len(), 2); // 两个顶级章节

        with_settings!({filters => cleanup_chapter_response()}, {
            assert_debug_snapshot!(tree);
        });
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_move_chapter_to_new_parent() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let logged_in_user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&logged_in_user.token);
        let auth_header = (auth_key, auth_value);

        let book = create_test_book(&request, &ctx, &auth_header).await;

        // 创建两个父章节
        let parent1 = create_test_chapter(&request, book.id, &auth_header, "Parent 1", None).await;

        let _parent2 = create_test_chapter(&request, book.id, &auth_header, "Parent 2", None).await;

        // 创建一个独立章节
        let independent_chapter =
            create_test_chapter(&request, book.id, &auth_header, "Independent Chapter", None).await;

        // 将独立章节移动到 parent1 下
        let payload = json!({
            "new_parent_id": parent1.id,
            "new_sort_order": 1
        });

        let response = request
            .post(&format!(
                "/api/books/{}/chapters/{}/move",
                book.id, independent_chapter.id
            ))
            .add_header(auth_header.0.clone(), auth_header.1.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), 200);

        let response_text = response.text();
        let moved_chapter: serde_json::Value = serde_json::from_str(&response_text).unwrap();

        assert_eq!(moved_chapter["id"], independent_chapter.id);
        assert_eq!(moved_chapter["parent_id"], parent1.id);
        assert_eq!(moved_chapter["level"], 1);

        with_settings!({filters => cleanup_chapter_response()}, {
            assert_debug_snapshot!(moved_chapter);
        });
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_flat_list_with_level() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let logged_in_user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&logged_in_user.token);
        let auth_header = (auth_key, auth_value);

        let book = create_test_book(&request, &ctx, &auth_header).await;

        // 创建父章节
        let parent =
            create_test_chapter(&request, book.id, &auth_header, "Parent Chapter", None).await;

        // 创建子章节
        let payload = json!({
            "title": "Child Chapter",
            "parent_id": parent.id
        });
        let response = request
            .post(&format!("/api/books/{}/chapters", book.id))
            .add_header(auth_header.0.clone(), auth_header.1.clone())
            .json(&payload)
            .await;
        assert_eq!(response.status_code(), 200);

        // 获取扁平列表
        let response = request
            .get(&format!("/api/books/{}/chapters/flat", book.id))
            .add_header(auth_header.0.clone(), auth_header.1.clone())
            .await;

        assert_eq!(response.status_code(), 200);

        let response_text = response.text();
        let flat_list: Vec<serde_json::Value> = serde_json::from_str(&response_text).unwrap();

        assert_eq!(flat_list.len(), 2);

        with_settings!({filters => cleanup_chapter_response()}, {
            assert_debug_snapshot!(flat_list);
        });
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_chapter_children() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let logged_in_user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&logged_in_user.token);
        let auth_header = (auth_key, auth_value);

        let book = create_test_book(&request, &ctx, &auth_header).await;

        // 创建父章节
        let parent =
            create_test_chapter(&request, book.id, &auth_header, "Parent Chapter", None).await;

        // 创建多个子章节
        for i in 1..=3 {
            let payload = json!({
                "title": format!("Child Chapter {}", i),
                "parent_id": parent.id
            });
            let response = request
                .post(&format!("/api/books/{}/chapters", book.id))
                .add_header(auth_header.0.clone(), auth_header.1.clone())
                .json(&payload)
                .await;
            assert_eq!(response.status_code(), 200);
        }

        // 获取子章节
        let response = request
            .get(&format!(
                "/api/books/{}/chapters/{}/children",
                book.id, parent.id
            ))
            .add_header(auth_header.0.clone(), auth_header.1.clone())
            .await;

        assert_eq!(response.status_code(), 200);

        let response_text = response.text();
        let children: Vec<serde_json::Value> = serde_json::from_str(&response_text).unwrap();

        assert_eq!(children.len(), 3);

        with_settings!({filters => cleanup_chapter_response()}, {
            assert_debug_snapshot!(children);
        });
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_create_deep_nested_chapters() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let logged_in_user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&logged_in_user.token);
        let auth_header = (auth_key, auth_value);

        let book = create_test_book(&request, &ctx, &auth_header).await;

        // 创建多层嵌套章节
        let level1 =
            create_test_chapter(&request, book.id, &auth_header, "Level 1 Chapter", None).await;

        let level2_payload = json!({
            "title": "Level 2 Chapter",
            "parent_id": level1.id
        });
        let response = request
            .post(&format!("/api/books/{}/chapters", book.id))
            .add_header(auth_header.0.clone(), auth_header.1.clone())
            .json(&level2_payload)
            .await;
        assert_eq!(response.status_code(), 200);
        let level2: ChapterResponse = serde_json::from_str(&response.text()).unwrap();

        let level3_payload = json!({
            "title": "Level 3 Chapter",
            "parent_id": level2.id
        });
        let response = request
            .post(&format!("/api/books/{}/chapters", book.id))
            .add_header(auth_header.0.clone(), auth_header.1.clone())
            .json(&level3_payload)
            .await;
        assert_eq!(response.status_code(), 200);
        let _level3: ChapterResponse = serde_json::from_str(&response.text()).unwrap();

        // 获取完整的章节树
        let response = request
            .get(&format!("/api/books/{}/chapters/tree", book.id))
            .add_header(auth_header.0.clone(), auth_header.1.clone())
            .await;

        assert_eq!(response.status_code(), 200);

        let response_text = response.text();
        let tree: serde_json::Value = serde_json::from_str(&response_text).unwrap();

        // 验证层级结构
        assert_eq!(tree.as_array().unwrap().len(), 1); // 一个顶级章节
        let root = &tree[0];
        assert_eq!(root["title"], "Level 1 Chapter");
        assert_eq!(root["children"].as_array().unwrap().len(), 1);

        let child = &root["children"][0];
        assert_eq!(child["title"], "Level 2 Chapter");
        assert_eq!(child["children"].as_array().unwrap().len(), 1);

        let grandchild = &child["children"][0];
        assert_eq!(grandchild["title"], "Level 3 Chapter");
        assert_eq!(grandchild["children"].as_array().unwrap().len(), 0);

        with_settings!({filters => cleanup_chapter_response()}, {
            assert_debug_snapshot!(tree);
        });
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_move_chapter_to_root() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let logged_in_user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&logged_in_user.token);
        let auth_header = (auth_key, auth_value);

        let book = create_test_book(&request, &ctx, &auth_header).await;

        // 创建父章节
        let parent =
            create_test_chapter(&request, book.id, &auth_header, "Parent Chapter", None).await;

        // 创建子章节
        let payload = json!({
            "title": "Child Chapter",
            "parent_id": parent.id
        });
        let response = request
            .post(&format!("/api/books/{}/chapters", book.id))
            .add_header(auth_header.0.clone(), auth_header.1.clone())
            .json(&payload)
            .await;
        assert_eq!(response.status_code(), 200);
        let child: ChapterResponse = serde_json::from_str(&response.text()).unwrap();

        // 将子章节移动到根级别
        let payload = json!({
            "new_parent_id": null,
            "new_sort_order": 10
        });

        let response = request
            .post(&format!(
                "/api/books/{}/chapters/{}/move",
                book.id, child.id
            ))
            .add_header(auth_header.0.clone(), auth_header.1.clone())
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), 200);

        let response_text = response.text();
        let moved_chapter: serde_json::Value = serde_json::from_str(&response_text).unwrap();

        assert_eq!(moved_chapter["id"], child.id);
        assert_eq!(moved_chapter["parent_id"], serde_json::Value::Null);
        assert_eq!(moved_chapter["level"], 0);

        with_settings!({filters => cleanup_chapter_response()}, {
            assert_debug_snapshot!(moved_chapter);
        });
    })
    .await;
}

#[tokio::test]
#[serial]
async fn cannot_create_circular_reference() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let logged_in_user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&logged_in_user.token);
        let auth_header = (auth_key, auth_value);

        let book = create_test_book(&request, &ctx, &auth_header).await;

        // 创建父章节
        let parent =
            create_test_chapter(&request, book.id, &auth_header, "Parent Chapter", None).await;

        // 创建子章节
        let payload = json!({
            "title": "Child Chapter",
            "parent_id": parent.id
        });
        let response = request
            .post(&format!("/api/books/{}/chapters", book.id))
            .add_header(auth_header.0.clone(), auth_header.1.clone())
            .json(&payload)
            .await;
        assert_eq!(response.status_code(), 200);
        let child: ChapterResponse = serde_json::from_str(&response.text()).unwrap();

        // 尝试将父章节移动到子章节下（应该失败）
        let payload = json!({
            "new_parent_id": child.id,
            "new_sort_order": 1
        });

        let response = request
            .post(&format!(
                "/api/books/{}/chapters/{}/move",
                book.id, parent.id
            ))
            .add_header(auth_header.0.clone(), auth_header.1.clone())
            .json(&payload)
            .await;

        // 应该返回错误
        assert_eq!(response.status_code(), 400);
    })
    .await;
}
