export default function PublicMediaPage() {
  return (
    <div className="min-h-screen bg-background">
      <div className="container mx-auto px-4 py-8 max-w-2xl">
        <div className="bg-card rounded-lg border p-8 text-center">
          <p className="text-muted-foreground mb-4">
            公开访问页面将在后端 API 完成后实现
          </p>
          <p className="text-sm text-muted-foreground">
            这里将显示媒体播放器、二维码和分享功能
          </p>
        </div>
      </div>
    </div>
  );
}