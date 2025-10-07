use insta::{assert_debug_snapshot, with_settings};
use loco_rs::testing::prelude::*;
use qcast::app::App;
use serial_test::serial;
use serde_json::json;

use super::prepare_data::{auth_header, init_user_login};

macro_rules! configure_insta {
    ($($expr:expr),*) => {
        let mut settings = insta::Settings::clone_current();
        settings.set_prepend_module_to_snapshot(false);
        settings.set_snapshot_suffix("books_request");
        let _guard = settings.bind_to_scope();
    };
}

fn cleanup_book_response() -> Vec<(&'static str, &'static str)> {
    vec![
        (r#""id":\s*\d+"#, r#""id": ID"#),
        (r#""user_id":\s*\d+"#, r#""user_id": USER_ID"#),
        (r#""parent_id":\s*\d+"#, r#""parent_id": PARENT_ID"#),
        (r#""sort_order":\s*\d+"#, r#""sort_order": ORDER"#),
        (r#""created_at":\s*"[^"]+""#, r#""created_at": "DATE""#),
        (r#""updated_at":\s*"[^"]+""#, r#""updated_at": "DATE""#),
    ]
}

#[tokio::test]
#[serial]
async fn can_create_book() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let logged_in_user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&logged_in_user.token);

        let payload = json!({
            "title": "我的播客系列",
            "description": "这是一个测试播客",
            "is_public": true
        });

        let response = request
            .post("/api/books")
            .add_header(auth_key, auth_value)
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), 200);

        with_settings!({
            filters => cleanup_book_response()
        }, {
            assert_debug_snapshot!(response.text());
        });
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_books() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let logged_in_user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&logged_in_user.token);

        // 创建几本书
        let book1 = json!({
            "title": "Book 1",
            "description": "Description 1"
        });

        let book2 = json!({
            "title": "Book 2",
            "description": "Description 2"
        });

        request
            .post("/api/books")
            .add_header(auth_key.clone(), auth_value.clone())
            .json(&book1)
            .await;

        request
            .post("/api/books")
            .add_header(auth_key.clone(), auth_value.clone())
            .json(&book2)
            .await;

        // 获取书籍列表
        let response = request
            .get("/api/books")
            .add_header(auth_key, auth_value)
            .await;

        assert_eq!(response.status_code(), 200);

        with_settings!({
            filters => cleanup_book_response()
        }, {
            assert_debug_snapshot!(response.text());
        });
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_book_by_id() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let logged_in_user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&logged_in_user.token);

        // 创建一本书
        let payload = json!({
            "title": "Test Book",
            "description": "Test Description"
        });

        let create_response = request
            .post("/api/books")
            .add_header(auth_key.clone(), auth_value.clone())
            .json(&payload)
            .await;

        let created_book: serde_json::Value = {
            let response_text = create_response.text();
            serde_json::from_str(&response_text).unwrap()
        };
        let book_id = created_book["id"].as_i64().unwrap();

        // 获取书籍详情
        let response = request
            .get(&format!("/api/books/{}", book_id))
            .add_header(auth_key, auth_value)
            .await;

        assert_eq!(response.status_code(), 200);

        with_settings!({
            filters => cleanup_book_response()
        }, {
            assert_debug_snapshot!(response.text());
        });
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_update_book() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let logged_in_user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&logged_in_user.token);

        // 创建一本书
        let create_payload = json!({
            "title": "Original Title",
            "description": "Original Description"
        });

        let create_response = request
            .post("/api/books")
            .add_header(auth_key.clone(), auth_value.clone())
            .json(&create_payload)
            .await;

        let created_book: serde_json::Value = {
            let response_text = create_response.text();
            serde_json::from_str(&response_text).unwrap()
        };
        let book_id = created_book["id"].as_i64().unwrap();

        // 更新书籍
        let update_payload = json!({
            "title": "Updated Title",
            "description": "Updated Description"
        });

        let response = request
            .put(&format!("/api/books/{}", book_id))
            .add_header(auth_key, auth_value)
            .json(&update_payload)
            .await;

        assert_eq!(response.status_code(), 200);

        with_settings!({
            filters => cleanup_book_response()
        }, {
            assert_debug_snapshot!(response.text());
        });
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_delete_book() {
    request::<App, _, _>(|request, ctx| async move {
        let logged_in_user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&logged_in_user.token);

        // 创建一本书
        let payload = json!({
            "title": "Book to Delete",
            "description": "Will be deleted"
        });

        let create_response = request
            .post("/api/books")
            .add_header(auth_key.clone(), auth_value.clone())
            .json(&payload)
            .await;

        let response_text = create_response.text();
        let created_book: serde_json::Value =
            serde_json::from_str(&response_text).unwrap();
        let book_id = created_book["id"].as_i64().unwrap();

        // 删除书籍
        let response = request
            .delete(&format!("/api/books/{}", book_id))
            .add_header(auth_key.clone(), auth_value.clone())
            .await;

        assert_eq!(response.status_code(), 200);

        // 验证书籍已被删除
        let get_response = request
            .get(&format!("/api/books/{}", book_id))
            .add_header(auth_key, auth_value)
            .await;

        assert_eq!(get_response.status_code(), 404);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_search_books() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let logged_in_user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&logged_in_user.token);

        // 创建多本书
        let books = vec![
            json!({"title": "Rust 编程", "description": "学习 Rust"}),
            json!({"title": "Python 基础", "description": "Python 入门"}),
            json!({"title": "Rust 高级", "description": "深入 Rust"}),
        ];

        for book in books {
            request
                .post("/api/books")
                .add_header(auth_key.clone(), auth_value.clone())
                .json(&book)
                .await;
        }

        // 搜索包含 "Rust" 的书籍
        let response = request
            .get("/api/books/search?q=Rust")
            .add_header(auth_key, auth_value)
            .await;

        assert_eq!(response.status_code(), 200);

        with_settings!({
            filters => cleanup_book_response()
        }, {
            assert_debug_snapshot!(response.text());
        });
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_book_tree() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let logged_in_user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&logged_in_user.token);

        // 创建根书籍
        let root_payload = json!({
            "title": "Root Book",
            "description": "Root Level"
        });

        let root_response = request
            .post("/api/books")
            .add_header(auth_key.clone(), auth_value.clone())
            .json(&root_payload)
            .await;

        let root_book: serde_json::Value = {
            let response_text = root_response.text();
            serde_json::from_str(&response_text).unwrap()
        };
        let root_id = root_book["id"].as_i64().unwrap();

        // 创建子书籍
        let child_payload = json!({
            "title": "Child Book",
            "description": "Child Level",
            "parent_id": root_id
        });

        request
            .post("/api/books")
            .add_header(auth_key.clone(), auth_value.clone())
            .json(&child_payload)
            .await;

        // 获取树形结构
        let response = request
            .get(&format!("/api/books/{}/tree", root_id))
            .add_header(auth_key, auth_value)
            .await;

        assert_eq!(response.status_code(), 200);

        with_settings!({
            filters => cleanup_book_response()
        }, {
            assert_debug_snapshot!(response.text());
        });
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_reorder_book() {
    configure_insta!();

    request::<App, _, _>(|request, ctx| async move {
        let logged_in_user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&logged_in_user.token);

        // 创建一本书
        let payload = json!({
            "title": "Book to Reorder",
            "sort_order": 1
        });

        let create_response = request
            .post("/api/books")
            .add_header(auth_key.clone(), auth_value.clone())
            .json(&payload)
            .await;

        let created_book: serde_json::Value = {
            let response_text = create_response.text();
            serde_json::from_str(&response_text).unwrap()
        };
        let book_id = created_book["id"].as_i64().unwrap();

        // 调整顺序
        let reorder_payload = json!({
            "sort_order": 10
        });

        let response = request
            .post(&format!("/api/books/{}/reorder", book_id))
            .add_header(auth_key, auth_value)
            .json(&reorder_payload)
            .await;

        assert_eq!(response.status_code(), 200);

        with_settings!({
            filters => cleanup_book_response()
        }, {
            assert_debug_snapshot!(response.text());
        });
    })
    .await;
}

#[tokio::test]
#[serial]
async fn cannot_access_other_user_book() {
    request::<App, _, _>(|request, ctx| async move {
        let logged_in_user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&logged_in_user.token);

        // 创建一本书
        let payload = json!({
            "title": "User 1 Book"
        });

        let create_response = request
            .post("/api/books")
            .add_header(auth_key, auth_value)
            .json(&payload)
            .await;

        let created_book: serde_json::Value = {
            let response_text = create_response.text();
            serde_json::from_str(&response_text).unwrap()
        };
        let book_id = created_book["id"].as_i64().unwrap();

        // 创建第二个用户
        let user2_payload = json!({
            "name": "user2",
            "email": "user2@test.com",
            "password": "password"
        });

        request
            .post("/api/auth/register")
            .json(&user2_payload)
            .await;

        let user2_login = request
            .post("/api/auth/login")
            .json(&json!({
                "email": "user2@test.com",
                "password": "password"
            }))
            .await;

        let user2_response: serde_json::Value = {
            let response_text = user2_login.text();
            serde_json::from_str(&response_text).unwrap()
        };
        let user2_token = user2_response["token"].as_str().unwrap();
        let (auth_key2, auth_value2) = auth_header(user2_token);

        // 用户2尝试访问用户1的书籍
        let response = request
            .get(&format!("/api/books/{}", book_id))
            .add_header(auth_key2, auth_value2)
            .await;

        assert_eq!(response.status_code(), 404);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn requires_authentication() {
    request::<App, _, _>(|request, _ctx| async move {
        // 尝试不带认证访问
        let response = request.get("/api/books").await;

        // 应该返回 401 Unauthorized
        assert_eq!(response.status_code(), 401);
    })
    .await;
}
