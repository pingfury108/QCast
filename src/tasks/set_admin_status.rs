use crate::models::users;
use loco_rs::prelude::*;

pub struct SetAdminStatus;

#[async_trait]
impl Task for SetAdminStatus {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "set_admin_status".to_string(),
            detail: "设置用户的管理员权限".to_string(),
        }
    }

    async fn run(&self, app_context: &AppContext, vars: &task::Vars) -> Result<()> {
        // 从命令行参数获取用户邮箱和权限设置
        let email = match vars.cli_arg("email") {
            Ok(email) => email,
            Err(_) => {
                println!("❌ 缺少必需参数: email");
                println!();
                println!("📖 使用方法:");
                println!("   cargo loco task set_admin_status email:<邮箱> is_staff:<true|false> is_superuser:<true|false>");
                println!();
                println!("💡 示例:");
                println!("   # 设置为普通管理员");
                println!("   cargo loco task set_admin_status email:user@example.com is_staff:true is_superuser:false");
                println!();
                println!("   # 设置为超级管理员");
                println!("   cargo loco task set_admin_status email:user@example.com is_staff:true is_superuser:true");
                println!();
                println!("   # 取消管理员权限");
                println!("   cargo loco task set_admin_status email:user@example.com is_staff:false is_superuser:false");
                return Err(Error::Message("缺少必需参数: email".to_string()));
            }
        };

        let is_staff = match vars.cli_arg("is_staff") {
            Ok(val) => val
                .parse::<bool>()
                .map_err(|_| Error::Message("is_staff 必须是 true 或 false".to_string()))?,
            Err(_) => {
                println!("❌ 缺少必需参数: is_staff");
                println!();
                println!("📖 使用方法:");
                println!("   cargo loco task set_admin_status email:<邮箱> is_staff:<true|false> is_superuser:<true|false>");
                return Err(Error::Message("缺少必需参数: is_staff".to_string()));
            }
        };

        let is_superuser = match vars.cli_arg("is_superuser") {
            Ok(val) => val
                .parse::<bool>()
                .map_err(|_| Error::Message("is_superuser 必须是 true 或 false".to_string()))?,
            Err(_) => {
                println!("❌ 缺少必需参数: is_superuser");
                println!();
                println!("📖 使用方法:");
                println!("   cargo loco task set_admin_status email:<邮箱> is_staff:<true|false> is_superuser:<true|false>");
                return Err(Error::Message("缺少必需参数: is_superuser".to_string()));
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

        // 更新管理员权限
        let updated_user =
            users::Model::update_admin_status(&app_context.db, user.id, is_staff, is_superuser)
                .await
                .map_err(|e| Error::Message(format!("更新管理员状态失败: {}", e)))?;

        let staff_status = if updated_user.is_staff { "是" } else { "否" };
        let super_status = if updated_user.is_superuser {
            "是"
        } else {
            "否"
        };

        println!("✅ 用户管理员状态更新成功!");
        println!("   邮箱: {}", updated_user.email);
        println!("   名称: {}", updated_user.name);
        println!("   员工权限: {}", staff_status);
        println!("   超级管理员: {}", super_status);

        Ok(())
    }
}
