use loco_rs::prelude::*;
use std::path::{Path, PathBuf};
use tokio::fs;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct StorageService {
    base_path: PathBuf,
}

#[derive(Debug)]
pub struct UploadedFile {
    pub filename: String,
    pub size: u64,
    pub content_type: String,
    pub path: PathBuf,
}

#[derive(Debug)]
pub struct FileValidationError {
    pub message: String,
}

impl StorageService {
    pub fn new<P: AsRef<Path>>(base_path: P) -> Self {
        Self {
            base_path: base_path.as_ref().to_path_buf(),
        }
    }

    /// 获取用户的媒体文件存储路径
    pub fn get_user_media_path(&self, user_id: i32, book_id: i32) -> PathBuf {
        self.base_path
            .join("users")
            .join(user_id.to_string())
            .join("books")
            .join(book_id.to_string())
            .join("media")
    }

    /// 获取二维码存储路径
    pub fn get_qrcode_path(&self) -> PathBuf {
        self.base_path.join("qrcodes")
    }

    /// 确保目录存在
    async fn ensure_dir_exists(&self, path: &Path) -> Result<()> {
        if !path.exists() {
            fs::create_dir_all(path).await?;
        }
        Ok(())
    }

    /// 验证文件类型
    pub fn validate_file_type(&self, filename: &str, content_type: &str) -> Result<()> {
        let allowed_types = vec![
            // 音频格式
            "audio/mpeg",  // MP3
            "audio/mp4",   // M4A
            "audio/x-m4a", // M4A (alternative)
            "audio/wav",   // WAV
            "audio/x-wav", // WAV (alternative)
            "audio/aac",   // AAC
            "audio/ogg",   // OGG
            "audio/flac",  // FLAC
            "audio/webm",  // WebM Audio
            // 视频格式
            "video/mp4",        // MP4
            "video/quicktime",  // MOV
            "video/x-msvideo",  // AVI
            "video/x-matroska", // MKV
            "video/webm",       // WebM
            "video/mpeg",       // MPEG
            "video/x-flv",      // FLV
            "video/3gpp",       // 3GP
            "video/3gpp2",      // 3G2
        ];

        if !allowed_types.contains(&content_type) {
            return Err(Error::Message(format!(
                "不支持的文件类型: {}. 支持的格式: 音频(MP3, M4A, WAV, AAC, OGG, FLAC, WebM), 视频(MP4, MOV, AVI, MKV, WebM, MPEG, FLV, 3GP)",
                content_type
            )));
        }

        // 验证文件扩展名与MIME类型是否匹配
        let extension = Path::new(filename)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();

        match content_type {
            "audio/mpeg" => {
                if !["mp3"].contains(&extension.as_str()) {
                    return Err(Error::Message("文件扩展名与MIME类型不匹配".to_string()));
                }
            }
            "audio/mp4" | "audio/x-m4a" => {
                if !["m4a", "mp4"].contains(&extension.as_str()) {
                    return Err(Error::Message("文件扩展名与MIME类型不匹配".to_string()));
                }
            }
            "audio/wav" | "audio/x-wav" => {
                if !["wav"].contains(&extension.as_str()) {
                    return Err(Error::Message("文件扩展名与MIME类型不匹配".to_string()));
                }
            }
            "audio/aac" => {
                if !["aac"].contains(&extension.as_str()) {
                    return Err(Error::Message("文件扩展名与MIME类型不匹配".to_string()));
                }
            }
            "audio/ogg" => {
                if !["ogg", "oga"].contains(&extension.as_str()) {
                    return Err(Error::Message("文件扩展名与MIME类型不匹配".to_string()));
                }
            }
            "audio/flac" => {
                if !["flac"].contains(&extension.as_str()) {
                    return Err(Error::Message("文件扩展名与MIME类型不匹配".to_string()));
                }
            }
            "video/mp4" => {
                if !["mp4", "m4v"].contains(&extension.as_str()) {
                    return Err(Error::Message("文件扩展名与MIME类型不匹配".to_string()));
                }
            }
            "video/quicktime" => {
                if !["mov", "qt"].contains(&extension.as_str()) {
                    return Err(Error::Message("文件扩展名与MIME类型不匹配".to_string()));
                }
            }
            "video/x-msvideo" => {
                if !["avi"].contains(&extension.as_str()) {
                    return Err(Error::Message("文件扩展名与MIME类型不匹配".to_string()));
                }
            }
            "video/x-matroska" => {
                if !["mkv", "mk3d", "mka", "mks"].contains(&extension.as_str()) {
                    return Err(Error::Message("文件扩展名与MIME类型不匹配".to_string()));
                }
            }
            "video/webm" | "audio/webm" => {
                if !["webm"].contains(&extension.as_str()) {
                    return Err(Error::Message("文件扩展名与MIME类型不匹配".to_string()));
                }
            }
            "video/mpeg" => {
                if !["mpg", "mpeg", "mpe", "m1v", "m2v"].contains(&extension.as_str()) {
                    return Err(Error::Message("文件扩展名与MIME类型不匹配".to_string()));
                }
            }
            "video/x-flv" => {
                if !["flv"].contains(&extension.as_str()) {
                    return Err(Error::Message("文件扩展名与MIME类型不匹配".to_string()));
                }
            }
            "video/3gpp" => {
                if !["3gp"].contains(&extension.as_str()) {
                    return Err(Error::Message("文件扩展名与MIME类型不匹配".to_string()));
                }
            }
            "video/3gpp2" => {
                if !["3g2"].contains(&extension.as_str()) {
                    return Err(Error::Message("文件扩展名与MIME类型不匹配".to_string()));
                }
            }
            _ => {} // 其他类型暂时跳过扩展名验证
        }

        Ok(())
    }

    /// 保存上传的文件
    pub async fn save_file(
        &self,
        user_id: i32,
        book_id: i32,
        filename: &str,
        content_type: &str,
        data: &[u8],
    ) -> Result<UploadedFile> {
        // 验证文件类型
        self.validate_file_type(filename, content_type)?;

        // 生成唯一的文件名
        let extension = Path::new(filename)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("bin");
        let unique_filename = format!("{}.{}", Uuid::new_v4(), extension);

        // 确保目标目录存在
        let target_dir = self.get_user_media_path(user_id, book_id);
        self.ensure_dir_exists(&target_dir).await?;

        // 保存文件
        let file_path = target_dir.join(&unique_filename);
        fs::write(&file_path, data).await?;

        Ok(UploadedFile {
            filename: filename.to_string(),
            size: data.len() as u64,
            content_type: content_type.to_string(),
            path: file_path,
        })
    }

    /// 替换现有文件
    pub async fn replace_file(
        &self,
        user_id: i32,
        book_id: i32,
        old_file_path: &Path,
        filename: &str,
        content_type: &str,
        data: &[u8],
    ) -> Result<UploadedFile> {
        // 验证文件类型
        self.validate_file_type(filename, content_type)?;

        // 生成新的唯一文件名
        let extension = Path::new(filename)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("bin");
        let unique_filename = format!("{}.{}", Uuid::new_v4(), extension);

        // 确保目标目录存在
        let target_dir = self.get_user_media_path(user_id, book_id);
        self.ensure_dir_exists(&target_dir).await?;

        // 保存新文件到临时位置
        let temp_path = target_dir.join(format!("{}.tmp", unique_filename));
        fs::write(&temp_path, data).await?;

        // 原子性替换文件
        let new_file_path = target_dir.join(&unique_filename);

        // 删除旧文件
        if old_file_path.exists() {
            fs::remove_file(old_file_path).await?;
        }

        // 重命名临时文件为正式文件
        fs::rename(&temp_path, &new_file_path).await?;

        Ok(UploadedFile {
            filename: filename.to_string(),
            size: data.len() as u64,
            content_type: content_type.to_string(),
            path: new_file_path,
        })
    }

    /// 删除文件
    pub async fn delete_file(&self, file_path: &Path) -> Result<()> {
        if file_path.exists() {
            fs::remove_file(file_path).await?;
        }
        Ok(())
    }

    /// 检查文件是否存在
    pub fn file_exists(&self, file_path: &Path) -> bool {
        file_path.exists()
    }

    /// 获取文件大小
    pub async fn get_file_size(&self, file_path: &Path) -> Result<u64> {
        if file_path.exists() {
            let metadata = fs::metadata(file_path).await?;
            Ok(metadata.len())
        } else {
            Err(Error::Message("文件不存在".to_string()))
        }
    }

    /// 流式保存文件到临时位置（不占用内存）
    /// 返回临时文件路径和文件大小
    pub async fn save_to_temp<S, E>(
        &self,
        mut stream: S,
        original_filename: &str,
        content_type: &str,
        max_size: u64,
    ) -> Result<(PathBuf, u64)>
    where
        S: futures_util::Stream<Item = Result<bytes::Bytes, E>> + Unpin,
        E: std::fmt::Display,
    {
        use futures_util::StreamExt;
        use tokio::io::AsyncWriteExt;

        // 验证文件类型
        self.validate_file_type(original_filename, content_type)?;

        // 创建临时文件
        let temp_dir = std::env::temp_dir();
        let temp_filename = format!("upload_{}.tmp", Uuid::new_v4());
        let temp_path = temp_dir.join(temp_filename);

        let mut file = tokio::fs::File::create(&temp_path)
            .await
            .map_err(|e| Error::Message(format!("创建临时文件失败: {e}")))?;

        let mut total_size: u64 = 0;

        // 逐块读取并写入
        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result.map_err(|e| Error::Message(format!("读取数据块失败: {e}")))?;

            total_size += chunk.len() as u64;

            // 检查大小限制
            if total_size > max_size {
                // 清理临时文件
                let _ = tokio::fs::remove_file(&temp_path).await;
                return Err(Error::Message(format!(
                    "文件大小超过限制 ({}GB)",
                    max_size / 1_073_741_824
                )));
            }

            // 写入块
            file.write_all(&chunk)
                .await
                .map_err(|e| Error::Message(format!("写入数据失败: {e}")))?;
        }

        // 确保所有数据都写入磁盘
        file.flush()
            .await
            .map_err(|e| Error::Message(format!("刷新缓冲区失败: {e}")))?;

        Ok((temp_path, total_size))
    }

    /// 从临时文件移动文件到永久存储位置（避免内存加载）
    ///
    /// # Errors
    ///
    /// Will return error if file validation fails or file operations fail
    pub async fn move_temp_file(
        &self,
        user_id: i32,
        book_id: i32,
        temp_file_path: &Path,
        filename: &str,
        content_type: &str,
        file_size: u64,
    ) -> Result<UploadedFile> {
        // 验证文件类型
        self.validate_file_type(filename, content_type)?;

        // 获取目标路径
        let media_path = self.get_user_media_path(user_id, book_id);
        self.ensure_dir_exists(&media_path).await?;

        // 生成唯一文件名
        let file_extension = Path::new(filename)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("bin");

        let unique_filename = format!("{}.{}", Uuid::new_v4(), file_extension);
        let final_path = media_path.join(&unique_filename);

        // 移动文件（这是原子操作，比复制+删除更高效）
        fs::rename(temp_file_path, &final_path)
            .await
            .map_err(|e| Error::Message(format!("移动文件失败: {e}")))?;

        Ok(UploadedFile {
            filename: unique_filename,
            size: file_size,
            content_type: content_type.to_string(),
            path: final_path,
        })
    }

    /// 确定文件类型（音频或视频）
    ///
    /// # Errors
    ///
    /// Will return error if content type is not supported
    pub fn determine_file_type(&self, content_type: &str) -> Result<String> {
        if content_type.starts_with("audio/") {
            Ok("audio".to_string())
        } else if content_type.starts_with("video/") {
            Ok("video".to_string())
        } else {
            Err(Error::Message(format!("不支持的文件类型: {content_type}")))
        }
    }
}

// 全局存储服务实例
pub static STORAGE_SERVICE: std::sync::LazyLock<StorageService> = std::sync::LazyLock::new(|| {
    let storage_path = std::env::var("STORAGE_PATH").unwrap_or_else(|_| "uploads".to_string());
    StorageService::new(storage_path)
});

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_save_file() {
        let temp_dir = TempDir::new().unwrap();
        let storage = StorageService::new(temp_dir.path());

        let user_id = 1;
        let book_id = 1;
        let filename = "test.mp3";
        let content_type = "audio/mpeg";
        let data = b"test audio data";

        let uploaded_file = storage
            .save_file(user_id, book_id, filename, content_type, data)
            .await
            .unwrap();

        assert_eq!(uploaded_file.filename, filename);
        assert_eq!(uploaded_file.content_type, content_type);
        assert_eq!(uploaded_file.size, data.len() as u64);
        assert!(uploaded_file.path.exists());
    }

    #[tokio::test]
    async fn test_validate_file_type() {
        let temp_dir = TempDir::new().unwrap();
        let storage = StorageService::new(temp_dir.path());

        // 有效的文件类型
        assert!(storage.validate_file_type("test.mp3", "audio/mpeg").is_ok());
        assert!(storage.validate_file_type("test.mp4", "video/mp4").is_ok());

        // 无效的文件类型
        assert!(storage
            .validate_file_type("test.txt", "text/plain")
            .is_err());

        // 扩展名不匹配
        assert!(storage
            .validate_file_type("test.txt", "audio/mpeg")
            .is_err());
    }

    #[tokio::test]
    async fn test_determine_file_type() {
        let temp_dir = TempDir::new().unwrap();
        let storage = StorageService::new(temp_dir.path());

        assert_eq!(storage.determine_file_type("audio/mpeg").unwrap(), "audio");
        assert_eq!(storage.determine_file_type("video/mp4").unwrap(), "video");
        assert!(storage.determine_file_type("text/plain").is_err());
    }
}
