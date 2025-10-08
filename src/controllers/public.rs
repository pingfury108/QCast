#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use axum::extract::{Path, State};
use axum::http::{header, StatusCode};
use axum::response::Response;
use axum::body::Body;
use loco_rs::prelude::*;
use tokio::fs::File;
use tokio::io::{AsyncSeekExt, SeekFrom};

use crate::models::_entities::medias::{Entity, Column};
use crate::views::medias::{PublicMediaResponse};

/// 通过 access_token 公开访问媒体文件
#[debug_handler]
pub async fn get_media(
    Path(access_token): Path<String>,
    State(ctx): State<AppContext>,
    headers: header::HeaderMap,
) -> Result<Response> {
    // 查找媒体记录
    let media = Entity::find()
        .filter(Column::AccessToken.eq(&access_token))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::Message("媒体不存在或访问令牌无效".to_string()))?;

    // 检查媒体是否为公开状态
    if !media.is_public {
        return Err(Error::Unauthorized("媒体未公开".to_string()));
    }

    // 检查文件是否存在
    let file_path = &media.file_path;
    if !tokio::fs::metadata(file_path).await.is_ok() {
        return Err(Error::NotFound);
    }

    // 获取文件大小
    let file_size = tokio::fs::metadata(file_path).await?.len();

    // 处理 Range 请求（支持断点续传和拖动播放）
    if let Some(range_value) = headers.get(header::RANGE) {
        // 解析 Range 头
        let range_str = range_value.to_str().map_err(|_| {
            Error::BadRequest("无效的 Range 头".to_string())
        })?;

        if let Some((start, end)) = parse_range_header(range_str, file_size) {
            return serve_range_file(file_path, start, end, file_size, &media.mime_type).await;
        }
    }

    // 增加播放次数
    increment_play_count(&ctx, media.id).await?;

    // 完整文件服务
    serve_full_file(file_path, &media.mime_type).await
}

/// 获取媒体公开信息
#[debug_handler]
pub async fn get_media_info(
    Path(access_token): Path<String>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    // 查找媒体记录
    let media = Entity::find()
        .filter(Column::AccessToken.eq(&access_token))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::Message("媒体不存在或访问令牌无效".to_string()))?;

    // 检查媒体是否为公开状态
    if !media.is_public {
        return Err(Error::Unauthorized("媒体未公开".to_string()));
    }

    // 增加播放次数
    increment_play_count(&ctx, media.id).await?;

    format::json(PublicMediaResponse::from(media))
}

/// 增加播放次数
async fn increment_play_count(ctx: &AppContext, media_id: i32) -> Result<()> {
    use sea_orm::{Set, ActiveModelTrait};

    let media = Entity::find_by_id(media_id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::Message("媒体不存在".to_string()))?;

    let play_count = media.play_count;
    let mut active_model: crate::models::_entities::medias::ActiveModel = media.into();
    active_model.play_count = Set(play_count + 1);

    active_model.update(&ctx.db).await?;

    Ok(())
}

/// 解析 Range 头
fn parse_range_header(range_str: &str, file_size: u64) -> Option<(u64, u64)> {
    // 解析 "bytes=start-end" 格式
    if !range_str.starts_with("bytes=") {
        return None;
    }

    let range_part = &range_str[6..]; // 跳过 "bytes="

    // 简单解析，只处理单一范围
    if let Some((start_str, end_str)) = range_part.split_once('-') {
        let start = if start_str.is_empty() {
            0
        } else {
            start_str.parse::<u64>().ok()?
        };

        let end = if end_str.is_empty() {
            file_size - 1
        } else {
            end_str.parse::<u64>().ok()?
        };

        if start <= end && end < file_size {
            return Some((start, end));
        }
    }

    None
}

/// 服务文件范围（Range 请求）
async fn serve_range_file(
    file_path: &str,
    start: u64,
    end: u64,
    file_size: u64,
    mime_type: &Option<String>,
) -> Result<Response> {
    // 打开文件
    let mut file = File::open(file_path).await
        .map_err(|_| Error::InternalServerError)?;

    // 跳转到开始位置
    file.seek(SeekFrom::Start(start)).await
        .map_err(|_| Error::InternalServerError)?;

    // 读取指定范围的数据
    let mut buffer = Vec::new();
    let remaining = end - start + 1;
    use tokio::io::AsyncReadExt;
    file.take(remaining).read_to_end(&mut buffer).await
        .map_err(|_| Error::InternalServerError)?;

    // 构建 206 Partial Content 响应
    let content_type = mime_type.as_deref().unwrap_or("application/octet-stream");
    let content_range = format!("bytes {}-{}/{}", start, end, file_size);
    let content_length = buffer.len();

    let response = Response::builder()
        .status(StatusCode::PARTIAL_CONTENT)
        .header(header::CONTENT_TYPE, content_type)
        .header(header::CONTENT_RANGE, content_range)
        .header(header::ACCEPT_RANGES, "bytes")
        .header(header::CONTENT_LENGTH, content_length)
        .body(Body::from(buffer))
        .map_err(|_| Error::InternalServerError)?;

    Ok(response)
}

/// 服务完整文件
async fn serve_full_file(file_path: &str, mime_type: &Option<String>) -> Result<Response> {
    // 读取整个文件
    let file_contents = tokio::fs::read(file_path).await
        .map_err(|_| Error::InternalServerError)?;

    let content_type = mime_type.as_deref().unwrap_or("application/octet-stream");

    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, content_type)
        .header(header::ACCEPT_RANGES, "bytes")
        .header(header::CONTENT_LENGTH, file_contents.len())
        .body(Body::from(file_contents))
        .map_err(|_| Error::InternalServerError)?;

    Ok(response)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/public/media")
        .add("/{access_token}", get(get_media))
        .add("/{access_token}/info", get(get_media_info))
}