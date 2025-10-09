#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use axum::extract::{Path as AxumPath, Query, DefaultBodyLimit};
use axum::routing::method_routing::delete as axum_delete;
use axum_extra::extract::Multipart;
use loco_rs::prelude::*;
use uuid::Uuid;

use crate::models::_entities::medias::{ActiveModel, Column, Entity, Model};
use crate::models::users;
use crate::services::storage::StorageService;
use crate::services::qrcode::QRCODE_SERVICE;
use crate::services::audio_metadata::AUDIO_METADATA_SERVICE;
use crate::views::medias::{MediaResponse, UpdateMediaParams};


async fn load_item(ctx: &AppContext, id: i32, user_id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id)
        .filter(Column::UserId.eq(user_id))
        .one(&ctx.db)
        .await?;
    item.ok_or_else(|| Error::NotFound)
}

/// 获取当前用户的所有媒体
#[debug_handler]
pub async fn list(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let medias = Entity::find()
        .filter(Column::UserId.eq(user.id))
        .all(&ctx.db)
        .await?;

    let responses: Vec<MediaResponse> = medias.into_iter().map(MediaResponse::from).collect();

    format::json(responses)
}

/// 获取媒体详情
#[debug_handler]
pub async fn show(
    auth: auth::JWT,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let item = load_item(&ctx, id, user.id).await?;

    format::json(MediaResponse::from(item))
}

/// 更新媒体
#[debug_handler]
pub async fn update(
    auth: auth::JWT,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<UpdateMediaParams>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let item = load_item(&ctx, id, user.id).await?;

    let mut item = item.into_active_model();

    if let Some(title) = params.title {
        item.title = Set(title);
    }
    if let Some(description) = params.description {
        item.description = Set(Some(description));
    }
    if let Some(is_public) = params.is_public {
        item.is_public = Set(is_public);
    }
    if let Some(chapter_id) = params.chapter_id {
        // 如果提供了 chapter_id，验证章节是否存在且属于同一个书籍
        if chapter_id > 0 {
            use crate::models::_entities::chapters;
            let chapter = chapters::Entity::find_by_id(chapter_id)
                .one(&ctx.db)
                .await?
                .ok_or_else(|| Error::Message("章节不存在".to_string()))?;

            // 验证章节属于同一个书籍
            let book_id = item.book_id.as_ref();
            if chapter.book_id != *book_id {
                return Err(Error::Message("章节不属于该媒体所在的书籍".to_string()));
            }

            item.chapter_id = Set(Some(chapter_id));
        } else {
            // chapter_id 为 0 或负数时，表示移除章节关联
            item.chapter_id = Set(None);
        }
    }

    let item = item.update(&ctx.db).await?;

    format::json(MediaResponse::from(item))
}

/// 删除媒体
#[debug_handler]
pub async fn delete(
    auth: auth::JWT,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let item = load_item(&ctx, id, user.id).await?;

    item.delete(&ctx.db).await?;
    format::empty()
}

/// 搜索媒体
#[debug_handler]
pub async fn search(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let query = params.get("q").map(|q| q.as_str()).unwrap_or("");

    if query.is_empty() {
        return format::json(Vec::<MediaResponse>::new());
    }

    let medias = Model::search(&ctx.db, user.id, query).await?;
    let responses: Vec<MediaResponse> = medias.into_iter().map(MediaResponse::from).collect();

    format::json(responses)
}

/// 上传媒体文件
#[debug_handler]
pub async fn upload(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    mut multipart: Multipart,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let mut temp_file_path: Option<std::path::PathBuf> = None;
    let mut file_size: Option<u64> = None;
    let mut filename: Option<String> = None;
    let mut content_type: Option<String> = None;
    let mut title: Option<String> = None;
    let mut description: Option<String> = None;
    let mut book_id: Option<i32> = None;
    let mut chapter_id: Option<i32> = None;

    // 设置最大文件大小为 2GB
    const MAX_FILE_SIZE: u64 = 2_147_483_648;

    // 解析 multipart 数据
    while let Some(mut field) = multipart
        .next_field()
        .await
        .map_err(|e| Error::Message(format!("解析上传数据失败: {}", e)))?
    {
        let name = field.name().unwrap_or("").to_string();

        match name.as_str() {
            "file" => {
                let field_filename = field.file_name().map(|s| s.to_string());
                let field_content_type = field.content_type().map(|s| s.to_string());

                // 验证必需信息
                let fname = field_filename.clone()
                    .ok_or_else(|| Error::Message("缺少文件名".to_string()))?;
                let ctype = field_content_type.clone()
                    .ok_or_else(|| Error::Message("缺少文件类型".to_string()))?;

                // 先验证文件类型
                let storage = StorageService::new("uploads");
                storage.validate_file_type(&fname, &ctype)?;

                // 创建临时文件并流式写入
                use tokio::io::AsyncWriteExt;
                let temp_dir = std::env::temp_dir();
                let temp_filename = format!("upload_{}.tmp", uuid::Uuid::new_v4());
                let temp_path = temp_dir.join(&temp_filename);

                let mut file = tokio::fs::File::create(&temp_path)
                    .await
                    .map_err(|e| Error::Message(format!("创建临时文件失败: {}", e)))?;

                let mut total_size: u64 = 0;

                // 逐块读取并写入
                while let Some(chunk) = field.chunk().await
                    .map_err(|e| Error::Message(format!("读取数据块失败: {}", e)))?
                {
                    total_size += chunk.len() as u64;

                    // 检查大小限制
                    if total_size > MAX_FILE_SIZE {
                        // 清理临时文件
                        let _ = tokio::fs::remove_file(&temp_path).await;
                        return Err(Error::Message(format!(
                            "文件大小超过限制 ({}GB)",
                            MAX_FILE_SIZE / 1_073_741_824
                        )));
                    }

                    // 写入块
                    file.write_all(&chunk)
                        .await
                        .map_err(|e| Error::Message(format!("写入数据失败: {}", e)))?;
                }

                // 确保所有数据都写入磁盘
                file.flush()
                    .await
                    .map_err(|e| Error::Message(format!("刷新缓冲区失败: {}", e)))?;

                temp_file_path = Some(temp_path);
                file_size = Some(total_size);
                filename = Some(fname);
                content_type = Some(ctype);
            }
            "title" => {
                title = Some(
                    field
                        .text()
                        .await
                        .map_err(|e| Error::Message(format!("读取标题失败: {}", e)))?,
                );
            }
            "description" => {
                description = Some(
                    field
                        .text()
                        .await
                        .map_err(|e| Error::Message(format!("读取描述失败: {}", e)))?,
                );
            }
            "book_id" => {
                let book_id_str = field
                    .text()
                    .await
                    .map_err(|e| Error::Message(format!("读取书籍ID失败: {}", e)))?;
                book_id = Some(
                    book_id_str
                        .parse::<i32>()
                        .map_err(|_| Error::Message("无效的书籍ID".to_string()))?,
                );
            }
            "chapter_id" => {
                let chapter_id_str = field
                    .text()
                    .await
                    .map_err(|e| Error::Message(format!("读取章节ID失败: {}", e)))?;
                chapter_id = Some(
                    chapter_id_str
                        .parse::<i32>()
                        .map_err(|_| Error::Message("无效的章节ID".to_string()))?,
                );
            }
            _ => {
                // 忽略未知字段
            }
        }
    }

    // 验证必需字段
    let temp_path = temp_file_path
        .ok_or_else(|| Error::Message("缺少文件数据".to_string()))?;
    let file_size = file_size
        .ok_or_else(|| Error::Message("未获取文件大小".to_string()))?;
    let filename = filename.ok_or_else(|| Error::Message("缺少文件名".to_string()))?;
    let content_type = content_type.ok_or_else(|| Error::Message("缺少文件类型".to_string()))?;
    let title = title.ok_or_else(|| Error::Message("缺少标题".to_string()))?;
    let book_id = book_id.ok_or_else(|| Error::Message("缺少书籍ID".to_string()))?;

    // 验证用户是否拥有该书籍
    use crate::models::_entities::books;
    let _book = books::Entity::find_by_id(book_id)
        .filter(books::Column::UserId.eq(user.id))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::Message("书籍不存在或无权限".to_string()))?;

    // 如果指定了章节ID，验证章节是否属于该书籍
    if let Some(chapter_id) = chapter_id {
        use crate::models::_entities::chapters;
        let _chapter = chapters::Entity::find_by_id(chapter_id)
            .filter(chapters::Column::BookId.eq(book_id))
            .one(&ctx.db)
            .await?
            .ok_or_else(|| Error::Message("章节不存在或不属于指定书籍".to_string()))?;
    }

    // 使用存储服务移动临时文件到永久位置
    let storage = StorageService::new("uploads");
    let uploaded_file = storage
        .move_temp_file(user.id, book_id, &temp_path, &filename, &content_type, file_size)
        .await?;

    // 确定文件类型
    let file_type = storage.determine_file_type(&content_type)?;

    // 提取音频元数据（包含时长）
    let metadata = AUDIO_METADATA_SERVICE.extract_with_fallback(
        &uploaded_file.path.to_string_lossy(),
        &content_type
    );
    let duration = metadata.duration;

    // 记录音频时长提取结果
    if duration.is_some() {
        tracing::info!("音频文件时长提取成功: {}秒 - 文件: {:?}", duration.unwrap(), filename);
    } else if content_type.starts_with("audio/") {
        tracing::warn!("音频文件时长提取失败: 文件: {}, MIME类型: {}", filename, content_type);
    }

    // 生成访问令牌
    let access_token = Uuid::new_v4().to_string();

    // 创建媒体记录
    let media = ActiveModel {
        user_id: Set(user.id),
        book_id: Set(book_id),
        chapter_id: Set(chapter_id),
        title: Set(title),
        description: Set(description),
        file_type: Set(file_type),
        file_path: Set(uploaded_file.path.to_string_lossy().to_string()),
        file_size: Set(Some(uploaded_file.size as i64)),
        duration: Set(duration), // 使用提取的音频时长
        mime_type: Set(Some(content_type)),
        access_token: Set(access_token.clone()),
        access_url: Set(Some(format!(
            "{}/public/media/{}",
            ctx.config.server.full_url(),
            access_token
        ))),
        qr_code_path: Set(None), // 暂时为空，后续可以生成二维码
        file_version: Set(1),
        original_filename: Set(Some(filename)),
        play_count: Set(0),
        is_public: Set(false),
        ..Default::default()
    };

    let media = media.insert(&ctx.db).await?;

    // 异步生成二维码（不阻塞响应）
    if let Some(ref access_url) = media.access_url {
        let media_id = media.id;
        let access_url_clone = access_url.clone();
        let ctx_clone = ctx.clone();

        // 使用 tokio spawn 异步生成二维码
        tokio::spawn(async move {
            if let Err(e) = generate_qrcode_for_media(&ctx_clone, media_id, &access_url_clone).await {
                tracing::error!("异步生成二维码失败: {}", e);
            }
        });
    }

    format::json(MediaResponse::from(media))
}

/// 替换媒体文件
#[debug_handler]
pub async fn replace_file(
    auth: auth::JWT,
    AxumPath(id): AxumPath<i32>,
    State(ctx): State<AppContext>,
    mut multipart: Multipart,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    // 验证媒体是否存在且属于当前用户
    let media = load_item(&ctx, id, user.id).await?;
    let book_id = media.book_id;
    let old_file_path = media.file_path.clone();
    let old_file_version = media.file_version;

    let mut temp_file_path: Option<std::path::PathBuf> = None;
    let mut file_size: Option<u64> = None;
    let mut filename: Option<String> = None;
    let mut content_type: Option<String> = None;

    // 设置最大文件大小为 2GB
    const MAX_FILE_SIZE: u64 = 2_147_483_648;

    // 解析 multipart 数据
    while let Some(mut field) = multipart
        .next_field()
        .await
        .map_err(|e| Error::Message(format!("解析上传数据失败: {}", e)))?
    {
        let name = field.name().unwrap_or("").to_string();

        if name == "file" {
            let field_filename = field.file_name().map(|s| s.to_string());
            let field_content_type = field.content_type().map(|s| s.to_string());

            // 验证必需信息
            let fname = field_filename.clone()
                .ok_or_else(|| Error::Message("缺少文件名".to_string()))?;
            let ctype = field_content_type.clone()
                .ok_or_else(|| Error::Message("缺少文件类型".to_string()))?;

            // 先验证文件类型
            let storage = StorageService::new("uploads");
            storage.validate_file_type(&fname, &ctype)?;

            // 创建临时文件并流式写入
            use tokio::io::AsyncWriteExt;
            let temp_dir = std::env::temp_dir();
            let temp_filename = format!("upload_{}.tmp", uuid::Uuid::new_v4());
            let temp_path = temp_dir.join(&temp_filename);

            let mut file = tokio::fs::File::create(&temp_path)
                .await
                .map_err(|e| Error::Message(format!("创建临时文件失败: {}", e)))?;

            let mut total_size: u64 = 0;

            // 逐块读取并写入
            while let Some(chunk) = field.chunk().await
                .map_err(|e| Error::Message(format!("读取数据块失败: {}", e)))?
            {
                total_size += chunk.len() as u64;

                // 检查大小限制
                if total_size > MAX_FILE_SIZE {
                    // 清理临时文件
                    let _ = tokio::fs::remove_file(&temp_path).await;
                    return Err(Error::Message(format!(
                        "文件大小超过限制 ({}GB)",
                        MAX_FILE_SIZE / 1_073_741_824
                    )));
                }

                // 写入块
                file.write_all(&chunk)
                    .await
                    .map_err(|e| Error::Message(format!("写入数据失败: {}", e)))?;
            }

            // 确保所有数据都写入磁盘
            file.flush()
                .await
                .map_err(|e| Error::Message(format!("刷新缓冲区失败: {}", e)))?;

            temp_file_path = Some(temp_path);
            file_size = Some(total_size);
            filename = Some(fname);
            content_type = Some(ctype);
            break;
        }
    }

    // 验证必需字段
    let temp_path = temp_file_path
        .ok_or_else(|| Error::Message("缺少文件数据".to_string()))?;
    let file_size = file_size
        .ok_or_else(|| Error::Message("未获取文件大小".to_string()))?;
    let filename = filename.ok_or_else(|| Error::Message("缺少文件名".to_string()))?;
    let content_type = content_type.ok_or_else(|| Error::Message("缺少文件类型".to_string()))?;

    // 使用存储服务移动临时文件到永久位置（替换旧文件）
    let storage = StorageService::new("uploads");
    let uploaded_file = storage
        .move_temp_file(user.id, book_id, &temp_path, &filename, &content_type, file_size)
        .await?;

    // 删除旧文件
    let _ = storage.delete_file(std::path::Path::new(&old_file_path)).await;

    // 确定文件类型
    let file_type = storage.determine_file_type(&content_type)?;

    // 提取音频元数据（包含时长）
    let metadata = AUDIO_METADATA_SERVICE.extract_with_fallback(
        &uploaded_file.path.to_string_lossy(),
        &content_type
    );

    // 记录音频时长提取结果
    if metadata.duration.is_some() {
        tracing::info!("音频文件替换后时长提取成功: {}秒 - 文件: {:?}", metadata.duration.unwrap(), filename);
    } else if content_type.starts_with("audio/") {
        tracing::warn!("音频文件替换后时长提取失败: 文件: {}, MIME类型: {}", filename, content_type);
    }

    // 更新媒体记录（保持 access_token 不变）
    let mut active_model: ActiveModel = media.into();
    active_model.file_type = Set(file_type);
    active_model.file_path = Set(uploaded_file.path.to_string_lossy().to_string());
    active_model.file_size = Set(Some(uploaded_file.size as i64));
    active_model.duration = Set(metadata.duration); // 使用提取的时长
    active_model.mime_type = Set(Some(content_type));
    active_model.file_version = Set(old_file_version + 1);
    active_model.original_filename = Set(Some(filename));
    active_model.updated_at = Set(chrono::Utc::now().into());
    // access_token 保持不变

    let updated_media = active_model.update(&ctx.db).await?;

    format::json(MediaResponse::from(updated_media))
}

/// 异步生成二维码的辅助函数
async fn generate_qrcode_for_media(
    ctx: &AppContext,
    media_id: i32,
    access_url: &str,
) -> Result<()> {
    tracing::info!("开始为媒体 {} 生成二维码", media_id);

    // 生成二维码
    let qrcode_path = QRCODE_SERVICE
        .generate_media_qrcode(media_id, access_url)
        .await
        .map_err(|e| {
            tracing::error!("为媒体 {} 生成二维码失败: {}", media_id, e);
            e
        })?;

    // 更新媒体记录中的二维码路径
    let media = crate::models::_entities::medias::Entity::find_by_id(media_id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::Message(format!("媒体 {} 不存在", media_id)))?;

    use sea_orm::ActiveModelTrait;
    use sea_orm::Set;
    let mut active_model: crate::models::_entities::medias::ActiveModel = media.into();
    active_model.qr_code_path = Set(Some(qrcode_path.clone()));

    active_model.update(&ctx.db).await
        .map_err(|e| {
            tracing::error!("更新媒体 {} 的二维码路径失败: {}", media_id, e);
            Error::Message(format!("更新二维码路径失败: {}", e))
        })?;

    tracing::info!("媒体 {} 的二维码生成完成: {}", media_id, qrcode_path);

    Ok(())
}

/// 发布/取消发布媒体
#[debug_handler]
pub async fn publish(
    auth: auth::JWT,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let _ = load_item(&ctx, id, user.id).await?;

    let media = ActiveModel::toggle_public(&ctx.db, id, user.id).await?;

    format::json(MediaResponse::from(media))
}

/// 获取媒体二维码
#[debug_handler]
pub async fn get_qrcode(
    auth: auth::JWT,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let media = load_item(&ctx, id, user.id).await?;

    // 获取访问链接
    let access_url = media.access_url.as_ref()
        .ok_or_else(|| Error::Message("访问链接未生成".to_string()))?;

    // 生成二维码 SVG 字符串
    let svg_data = QRCODE_SERVICE.generate_qrcode_svg_string(access_url)?;

    // 如果这是第一次请求，保存文件并更新记录（可选）
    if media.qr_code_path.is_none() {
        let qrcode_relative_path = QRCODE_SERVICE
            .generate_media_qrcode(media.id, access_url)
            .await?;

        let mut active_model: ActiveModel = media.into();
        active_model.qr_code_path = Set(Some(qrcode_relative_path));
        let _updated_media = active_model.update(&ctx.db).await?;
    }

    // 直接返回二维码图片数据
    use axum::http::{header, StatusCode};
    use axum::body::Body;

    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "image/svg+xml")
        .header(header::CACHE_CONTROL, "public, max-age=3600") // 缓存1小时
        .body(Body::from(svg_data))
        .map_err(|_| Error::InternalServerError)?;

    Ok(response)
}

/// 重新生成媒体二维码
#[debug_handler]
pub async fn regenerate_qrcode(
    auth: auth::JWT,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let media = load_item(&ctx, id, user.id).await?;

    // 重新生成二维码
    let access_url = media.access_url.as_ref()
        .ok_or_else(|| Error::Message("访问链接未生成".to_string()))?;

    let qrcode_relative_path = QRCODE_SERVICE
        .regenerate_media_qrcode(media.id, access_url)
        .await?;

    // 更新媒体记录
    let mut active_model: ActiveModel = media.into();
    active_model.qr_code_path = Set(Some(qrcode_relative_path.clone()));
    let _updated_media = active_model.update(&ctx.db).await?;

    format::json(serde_json::json!({
        "qrcode_path": qrcode_relative_path,
        "qrcode_url": format!("{}{}",
            ctx.config.server.full_url().trim_end_matches('/'),
            qrcode_relative_path
        )
    }))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/media")
        // 上传路由需要更大的 body limit (2.5GB)
        .add(
            "/upload",
            post(upload).layer(DefaultBodyLimit::max(2_500_000_000))
        )
        // 替换文件路由也需要更大的 body limit
        .add(
            "/{id}/replace-file",
            put(replace_file).layer(DefaultBodyLimit::max(2_500_000_000))
        )
        .add("/", get(list))
        .add("/search", get(search))
        .add("/{id}", get(show))
        .add("/{id}", put(update))
        .add("/{id}", patch(update))
        .add("/{id}", axum_delete(delete))
        .add("/{id}/publish", post(publish))
        .add("/{id}/qrcode", get(get_qrcode))
        .add("/{id}/regenerate-qr", post(regenerate_qrcode))
}
