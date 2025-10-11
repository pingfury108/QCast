use insta::{assert_debug_snapshot, with_settings};
use loco_rs::testing::prelude::*;
use qcast::app::App;
use qcast::models::site_settings;
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
async fn can_get_or_create_settings() {
    configure_insta!();

    let boot = boot_test::<App>().await.unwrap();
    seed::<App>(&boot.app_context).await.unwrap();

    let default_url = "http://localhost:5150";
    let settings = site_settings::Model::get_or_create(&boot.app_context.db, default_url)
        .await
        .unwrap();

    assert_eq!(settings.id, 1);
    assert_eq!(settings.site_url, default_url);

    with_settings!({
        filters => cleanup_site_settings_model()
    }, {
        assert_debug_snapshot!(settings);
    });
}

#[tokio::test]
#[serial]
async fn can_update_site_url() {
    configure_insta!();

    let boot = boot_test::<App>().await.unwrap();
    seed::<App>(&boot.app_context).await.unwrap();

    // 创建初始设置
    let _initial_settings =
        site_settings::Model::get_or_create(&boot.app_context.db, "http://localhost:5150")
            .await
            .unwrap();

    // 更新URL
    let new_url = "https://example.com";
    let updated_settings =
        site_settings::Model::update_url(&boot.app_context.db, new_url.to_string())
            .await
            .unwrap();

    assert_eq!(updated_settings.site_url, new_url);
    // 注意：由于更新太快，updated_at 可能等于 created_at，所以使用 >= 比较
    assert!(updated_settings.updated_at >= updated_settings.created_at);

    with_settings!({
        filters => cleanup_site_settings_model()
    }, {
        assert_debug_snapshot!(updated_settings);
    });
}

#[tokio::test]
#[serial]
async fn should_normalize_url_removing_trailing_slash() {
    configure_insta!();

    let boot = boot_test::<App>().await.unwrap();
    seed::<App>(&boot.app_context).await.unwrap();

    // 创建初始设置
    let _initial_settings =
        site_settings::Model::get_or_create(&boot.app_context.db, "http://localhost:5150")
            .await
            .unwrap();

    // 更新URL（带尾部斜杠）
    let url_with_slash = "https://example.com/";
    let settings =
        site_settings::Model::update_url(&boot.app_context.db, url_with_slash.to_string())
            .await
            .unwrap();

    // 验证尾部斜杠被去除
    assert_eq!(settings.site_url, "https://example.com");

    with_settings!({
        filters => cleanup_site_settings_model()
    }, {
        assert_debug_snapshot!(settings);
    });
}

#[tokio::test]
#[serial]
async fn should_reject_invalid_url_format() {
    let boot = boot_test::<App>().await.unwrap();
    seed::<App>(&boot.app_context).await.unwrap();

    // 创建初始设置
    let _initial_settings =
        site_settings::Model::get_or_create(&boot.app_context.db, "http://localhost:5150")
            .await
            .unwrap();

    // 尝试使用无效URL
    let invalid_url = "not-a-valid-url";
    let result =
        site_settings::Model::update_url(&boot.app_context.db, invalid_url.to_string()).await;

    assert!(result.is_err());
}

#[tokio::test]
#[serial]
async fn should_reject_url_without_http_scheme() {
    let boot = boot_test::<App>().await.unwrap();
    seed::<App>(&boot.app_context).await.unwrap();

    // 创建初始设置
    let _initial_settings =
        site_settings::Model::get_or_create(&boot.app_context.db, "http://localhost:5150")
            .await
            .unwrap();

    // 尝试使用没有 http/https 的URL
    let url_without_scheme = "example.com";
    let result =
        site_settings::Model::update_url(&boot.app_context.db, url_without_scheme.to_string())
            .await;

    assert!(result.is_err());
}

#[tokio::test]
#[serial]
async fn should_reject_empty_url() {
    let boot = boot_test::<App>().await.unwrap();
    seed::<App>(&boot.app_context).await.unwrap();

    // 创建初始设置
    let _initial_settings =
        site_settings::Model::get_or_create(&boot.app_context.db, "http://localhost:5150")
            .await
            .unwrap();

    // 尝试使用空URL
    let result = site_settings::Model::update_url(&boot.app_context.db, "".to_string()).await;

    assert!(result.is_err());
}

#[tokio::test]
#[serial]
async fn should_allow_both_http_and_https() {
    configure_insta!();

    let boot = boot_test::<App>().await.unwrap();
    seed::<App>(&boot.app_context).await.unwrap();

    // 创建初始设置
    let _initial_settings =
        site_settings::Model::get_or_create(&boot.app_context.db, "http://localhost:5150")
            .await
            .unwrap();

    // 测试 http
    let http_settings =
        site_settings::Model::update_url(&boot.app_context.db, "http://example.com".to_string())
            .await
            .unwrap();
    assert_eq!(http_settings.site_url, "http://example.com");

    // 测试 https
    let https_settings =
        site_settings::Model::update_url(&boot.app_context.db, "https://example.com".to_string())
            .await
            .unwrap();
    assert_eq!(https_settings.site_url, "https://example.com");

    with_settings!({
        filters => cleanup_site_settings_model()
    }, {
        assert_debug_snapshot!(https_settings);
    });
}

#[tokio::test]
#[serial]
async fn should_maintain_singleton_pattern() {
    configure_insta!();

    let boot = boot_test::<App>().await.unwrap();
    seed::<App>(&boot.app_context).await.unwrap();

    // 第一次获取或创建
    let settings1 =
        site_settings::Model::get_or_create(&boot.app_context.db, "http://localhost:5150")
            .await
            .unwrap();

    // 第二次获取或创建（应该返回已存在的记录）
    let settings2 =
        site_settings::Model::get_or_create(&boot.app_context.db, "http://different.com")
            .await
            .unwrap();

    // 应该是同一条记录
    assert_eq!(settings1.id, settings2.id);
    assert_eq!(settings1.site_url, settings2.site_url);

    with_settings!({
        filters => cleanup_site_settings_model()
    }, {
        assert_debug_snapshot!(settings2);
    });
}

fn cleanup_site_settings_model() -> Vec<(&'static str, &'static str)> {
    vec![
        (r"id: \d+,", "id: ID"),
        (
            r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(\.\d+)?\+\d{2}:\d{2}",
            "DATE",
        ),
    ]
}
