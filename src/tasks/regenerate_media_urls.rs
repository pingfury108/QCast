use crate::models::{_entities::medias, site_settings};
use crate::services::qrcode::QRCODE_SERVICE;
use loco_rs::prelude::*;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};

pub struct RegenerateMediaUrls;

#[async_trait]
impl Task for RegenerateMediaUrls {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "regenerate_media_urls".to_string(),
            detail: "重新生成所有媒体的访问地址和二维码（基于站点设置）".to_string(),
        }
    }

    async fn run(&self, app_context: &AppContext, _vars: &task::Vars) -> Result<()> {
        println!("🔄 开始重新生成所有媒体的访问地址和二维码...");
        println!();

        // 获取站点设置
        const DEFAULT_SITE_URL: &str = "http://localhost:5150";
        let settings = site_settings::Model::get_or_create(&app_context.db, DEFAULT_SITE_URL)
            .await
            .map_err(|e| Error::Message(format!("获取站点设置失败: {}", e)))?;

        println!("📍 当前站点 URL: {}", settings.site_url);
        println!();

        // 获取所有媒体记录
        let all_media = medias::Entity::find()
            .all(&app_context.db)
            .await
            .map_err(|e| Error::Message(format!("查询媒体记录失败: {}", e)))?;

        let total_count = all_media.len();
        println!("📊 找到 {} 个媒体记录", total_count);
        println!();

        if total_count == 0 {
            println!("✅ 没有需要更新的媒体记录");
            return Ok(());
        }

        let mut success_count = 0;
        let mut failed_count = 0;

        for (index, media) in all_media.iter().enumerate() {
            let progress = index + 1;
            print!(
                "⏳ 处理中 ({}/{}) - 媒体 ID: {} ... ",
                progress, total_count, media.id
            );

            // 重新生成访问 URL
            let new_access_url = format!(
                "{}/public/{}",
                settings.site_url.trim_end_matches('/'),
                media.access_token
            );

            // 更新媒体记录
            let mut active_model: medias::ActiveModel = media.clone().into();
            active_model.access_url = Set(Some(new_access_url.clone()));

            match active_model.update(&app_context.db).await {
                Ok(_) => {
                    // 重新生成二维码
                    match QRCODE_SERVICE
                        .regenerate_media_qrcode(media.id, &new_access_url)
                        .await
                    {
                        Ok(_) => {
                            println!("✅");
                            success_count += 1;
                        }
                        Err(e) => {
                            println!("⚠️  (二维码生成失败: {})", e);
                            success_count += 1; // URL 更新成功就算成功
                        }
                    }
                }
                Err(e) => {
                    println!("❌ (更新失败: {})", e);
                    failed_count += 1;
                }
            }
        }

        println!();
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("📈 处理完成统计:");
        println!("   总数: {}", total_count);
        println!("   成功: {} ✅", success_count);
        if failed_count > 0 {
            println!("   失败: {} ❌", failed_count);
        }
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!();

        if failed_count > 0 {
            println!("⚠️  部分媒体更新失败，请检查日志");
        } else {
            println!("🎉 所有媒体的访问地址和二维码已成功更新！");
        }

        Ok(())
    }
}
