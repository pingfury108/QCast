use crate::models::users;
use loco_rs::prelude::*;
use sea_orm::ActiveValue;

pub struct ChangeUserPassword;

#[async_trait]
impl Task for ChangeUserPassword {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "change_user_password".to_string(),
            detail: "修改指定用户的密码".to_string(),
        }
    }

    async fn run(&self, app_context: &AppContext, vars: &task::Vars) -> Result<()> {
        // 从命令行参数获取邮箱和新密码
        let email = match vars.cli_arg("email") {
            Ok(email) => email,
            Err(_) => {
                println!("❌ 缺少必需参数: email");
                println!();
                println!("📖 使用方法:");
                println!("   cargo loco task change_user_password email:<邮箱> password:<新密码>");
                println!();
                println!("💡 示例:");
                println!("   cargo loco task change_user_password email:user@example.com password:NewPassword123");
                return Err(Error::Message("缺少必需参数: email".to_string()));
            }
        };

        let new_password = match vars.cli_arg("password") {
            Ok(password) => password,
            Err(_) => {
                println!("❌ 缺少必需参数: password");
                println!();
                println!("📖 使用方法:");
                println!("   cargo loco task change_user_password email:<邮箱> password:<新密码>");
                println!();
                println!("💡 示例:");
                println!("   cargo loco task change_user_password email:user@example.com password:NewPassword123");
                return Err(Error::Message("缺少必需参数: password".to_string()));
            }
        };

        // 查找用户
        let user = match users::Model::find_by_email(&app_context.db, email).await {
            Ok(user) => user,
            Err(ModelError::EntityNotFound) => {
                return Err(Error::Message(format!("用户 {} 不存在", email)));
            }
            Err(e) => {
                return Err(Error::Model(e));
            }
        };

        // 哈希新密码
        let password_hash = loco_rs::hash::hash_password(new_password)
            .map_err(|e| Error::Message(format!("密码哈希失败: {}", e)))?;

        // 更新密码
        let mut active_user: users::ActiveModel = user.into();
        active_user.password = ActiveValue::Set(password_hash);

        let updated_user = active_user
            .update(&app_context.db)
            .await
            .map_err(|e| Error::Message(format!("更新密码失败: {}", e)))?;

        println!("✅ 用户密码修改成功!");
        println!("   邮箱: {}", updated_user.email);
        println!("   名称: {}", updated_user.name);
        println!("   用户ID: {}", updated_user.id);

        Ok(())
    }
}
