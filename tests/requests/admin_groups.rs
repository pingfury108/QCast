use loco_rs::testing::prelude::*;
use qcast::app::App;
use serial_test::serial;

use super::prepare_data::{auth_header, init_staff_login, init_superadmin_login, init_user_login};

#[tokio::test]
#[serial]
async fn can_create_group_as_admin() {
    request::<App, _, _>(|request, ctx| async move {
        // 创建超级管理员
        let admin = init_superadmin_login(&request, &ctx).await;

        // 创建用户组
        let payload = serde_json::json!({
            "name": "Editors",
            "description": "Content editors group"
        });

        let (auth_key, auth_value) = auth_header(&admin.token);
        let response = request
            .post("/api/admin/groups")
            .add_header(auth_key, auth_value)
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), 200);

        let body: serde_json::Value = serde_json::from_str(&response.text()).unwrap();
        assert_eq!(body["name"], "Editors");
        assert_eq!(body["description"], "Content editors group");
    })
    .await;
}

#[tokio::test]
#[serial]
async fn cannot_create_group_as_regular_user() {
    request::<App, _, _>(|request, ctx| async move {
        // 创建普通用户
        let user = init_user_login(&request, &ctx).await;

        // 普通用户尝试创建用户组
        let payload = serde_json::json!({
            "name": "Editors",
            "description": "Content editors group"
        });

        let (auth_key, auth_value) = auth_header(&user.token);
        let response = request
            .post("/api/admin/groups")
            .add_header(auth_key, auth_value)
            .json(&payload)
            .await;

        assert_eq!(response.status_code(), 401);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_list_groups_as_admin() {
    request::<App, _, _>(|request, ctx| async move {
        // 创建超级管理员
        let admin = init_superadmin_login(&request, &ctx).await;

        // 创建用户组
        let payload = serde_json::json!({
            "name": "Editors",
            "description": "Content editors group"
        });

        let (auth_key1, auth_value1) = auth_header(&admin.token);
        request
            .post("/api/admin/groups")
            .add_header(auth_key1, auth_value1)
            .json(&payload)
            .await;

        // 列出所有用户组
        let (auth_key2, auth_value2) = auth_header(&admin.token);
        let response = request
            .get("/api/admin/groups")
            .add_header(auth_key2, auth_value2)
            .await;

        assert_eq!(response.status_code(), 200);

        let body: serde_json::Value = serde_json::from_str(&response.text()).unwrap();
        assert!(body.is_array());
        assert!(body.as_array().unwrap().len() > 0);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_group_details_as_admin() {
    request::<App, _, _>(|request, ctx| async move {
        // 创建超级管理员
        let admin = init_superadmin_login(&request, &ctx).await;

        // 创建用户组
        let payload = serde_json::json!({
            "name": "Editors",
            "description": "Content editors group"
        });

        let (auth_key1, auth_value1) = auth_header(&admin.token);
        let create_response = request
            .post("/api/admin/groups")
            .add_header(auth_key1, auth_value1)
            .json(&payload)
            .await;

        let group: serde_json::Value = serde_json::from_str(&create_response.text()).unwrap();
        let group_id = group["id"].as_i64().unwrap();

        // 获取用户组详情
        let (auth_key2, auth_value2) = auth_header(&admin.token);
        let response = request
            .get(&format!("/api/admin/groups/{}", group_id))
            .add_header(auth_key2, auth_value2)
            .await;

        assert_eq!(response.status_code(), 200);

        let body: serde_json::Value = serde_json::from_str(&response.text()).unwrap();
        assert!(body["group"].is_object());
        assert!(body["members"].is_array());
        assert_eq!(body["member_count"], 0);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_add_member_to_group_as_admin() {
    request::<App, _, _>(|request, ctx| async move {
        // 创建超级管理员
        let admin = init_superadmin_login(&request, &ctx).await;

        // 创建普通用户
        let user = init_user_login(&request, &ctx).await;

        // 创建用户组
        let group_payload = serde_json::json!({
            "name": "Editors",
            "description": "Content editors group"
        });

        let (auth_key1, auth_value1) = auth_header(&admin.token);
        let create_response = request
            .post("/api/admin/groups")
            .add_header(auth_key1, auth_value1)
            .json(&group_payload)
            .await;

        let group: serde_json::Value = serde_json::from_str(&create_response.text()).unwrap();
        let group_id = group["id"].as_i64().unwrap();

        // 添加用户到组
        let member_payload = serde_json::json!({
            "user_id": user.user.id
        });

        let (auth_key2, auth_value2) = auth_header(&admin.token);
        let response = request
            .post(&format!("/api/admin/groups/{}/members", group_id))
            .add_header(auth_key2, auth_value2)
            .json(&member_payload)
            .await;

        assert_eq!(response.status_code(), 200);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn cannot_add_duplicate_member_to_group() {
    request::<App, _, _>(|request, ctx| async move {
        // 创建超级管理员
        let admin = init_superadmin_login(&request, &ctx).await;

        // 创建普通用户
        let user = init_user_login(&request, &ctx).await;

        // 创建用户组
        let group_payload = serde_json::json!({
            "name": "Editors",
            "description": "Content editors group"
        });

        let (auth_key1, auth_value1) = auth_header(&admin.token);
        let create_response = request
            .post("/api/admin/groups")
            .add_header(auth_key1, auth_value1)
            .json(&group_payload)
            .await;

        let group: serde_json::Value = serde_json::from_str(&create_response.text()).unwrap();
        let group_id = group["id"].as_i64().unwrap();

        // 添加用户到组
        let member_payload = serde_json::json!({
            "user_id": user.user.id
        });

        let (auth_key2, auth_value2) = auth_header(&admin.token);
        request
            .post(&format!("/api/admin/groups/{}/members", group_id))
            .add_header(auth_key2, auth_value2)
            .json(&member_payload)
            .await;

        // 再次添加同一个用户（应该失败）
        let (auth_key3, auth_value3) = auth_header(&admin.token);
        let response = request
            .post(&format!("/api/admin/groups/{}/members", group_id))
            .add_header(auth_key3, auth_value3)
            .json(&member_payload)
            .await;

        assert_eq!(response.status_code(), 400);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_remove_member_from_group_as_admin() {
    request::<App, _, _>(|request, ctx| async move {
        // 创建超级管理员
        let admin = init_superadmin_login(&request, &ctx).await;

        // 创建普通用户
        let user = init_user_login(&request, &ctx).await;

        // 创建用户组
        let group_payload = serde_json::json!({
            "name": "Editors",
            "description": "Content editors group"
        });

        let (auth_key1, auth_value1) = auth_header(&admin.token);
        let create_response = request
            .post("/api/admin/groups")
            .add_header(auth_key1, auth_value1)
            .json(&group_payload)
            .await;

        let group: serde_json::Value = serde_json::from_str(&create_response.text()).unwrap();
        let group_id = group["id"].as_i64().unwrap();

        // 添加用户到组
        let member_payload = serde_json::json!({
            "user_id": user.user.id
        });

        let (auth_key2, auth_value2) = auth_header(&admin.token);
        request
            .post(&format!("/api/admin/groups/{}/members", group_id))
            .add_header(auth_key2, auth_value2)
            .json(&member_payload)
            .await;

        // 从组中移除用户
        let (auth_key3, auth_value3) = auth_header(&admin.token);
        let response = request
            .delete(&format!(
                "/api/admin/groups/{}/members/{}",
                group_id, user.user.id
            ))
            .add_header(auth_key3, auth_value3)
            .await;

        assert_eq!(response.status_code(), 200);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_delete_group_as_superadmin() {
    request::<App, _, _>(|request, ctx| async move {
        // 创建超级管理员
        let admin = init_superadmin_login(&request, &ctx).await;

        // 创建用户组
        let group_payload = serde_json::json!({
            "name": "Editors",
            "description": "Content editors group"
        });

        let (auth_key1, auth_value1) = auth_header(&admin.token);
        let create_response = request
            .post("/api/admin/groups")
            .add_header(auth_key1, auth_value1)
            .json(&group_payload)
            .await;

        let group: serde_json::Value = serde_json::from_str(&create_response.text()).unwrap();
        let group_id = group["id"].as_i64().unwrap();

        // 删除用户组
        let (auth_key2, auth_value2) = auth_header(&admin.token);
        let response = request
            .delete(&format!("/api/admin/groups/{}", group_id))
            .add_header(auth_key2, auth_value2)
            .await;

        assert_eq!(response.status_code(), 200);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn cannot_delete_group_as_staff() {
    request::<App, _, _>(|request, ctx| async move {
        // 创建超级管理员
        let admin = init_superadmin_login(&request, &ctx).await;

        // 创建员工
        let staff = init_staff_login(&request, &ctx).await;

        // 创建用户组
        let group_payload = serde_json::json!({
            "name": "Editors",
            "description": "Content editors group"
        });

        let (auth_key1, auth_value1) = auth_header(&admin.token);
        let create_response = request
            .post("/api/admin/groups")
            .add_header(auth_key1, auth_value1)
            .json(&group_payload)
            .await;

        let group: serde_json::Value = serde_json::from_str(&create_response.text()).unwrap();
        let group_id = group["id"].as_i64().unwrap();

        // 员工尝试删除用户组（应该失败，需要超级管理员权限）
        let (auth_key2, auth_value2) = auth_header(&staff.token);
        let response = request
            .delete(&format!("/api/admin/groups/{}", group_id))
            .add_header(auth_key2, auth_value2)
            .await;

        assert_eq!(response.status_code(), 401);
    })
    .await;
}
