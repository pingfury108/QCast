use std::path::Path;
use loco_rs::prelude::*;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::probe::Hint;
use symphonia::default::get_probe;

/// 音频元数据结构体
#[derive(Debug, Clone, Default)]
pub struct AudioMetadata {
    pub duration: Option<i32>,     // 时长（秒）
}

/// 音频元数据提取服务
#[derive(Debug)]
pub struct AudioMetadataService;

impl AudioMetadataService {
    /// 创建新的音频元数据服务实例
    pub fn new() -> Self {
        Self
    }

    /// 从文件路径提取音频时长
    pub fn extract_from_file(&self, file_path: &str, mime_type: &str) -> Result<AudioMetadata> {
        // 只处理音频文件
        if !mime_type.starts_with("audio/") {
            return Ok(AudioMetadata::default());
        }

        let file_path = Path::new(file_path);
        if !file_path.exists() {
            return Ok(AudioMetadata::default());
        }

        // 打开文件
        let file = std::fs::File::open(file_path)
            .map_err(|e| Error::Message(format!("无法打开文件: {}", e)))?;

        let mss = MediaSourceStream::new(Box::new(file), Default::default());

        // 创建格式提示
        let mut hint = Hint::new();

        // 根据MIME类型设置格式提示
        let extension = file_path.extension().and_then(|s| s.to_str());
        let ext_to_use = extension.unwrap_or_else(|| {
            match mime_type {
                "audio/mpeg" | "audio/mp3" => "mp3",
                "audio/mp4" | "audio/x-m4a" => "m4a",
                "audio/flac" => "flac",
                "audio/ogg" => "ogg",
                "audio/wav" | "audio/x-wav" => "wav",
                "audio/opus" => "opus",
                _ => "mp3", // 默认设为mp3
            }
        });

        hint.with_extension(ext_to_use);

        // 探测格式
        let format_opts = FormatOptions {
            enable_gapless: true,
            ..Default::default()
        };

        let probed = get_probe()
            .format(&hint, mss, &format_opts, &Default::default())
            .map_err(|e| Error::Message(format!("无法识别音频格式: {}", e)))?;

        let format = probed.format;

        // 查找第一个音频轨道
        let track = format
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != symphonia::core::codecs::CODEC_TYPE_NULL)
            .ok_or_else(|| Error::Message("未找到音频轨道".to_string()))?;

        // 获取编解码器参数
        let codec_params = &track.codec_params;

        // 计算时长
        let duration = if let Some(n_frames) = codec_params.n_frames {
            if let Some(time_base) = codec_params.time_base {
                let duration_secs = n_frames as f64 * time_base.numer as f64 / time_base.denom as f64;
                Some(duration_secs as i32)
            } else {
                None
            }
        } else {
            None
        };

        Ok(AudioMetadata { duration })
    }

    /// 仅提取时长（轻量级方法）
    pub fn extract_duration_only(&self, file_path: &str, mime_type: &str) -> Option<i32> {
        match self.extract_from_file(file_path, mime_type) {
            Ok(metadata) => metadata.duration,
            Err(_) => None,
        }
    }

    /// 带容错的元数据提取
    pub fn extract_with_fallback(&self, file_path: &str, mime_type: &str) -> AudioMetadata {
        self.extract_from_file(file_path, mime_type)
            .unwrap_or_else(|_| AudioMetadata::default())
    }
}

impl Default for AudioMetadataService {
    fn default() -> Self {
        Self::new()
    }
}

// 全局音频元数据服务实例
lazy_static::lazy_static! {
    pub static ref AUDIO_METADATA_SERVICE: AudioMetadataService = AudioMetadataService::new();
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::io::Write;

    #[test]
    fn test_metadata_service_creation() {
        let _service = AudioMetadataService::new();
        assert!(true); // 基本创建测试
    }

    #[test]
    fn test_non_audio_file() {
        let service = AudioMetadataService::new();
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        let mut file = std::fs::File::create(&file_path).unwrap();
        writeln!(file, "This is not an audio file").unwrap();

        let metadata = service.extract_from_file(
            file_path.to_str().unwrap(),
            "text/plain"
        ).unwrap();

        assert_eq!(metadata.duration, None);
    }

    #[test]
    fn test_nonexistent_file() {
        let service = AudioMetadataService::new();
        let metadata = service.extract_from_file(
            "/nonexistent/file.mp3",
            "audio/mpeg"
        ).unwrap();

        assert_eq!(metadata.duration, None);
    }

    #[test]
    fn test_fallback_extraction() {
        let service = AudioMetadataService::new();
        let metadata = service.extract_with_fallback(
            "/nonexistent/file.mp3",
            "audio/mpeg"
        );

        assert_eq!(metadata.duration, None);
    }

    #[test]
    fn test_duration_only_extraction() {
        let service = AudioMetadataService::new();
        let duration = service.extract_duration_only(
            "/nonexistent/file.mp3",
            "audio/mpeg"
        );

        assert_eq!(duration, None);
    }
}