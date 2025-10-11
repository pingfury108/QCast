use crate::models::users;
use loco_rs::prelude::*;

pub struct ListAdmins;

#[async_trait]
impl Task for ListAdmins {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "list_admins".to_string(),
            detail: "列出所有管理员用户".to_string(),
        }
    }

    async fn run(&self, app_context: &AppContext, _vars: &task::Vars) -> Result<()> {
        // 获取所有用户并筛选管理员
        let all_users = match users::Model::list_all(&app_context.db, 1, 1000).await {
            Ok((users, _)) => users,
            Err(e) => {
                return Err(Error::Message(format!("获取用户列表失败: {}", e)));
            }
        };

        // 筛选出管理员用户
        let admins: Vec<_> = all_users
            .into_iter()
            .filter(|user| user.is_admin())
            .collect();

        if admins.is_empty() {
            println!("📋 暂无管理员用户");
            return Ok(());
        }

        println!("📋 管理员用户列表 (共 {} 个):", admins.len());
        println!("{}", "-".repeat(80));

        for (index, user) in admins.iter().enumerate() {
            let user_type = if user.is_superuser {
                "超级管理员"
            } else {
                "普通管理员"
            };

            println!("{}. {}", index + 1, user.name);
            println!("   📧 邮箱: {}", user.email);
            println!("   👤 类型: {}", user_type);
            println!("   🆔 ID: {}", user.id);
            println!("   🔑 PID: {}", user.pid);
            println!(
                "   📅 注册时间: {}",
                user.created_at.format("%Y-%m-%d %H:%M:%S")
            );

            if index < admins.len() - 1 {
                println!();
            }
        }

        println!("{}", "-".repeat(80));
        println!("✨ 总计: {} 个管理员", admins.len());

        Ok(())
    }
}
