use axum::http::{HeaderName, HeaderValue};
use loco_rs::{app::AppContext, TestServer};
use qcast::{models::users, views::auth::LoginResponse};
use sea_orm::{Set, ActiveModelTrait};

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
