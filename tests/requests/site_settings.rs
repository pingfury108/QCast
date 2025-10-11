use loco_rs::testing::prelude::*;
use qcast::app::App;
use serial_test::serial;

use super::prepare_data::{auth_header, init_staff_login, init_superadmin_login, init_user_login};

#[tokio::test]
#[serial]
async fn can_get_settings_as_admin() {
    request::<App, _, _>(|request, ctx| async move {
        // 创建管理员
        let admin = init_superadmin_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&admin.token);

        // 获取站点设置
        let response = request
            .get("/api/admin/site-settings")
            .add_header(auth_key, auth_value)
            .await;

        assert_eq!(response.status_code(), 200);

        let body: serde_json::Value = serde_json::from_str(&response.text()).unwrap();
        assert!(body["id"].is_number());
        assert!(body["site_url"].is_string());
        assert!(body["created_at"].is_string());
        assert!(body["updated_at"].is_string());
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_settings_as_staff() {
    request::<App, _, _>(|request, ctx| async move {
        // 创建员工
        let staff = init_staff_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&staff.token);

        // 获取站点设置
        let response = request
            .get("/api/admin/site-settings")
            .add_header(auth_key, auth_value)
            .await;

        assert_eq!(response.status_code(), 200);

        let body: serde_json::Value = serde_json::from_str(&response.text()).unwrap();
        assert!(body["site_url"].is_string());
    })
    .await;
}

#[tokio::test]
#[serial]
async fn cannot_get_settings_as_regular_user() {
    request::<App, _, _>(|request, ctx| async move {
        // 创建普通用户
        let user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&user.token);

        // 普通用户尝试获取站点设置
        let response = request
            .get("/api/admin/site-settings")
            .add_header(auth_key, auth_value)
            .await;

        assert_eq!(response.status_code(), 401);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn cannot_get_settings_without_auth() {
    request::<App, _, _>(|request, _ctx| async move {
        // 未认证用户尝试获取站点设置
        let response = request.get("/api/admin/site-settings").await;

        assert_eq!(response.status_code(), 401);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_update_settings_as_admin() {
    request::<App, _, _>(|request, ctx| async move {
        // 创建管理员
        let admin = init_superadmin_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&admin.token);

        // 更新站点URL
        let payload = serde_json::json!({
            "site_url": "https://example.com"
        });

        let response = request
            .put("/api/admin/site-settings")
            .add_header(auth_key, auth_value)
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), 200);

        let body: serde_json::Value = serde_json::from_str(&response.text()).unwrap();
        assert_eq!(body["site_url"], "https://example.com");
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_update_settings_as_staff() {
    request::<App, _, _>(|request, ctx| async move {
        // 创建员工
        let staff = init_staff_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&staff.token);

        // 更新站点URL
        let payload = serde_json::json!({
            "site_url": "https://staff-example.com"
        });

        let response = request
            .put("/api/admin/site-settings")
            .add_header(auth_key, auth_value)
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), 200);

        let body: serde_json::Value = serde_json::from_str(&response.text()).unwrap();
        assert_eq!(body["site_url"], "https://staff-example.com");
    })
    .await;
}

#[tokio::test]
#[serial]
async fn cannot_update_settings_as_regular_user() {
    request::<App, _, _>(|request, ctx| async move {
        // 创建普通用户
        let user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&user.token);

        // 普通用户尝试更新站点设置
        let payload = serde_json::json!({
            "site_url": "https://hacker.com"
        });

        let response = request
            .put("/api/admin/site-settings")
            .add_header(auth_key, auth_value)
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), 401);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn should_normalize_url_removing_trailing_slash() {
    request::<App, _, _>(|request, ctx| async move {
        // 创建管理员
        let admin = init_superadmin_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&admin.token);

        // 更新站点URL（带尾部斜杠）
        let payload = serde_json::json!({
            "site_url": "https://example.com/"
        });

        let response = request
            .put("/api/admin/site-settings")
            .add_header(auth_key, auth_value)
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), 200);

        let body: serde_json::Value = serde_json::from_str(&response.text()).unwrap();
        // 验证尾部斜杠被去除
        assert_eq!(body["site_url"], "https://example.com");
    })
    .await;
}

#[tokio::test]
#[serial]
async fn should_reject_invalid_url_format() {
    request::<App, _, _>(|request, ctx| async move {
        // 创建管理员
        let admin = init_superadmin_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&admin.token);

        // 尝试使用无效URL
        let payload = serde_json::json!({
            "site_url": "not-a-valid-url"
        });

        let response = request
            .put("/api/admin/site-settings")
            .add_header(auth_key, auth_value)
            .json(&payload)
            .await;

        // 应该返回错误
        assert_ne!(response.status_code(), 200);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn should_reject_url_without_http_scheme() {
    request::<App, _, _>(|request, ctx| async move {
        // 创建管理员
        let admin = init_superadmin_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&admin.token);

        // 尝试使用没有 http/https 的URL
        let payload = serde_json::json!({
            "site_url": "example.com"
        });

        let response = request
            .put("/api/admin/site-settings")
            .add_header(auth_key, auth_value)
            .json(&payload)
            .await;

        // 应该返回错误
        assert_ne!(response.status_code(), 200);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn should_reject_empty_url() {
    request::<App, _, _>(|request, ctx| async move {
        // 创建管理员
        let admin = init_superadmin_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&admin.token);

        // 尝试使用空URL
        let payload = serde_json::json!({
            "site_url": ""
        });

        let response = request
            .put("/api/admin/site-settings")
            .add_header(auth_key, auth_value)
            .json(&payload)
            .await;

        // 应该返回错误
        assert_ne!(response.status_code(), 200);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn should_allow_both_http_and_https() {
    request::<App, _, _>(|request, ctx| async move {
        // 创建管理员
        let admin = init_superadmin_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&admin.token);

        // 测试 http
        let http_payload = serde_json::json!({
            "site_url": "http://example.com"
        });

        let http_response = request
            .put("/api/admin/site-settings")
            .add_header(auth_key.clone(), auth_value.clone())
            .json(&http_payload)
            .await;

        assert_eq!(http_response.status_code(), 200);
        let http_body: serde_json::Value = serde_json::from_str(&http_response.text()).unwrap();
        assert_eq!(http_body["site_url"], "http://example.com");

        // 测试 https
        let https_payload = serde_json::json!({
            "site_url": "https://example.com"
        });

        let https_response = request
            .put("/api/admin/site-settings")
            .add_header(auth_key, auth_value)
            .json(&https_payload)
            .await;

        assert_eq!(https_response.status_code(), 200);
        let https_body: serde_json::Value = serde_json::from_str(&https_response.text()).unwrap();
        assert_eq!(https_body["site_url"], "https://example.com");
    })
    .await;
}
