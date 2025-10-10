use loco_rs::testing::prelude::*;
use qcast::app::App;
use serial_test::serial;

use super::prepare_data::{auth_header, init_staff_login, init_superadmin_login, init_user_login};

#[tokio::test]
#[serial]
async fn can_list_users_as_admin() {
    request::<App, _, _>(|request, ctx| async move {
        // 创建超级管理员
        let admin = init_superadmin_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&admin.token);

        // 创建一些普通用户
        let _user1 = init_user_login(&request, &ctx).await;

        // 管理员列出所有用户
        let response = request
            .get("/api/admin/users")
            .add_header(auth_key, auth_value)
            .await;

        assert_eq!(response.status_code(), 200);

        let body: serde_json::Value = serde_json::from_str(&response.text()).unwrap();
        assert!(body["users"].is_array());
        assert!(body["pagination"].is_object());
    })
    .await;
}

#[tokio::test]
#[serial]
async fn cannot_list_users_as_regular_user() {
    request::<App, _, _>(|request, ctx| async move {
        // 创建普通用户
        let user = init_user_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&user.token);

        // 普通用户尝试列出所有用户
        let response = request
            .get("/api/admin/users")
            .add_header(auth_key, auth_value)
            .await;

        assert_eq!(response.status_code(), 401);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_search_users_as_admin() {
    request::<App, _, _>(|request, ctx| async move {
        // 创建超级管理员
        let admin = init_superadmin_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&admin.token);

        // 搜索用户
        let response = request
            .get("/api/admin/users?search=admin")
            .add_header(auth_key, auth_value)
            .await;

        assert_eq!(response.status_code(), 200);

        let body: serde_json::Value = serde_json::from_str(&response.text()).unwrap();
        assert!(body["users"].is_array());
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_update_user_role_as_superadmin() {
    request::<App, _, _>(|request, ctx| async move {
        // 创建超级管理员
        let admin = init_superadmin_login(&request, &ctx).await;

        // 创建普通用户
        let user = init_user_login(&request, &ctx).await;

        // 超级管理员更新用户角色
        let payload = serde_json::json!({
            "is_staff": true,
            "is_superuser": false
        });

        let (auth_key, auth_value) = auth_header(&admin.token);
        let response = request
            .put(&format!("/api/admin/users/{}/role", user.user.id))
            .add_header(auth_key, auth_value)
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), 200);

        let body: serde_json::Value = serde_json::from_str(&response.text()).unwrap();
        assert_eq!(body["is_staff"], true);
        assert_eq!(body["is_superuser"], false);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn cannot_update_user_role_as_staff() {
    request::<App, _, _>(|request, ctx| async move {
        // 创建员工
        let staff = init_staff_login(&request, &ctx).await;

        // 创建普通用户
        let user = init_user_login(&request, &ctx).await;

        // 员工尝试更新用户角色（应该失败，需要超级管理员权限）
        let payload = serde_json::json!({
            "is_staff": true,
            "is_superuser": false
        });

        let (auth_key, auth_value) = auth_header(&staff.token);
        let response = request
            .put(&format!("/api/admin/users/{}/role", user.user.id))
            .add_header(auth_key, auth_value)
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), 401);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_user_stats_as_admin() {
    request::<App, _, _>(|request, ctx| async move {
        // 创建超级管理员
        let admin = init_superadmin_login(&request, &ctx).await;
        let (auth_key, auth_value) = auth_header(&admin.token);

        // 获取统计信息
        let response = request
            .get("/api/admin/users/stats")
            .add_header(auth_key, auth_value)
            .await;

        assert_eq!(response.status_code(), 200);

        let body: serde_json::Value = serde_json::from_str(&response.text()).unwrap();
        assert!(body["total_users"].is_number());
        assert!(body["total_admins"].is_number());
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_delete_user_as_superadmin() {
    request::<App, _, _>(|request, ctx| async move {
        // 创建超级管理员
        let admin = init_superadmin_login(&request, &ctx).await;

        // 创建普通用户
        let user = init_user_login(&request, &ctx).await;

        // 超级管理员删除用户
        let (auth_key, auth_value) = auth_header(&admin.token);
        let response = request
            .delete(&format!("/api/admin/users/{}", user.user.id))
            .add_header(auth_key, auth_value)
            .await;

        assert_eq!(response.status_code(), 200);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn cannot_delete_self() {
    request::<App, _, _>(|request, ctx| async move {
        // 创建超级管理员
        let admin = init_superadmin_login(&request, &ctx).await;

        // 超级管理员尝试删除自己（应该失败）
        let (auth_key, auth_value) = auth_header(&admin.token);
        let response = request
            .delete(&format!("/api/admin/users/{}", admin.user.id))
            .add_header(auth_key, auth_value)
            .await;

        assert_eq!(response.status_code(), 400);
    })
    .await;
}
