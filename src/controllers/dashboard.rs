#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use sea_orm::{PaginatorTrait, QueryOrder, QuerySelect};

use crate::models::_entities::{books, chapters, medias};
use crate::models::users;

#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardStats {
    pub total_books: i64,
    pub total_medias: i64,
    pub total_chapters: i64,
    pub total_plays: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopMedia {
    pub id: i32,
    pub title: String,
    pub book_title: String,
    pub chapter_title: Option<String>,
    pub play_count: i32,
    pub book_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecentMedia {
    pub id: i32,
    pub title: String,
    pub book_title: String,
    pub chapter_title: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub book_id: i32,
}

/// 获取仪表盘统计数据
#[debug_handler]
pub async fn stats(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    // 统计书籍数量
    let total_books = books::Entity::find()
        .filter(books::Column::UserId.eq(user.id))
        .count(&ctx.db)
        .await?;

    // 统计媒体数量
    let total_medias = medias::Entity::find()
        .filter(medias::Column::UserId.eq(user.id))
        .count(&ctx.db)
        .await?;

    // 统计章节数量
    let total_chapters = chapters::Entity::find()
        .inner_join(books::Entity)
        .filter(books::Column::UserId.eq(user.id))
        .count(&ctx.db)
        .await?;

    // 统计总播放次数
    let medias_list = medias::Entity::find()
        .filter(medias::Column::UserId.eq(user.id))
        .all(&ctx.db)
        .await?;

    let total_plays: i64 = medias_list.iter().map(|m| m.play_count as i64).sum();

    let stats = DashboardStats {
        total_books: total_books as i64,
        total_medias: total_medias as i64,
        total_chapters: total_chapters as i64,
        total_plays,
    };

    format::json(stats)
}

/// 获取播放排行榜
#[debug_handler]
pub async fn top_medias(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    // 获取用户的所有媒体，按播放次数排序
    let medias_list = medias::Entity::find()
        .filter(medias::Column::UserId.eq(user.id))
        .order_by_desc(medias::Column::PlayCount)
        .limit(10)
        .all(&ctx.db)
        .await?;

    let mut top_list = Vec::new();
    for media in medias_list {
        // 获取书籍信息
        let book = books::Entity::find_by_id(media.book_id)
            .one(&ctx.db)
            .await?
            .ok_or_else(|| Error::NotFound)?;

        // 获取章节信息（如果有）
        let chapter_title = if let Some(chapter_id) = media.chapter_id {
            chapters::Entity::find_by_id(chapter_id)
                .one(&ctx.db)
                .await?
                .map(|c| c.title)
        } else {
            None
        };

        top_list.push(TopMedia {
            id: media.id,
            title: media.title,
            book_title: book.title,
            chapter_title,
            play_count: media.play_count,
            book_id: media.book_id,
        });
    }

    format::json(top_list)
}

/// 获取最近上传的媒体
#[debug_handler]
pub async fn recent_medias(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    // 获取用户最近上传的媒体
    let medias_list = medias::Entity::find()
        .filter(medias::Column::UserId.eq(user.id))
        .order_by_desc(medias::Column::CreatedAt)
        .limit(5)
        .all(&ctx.db)
        .await?;

    let mut recent_list = Vec::new();
    for media in medias_list {
        // 获取书籍信息
        let book = books::Entity::find_by_id(media.book_id)
            .one(&ctx.db)
            .await?
            .ok_or_else(|| Error::NotFound)?;

        // 获取章节信息（如果有）
        let chapter_title = if let Some(chapter_id) = media.chapter_id {
            chapters::Entity::find_by_id(chapter_id)
                .one(&ctx.db)
                .await?
                .map(|c| c.title)
        } else {
            None
        };

        recent_list.push(RecentMedia {
            id: media.id,
            title: media.title,
            book_title: book.title,
            chapter_title,
            created_at: media.created_at.into(),
            book_id: media.book_id,
        });
    }

    format::json(recent_list)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/dashboard")
        .add("/stats", get(stats))
        .add("/top-medias", get(top_medias))
        .add("/recent-medias", get(recent_medias))
}
