use image::Luma;
use loco_rs::prelude::*;
use qrcode::{render::svg, QrCode};
use std::path::{Path, PathBuf};
use tokio::fs;

#[derive(Debug, Clone)]
pub struct QRCodeService {
    base_path: PathBuf,
}

#[derive(Debug)]
pub struct QRCodeGenerationError {
    pub message: String,
}

impl QRCodeService {
    pub fn new<P: AsRef<Path>>(base_path: P) -> Self {
        Self {
            base_path: base_path.as_ref().to_path_buf(),
        }
    }

    /// 获取二维码存储路径
    pub fn get_qrcode_path(&self) -> PathBuf {
        self.base_path.join("qrcodes")
    }

    /// 确保二维码目录存在
    async fn ensure_dir_exists(&self) -> Result<()> {
        let qrcode_dir = self.get_qrcode_path();
        if !qrcode_dir.exists() {
            fs::create_dir_all(&qrcode_dir).await?;
        }
        Ok(())
    }

    /// 生成二维码并保存为 PNG 文件
    pub async fn generate_qrcode_png(&self, data: &str, filename: &str) -> Result<PathBuf> {
        self.ensure_dir_exists().await?;

        // 生成二维码
        let code =
            QrCode::new(data).map_err(|e| Error::Message(format!("生成二维码失败: {}", e)))?;

        // 渲染为 PNG
        let image = code.render::<Luma<u8>>().build();
        let qrcode_path = self.get_qrcode_path().join(format!("{}.png", filename));

        // 保存到文件
        image
            .save(&qrcode_path)
            .map_err(|e| Error::Message(format!("保存二维码失败: {}", e)))?;

        Ok(qrcode_path)
    }

    /// 生成二维码 SVG 字符串（不保存文件）
    pub fn generate_qrcode_svg_string(&self, data: &str) -> Result<String> {
        // 生成二维码
        let code =
            QrCode::new(data).map_err(|e| Error::Message(format!("生成二维码失败: {}", e)))?;

        // 渲染为 SVG
        let svg_string = code.render::<svg::Color>().build();
        Ok(svg_string)
    }

    /// 生成二维码并保存为 SVG 文件（推荐，体积更小）
    pub async fn generate_qrcode_svg(&self, data: &str, filename: &str) -> Result<PathBuf> {
        self.ensure_dir_exists().await?;

        // 生成二维码
        let code =
            QrCode::new(data).map_err(|e| Error::Message(format!("生成二维码失败: {}", e)))?;

        // 渲染为 SVG
        let svg_string = code.render::<svg::Color>().build();
        let qrcode_path = self.get_qrcode_path().join(format!("{}.svg", filename));

        // 保存到文件
        fs::write(&qrcode_path, svg_string.as_bytes())
            .await
            .map_err(|e| Error::Message(format!("保存二维码失败: {}", e)))?;

        Ok(qrcode_path)
    }

    /// 删除二维码文件
    pub async fn delete_qrcode(&self, filename: &str, extension: &str) -> Result<()> {
        let qrcode_path = self
            .get_qrcode_path()
            .join(format!("{}.{}", filename, extension));

        if qrcode_path.exists() {
            fs::remove_file(&qrcode_path).await?;
        }
        Ok(())
    }

    /// 检查二维码文件是否存在
    pub async fn qrcode_exists(&self, filename: &str, extension: &str) -> bool {
        let qrcode_path = self
            .get_qrcode_path()
            .join(format!("{}.{}", filename, extension));
        qrcode_path.exists()
    }

    /// 为媒体生成二维码（使用媒体ID作为文件名，access_url作为数据）
    pub async fn generate_media_qrcode(&self, media_id: i32, access_url: &str) -> Result<String> {
        // 使用 SVG 格式（体积更小，可缩放）
        let _qrcode_path = self
            .generate_qrcode_svg(access_url, &media_id.to_string())
            .await?;

        // 返回相对于 static 目录的路径
        Ok(format!("/static/qrcodes/{}.svg", media_id))
    }

    /// 重新生成媒体的二维码
    pub async fn regenerate_media_qrcode(&self, media_id: i32, access_url: &str) -> Result<String> {
        // 先删除现有的二维码（如果存在）
        self.delete_qrcode(&media_id.to_string(), "svg").await.ok();
        self.delete_qrcode(&media_id.to_string(), "png").await.ok();

        // 生成新的二维码
        self.generate_media_qrcode(media_id, access_url).await
    }

    /// 删除媒体相关的二维码
    pub async fn delete_media_qrcode(&self, media_id: i32) -> Result<()> {
        self.delete_qrcode(&media_id.to_string(), "svg").await?;
        self.delete_qrcode(&media_id.to_string(), "png").await?;
        Ok(())
    }
}

// 全局二维码服务实例
lazy_static::lazy_static! {
    pub static ref QRCODE_SERVICE: QRCodeService = {
        let storage_path = std::env::var("STORAGE_PATH").unwrap_or_else(|_| "uploads".to_string());
        QRCodeService::new(storage_path)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_generate_qrcode_svg() {
        let temp_dir = TempDir::new().unwrap();
        let service = QRCodeService::new(temp_dir.path());

        let test_data = "https://example.com/test";
        let filename = "test_qr";

        let qrcode_path = service
            .generate_qrcode_svg(test_data, filename)
            .await
            .unwrap();

        assert!(qrcode_path.exists());
        assert!(qrcode_path.to_string_lossy().ends_with(".svg"));
    }

    #[tokio::test]
    async fn test_generate_qrcode_png() {
        let temp_dir = TempDir::new().unwrap();
        let service = QRCodeService::new(temp_dir.path());

        let test_data = "https://example.com/test";
        let filename = "test_qr";

        let qrcode_path = service
            .generate_qrcode_png(test_data, filename)
            .await
            .unwrap();

        assert!(qrcode_path.exists());
        assert!(qrcode_path.to_string_lossy().ends_with(".png"));
    }

    #[tokio::test]
    async fn test_media_qrcode() {
        let temp_dir = TempDir::new().unwrap();
        let service = QRCodeService::new(temp_dir.path());

        let media_id = 123;
        let access_url = "https://example.com/public/media/test-token";

        let relative_path = service
            .generate_media_qrcode(media_id, access_url)
            .await
            .unwrap();

        assert_eq!(relative_path, "/static/qrcodes/123.svg");

        // 检查文件是否真的创建了
        let full_path = temp_dir.path().join("qrcodes/123.svg");
        assert!(full_path.exists());
    }

    #[tokio::test]
    async fn test_regenerate_qrcode() {
        let temp_dir = TempDir::new().unwrap();
        let service = QRCodeService::new(temp_dir.path());

        let media_id = 456;
        let access_url = "https://example.com/public/media/test-token";

        // 生成第一个二维码
        let path1 = service
            .generate_media_qrcode(media_id, access_url)
            .await
            .unwrap();

        // 重新生成二维码
        let path2 = service
            .regenerate_media_qrcode(media_id, access_url)
            .await
            .unwrap();

        assert_eq!(path1, path2);
        assert!(service.qrcode_exists(&media_id.to_string(), "svg").await);
    }

    #[tokio::test]
    async fn test_delete_qrcode() {
        let temp_dir = TempDir::new().unwrap();
        let service = QRCodeService::new(temp_dir.path());

        let media_id = 789;
        let access_url = "https://example.com/public/media/test-token";

        // 生成二维码
        service
            .generate_media_qrcode(media_id, access_url)
            .await
            .unwrap();

        // 确认文件存在
        assert!(service.qrcode_exists(&media_id.to_string(), "svg").await);

        // 删除二维码
        service.delete_media_qrcode(media_id).await.unwrap();

        // 确认文件不存在
        assert!(!service.qrcode_exists(&media_id.to_string(), "svg").await);
    }
}
