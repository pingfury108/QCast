use axum::http::{HeaderName, HeaderValue};
use loco_rs::{app::AppContext, TestServer};
use qcast::{models::users, views::auth::LoginResponse};
use sea_orm::{ActiveModelTrait, ActiveValue::Set};

const USER_EMAIL: &str = "test@loco.com";
const USER_PASSWORD: &str = "1234";

pub struct LoggedInUser {
    pub user: users::Model,
    pub token: String,
}

pub async fn init_user_login(request: &TestServer, ctx: &AppContext) -> LoggedInUser {
    let register_payload = serde_json::json!({
        "name": "loco",
        "email": USER_EMAIL,
        "password": USER_PASSWORD
    });

    //Creating a new user
    request
        .post("/api/auth/register")
        .json(&register_payload)
        .await;
    let user = users::Model::find_by_email(&ctx.db, USER_EMAIL)
        .await
        .unwrap();

    let verify_payload = serde_json::json!({
        "token": user.email_verification_token,
    });

    request.post("/api/auth/verify").json(&verify_payload).await;

    let response = request
        .post("/api/auth/login")
        .json(&serde_json::json!({
            "email": USER_EMAIL,
            "password": USER_PASSWORD
        }))
        .await;

    let login_response: LoginResponse = serde_json::from_str(&response.text()).unwrap();

    LoggedInUser {
        user: users::Model::find_by_email(&ctx.db, USER_EMAIL)
            .await
            .unwrap(),
        token: login_response.token,
    }
}

/// 创建超级管理员用户并登录
pub async fn init_superadmin_login(request: &TestServer, ctx: &AppContext) -> LoggedInUser {
    let admin_email = "admin@loco.com";
    let admin_password = "admin1234";

    let register_payload = serde_json::json!({
        "name": "Admin",
        "email": admin_email,
        "password": admin_password
    });

    // 创建用户
    request
        .post("/api/auth/register")
        .json(&register_payload)
        .await;

    let mut user = users::Model::find_by_email(&ctx.db, admin_email)
        .await
        .unwrap();

    // 设置为超级管理员
    let mut active_user: users::ActiveModel = user.clone().into();
    active_user.is_staff = Set(true);
    active_user.is_superuser = Set(true);
    active_user.email_verified_at = Set(Some(chrono::Local::now().into()));
    user = active_user.update(&ctx.db).await.unwrap();

    // 登录
    let response = request
        .post("/api/auth/login")
        .json(&serde_json::json!({
            "email": admin_email,
            "password": admin_password
        }))
        .await;

    let login_response: LoginResponse = serde_json::from_str(&response.text()).unwrap();

    LoggedInUser {
        user,
        token: login_response.token,
    }
}

/// 创建普通员工（is_staff=true, is_superuser=false）
pub async fn init_staff_login(request: &TestServer, ctx: &AppContext) -> LoggedInUser {
    let staff_email = "staff@loco.com";
    let staff_password = "staff1234";

    let register_payload = serde_json::json!({
        "name": "Staff",
        "email": staff_email,
        "password": staff_password
    });

    // 创建用户
    request
        .post("/api/auth/register")
        .json(&register_payload)
        .await;

    let mut user = users::Model::find_by_email(&ctx.db, staff_email)
        .await
        .unwrap();

    // 设置为员工
    let mut active_user: users::ActiveModel = user.clone().into();
    active_user.is_staff = Set(true);
    active_user.is_superuser = Set(false);
    active_user.email_verified_at = Set(Some(chrono::Local::now().into()));
    user = active_user.update(&ctx.db).await.unwrap();

    // 登录
    let response = request
        .post("/api/auth/login")
        .json(&serde_json::json!({
            "email": staff_email,
            "password": staff_password
        }))
        .await;

    let login_response: LoginResponse = serde_json::from_str(&response.text()).unwrap();

    LoggedInUser {
        user,
        token: login_response.token,
    }
}

pub fn auth_header(token: &str) -> (HeaderName, HeaderValue) {
    let auth_header_value = HeaderValue::from_str(&format!("Bearer {}", &token)).unwrap();

    (HeaderName::from_static("authorization"), auth_header_value)
}

/// 创建公开的测试媒体记录
#[allow(dead_code)]
pub async fn create_public_media(
    ctx: &AppContext,
    book_id: i32,
    chapter_id: Option<i32>,
    user_id: i32,
) -> qcast::models::_entities::medias::Model {
    let media = qcast::models::_entities::medias::ActiveModel::create_new(
        "Test Media".to_string(),
        Some("Test Description".to_string()),
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

    // 设置 access_url 和公开状态
    let mut active_media: qcast::models::_entities::medias::ActiveModel = media.into();
    active_media.access_url = Set(Some(format!(
        "http://localhost:5150/public/media/{}",
        active_media.access_token.as_ref()
    )));
    active_media.is_public = Set(true);

    active_media.update(&ctx.db).await.unwrap()
}
