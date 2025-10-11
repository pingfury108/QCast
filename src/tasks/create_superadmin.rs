use crate::models::users;
use loco_rs::prelude::*;
use sea_orm::ActiveValue;

pub struct CreateSuperadmin;

#[async_trait]
impl Task for CreateSuperadmin {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "create_superadmin".to_string(),
            detail: "创建超级管理员账号".to_string(),
        }
    }

    async fn run(&self, app_context: &AppContext, vars: &task::Vars) -> Result<()> {
        // 从命令行参数获取邮箱、密码和名称
        let email = match vars.cli_arg("email") {
            Ok(email) => email,
            Err(_) => {
                println!("❌ 缺少必需参数: email");
                println!();
                println!("📖 使用方法:");
                println!("   cargo loco task create_superadmin email:<邮箱> password:<密码> [name:<名称>]");
                println!();
                println!("💡 示例:");
                println!("   cargo loco task create_superadmin email:admin@example.com password:Admin123 name:\"系统管理员\"");
                return Err(Error::Message("缺少必需参数: email".to_string()));
            }
        };

        let password = match vars.cli_arg("password") {
            Ok(password) => password,
            Err(_) => {
                println!("❌ 缺少必需参数: password");
                println!();
                println!("📖 使用方法:");
                println!("   cargo loco task create_superadmin email:<邮箱> password:<密码> [name:<名称>]");
                println!();
                println!("💡 示例:");
                println!("   cargo loco task create_superadmin email:admin@example.com password:Admin123 name:\"系统管理员\"");
                return Err(Error::Message("缺少必需参数: password".to_string()));
            }
        };

        let default_name = "Super Admin".to_string();
        let name = vars.cli_arg("name").unwrap_or(&default_name);

        // 检查用户是否已存在
        match users::Model::find_by_email(&app_context.db, email).await {
            Ok(_) => {
                return Err(Error::Message(format!("邮箱 {} 已被注册", email)));
            }
            Err(ModelError::EntityNotFound) => {
                // 用户不存在，继续创建
            }
            Err(e) => {
                return Err(Error::Model(e));
            }
        }

        // 创建超级管理员
        let password_hash = loco_rs::hash::hash_password(password)
            .map_err(|e| Error::Message(format!("密码哈希失败: {}", e)))?;

        let user = users::ActiveModel {
            email: ActiveValue::Set(email.to_string()),
            password: ActiveValue::Set(password_hash),
            name: ActiveValue::Set(name.to_string()),
            is_staff: ActiveValue::Set(true),
            is_superuser: ActiveValue::Set(true),
            email_verified_at: ActiveValue::Set(Some(chrono::Local::now().into())),
            ..Default::default()
        }
        .insert(&app_context.db)
        .await?;

        println!("✅ 超级管理员创建成功!");
        println!("   邮箱: {}", user.email);
        println!("   名称: {}", user.name);
        println!("   ID: {}", user.id);
        println!("   PID: {}", user.pid);

        Ok(())
    }
}
