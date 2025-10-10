use loco_rs::prelude::*;
use std::path::Path;

/// 视频元数据结构体
#[derive(Debug, Clone, Default)]
pub struct VideoMetadata {
    pub duration: Option<i32>, // 时长（秒）
}

/// 视频元数据提取服务
#[derive(Debug)]
pub struct VideoMetadataService;

impl VideoMetadataService {
    /// 创建新的视频元数据服务实例
    pub fn new() -> Self {
        Self
    }

    /// 从文件路径提取视频时长
    pub fn extract_from_file(&self, file_path: &str, mime_type: &str) -> Result<VideoMetadata> {
        // 只处理视频文件
        if !mime_type.starts_with("video/") {
            return Ok(VideoMetadata::default());
        }

        let file_path = Path::new(file_path);
        if !file_path.exists() {
            return Ok(VideoMetadata::default());
        }

        // 初始化 FFmpeg（只需要初始化一次）
        ffmpeg_next::init().map_err(|e| Error::Message(format!("FFmpeg 初始化失败: {}", e)))?;

        // 打开输入文件
        let input = ffmpeg_next::format::input(&file_path)
            .map_err(|e| Error::Message(format!("无法打开视频文件: {}", e)))?;

        // 获取时长（微秒）
        let duration_micros = input.duration();

        if duration_micros > 0 {
            // 转换为秒
            let duration_secs = (duration_micros as f64 / 1_000_000.0).round() as i32;
            Ok(VideoMetadata {
                duration: Some(duration_secs),
            })
        } else {
            // 尝试从流中获取时长
            let duration = input
                .streams()
                .best(ffmpeg_next::media::Type::Video)
                .and_then(|stream| {
                    let duration_ts = stream.duration();
                    if duration_ts > 0 {
                        let time_base = stream.time_base();
                        let duration_secs = (duration_ts as f64 * time_base.numerator() as f64
                            / time_base.denominator() as f64)
                            .round() as i32;
                        Some(duration_secs)
                    } else {
                        None
                    }
                });

            Ok(VideoMetadata { duration })
        }
    }

    /// 仅提取时长（轻量级方法）
    pub fn extract_duration_only(&self, file_path: &str, mime_type: &str) -> Option<i32> {
        match self.extract_from_file(file_path, mime_type) {
            Ok(metadata) => metadata.duration,
            Err(_) => None,
        }
    }

    /// 带容错的元数据提取
    pub fn extract_with_fallback(&self, file_path: &str, mime_type: &str) -> VideoMetadata {
        self.extract_from_file(file_path, mime_type)
            .unwrap_or_else(|_| VideoMetadata::default())
    }
}

impl Default for VideoMetadataService {
    fn default() -> Self {
        Self::new()
    }
}

// 全局视频元数据服务实例
lazy_static::lazy_static! {
    pub static ref VIDEO_METADATA_SERVICE: VideoMetadataService = VideoMetadataService::new();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_service_creation() {
        let _service = VideoMetadataService::new();
        assert!(true);
    }

    #[test]
    fn test_non_video_file() {
        let service = VideoMetadataService::new();
        let metadata = service
            .extract_from_file("/nonexistent/file.txt", "text/plain")
            .unwrap();

        assert_eq!(metadata.duration, None);
    }

    #[test]
    fn test_nonexistent_file() {
        let service = VideoMetadataService::new();
        let metadata = service
            .extract_from_file("/nonexistent/file.mp4", "video/mp4")
            .unwrap();

        assert_eq!(metadata.duration, None);
    }

    #[test]
    fn test_fallback_extraction() {
        let service = VideoMetadataService::new();
        let metadata = service.extract_with_fallback("/nonexistent/file.mp4", "video/mp4");

        assert_eq!(metadata.duration, None);
    }
}
