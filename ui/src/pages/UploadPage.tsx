export default function UploadPage() {
  return (
    <div className="container mx-auto px-4 py-8">
      <h1 className="text-3xl font-bold mb-8">上传媒体文件</h1>

      <div className="bg-card rounded-lg border p-8 text-center">
        <p className="text-muted-foreground mb-4">
          文件上传功能将在后端 API 完成后实现
        </p>
        <p className="text-sm text-muted-foreground mb-6">
          这里将支持拖拽上传、批量上传和上传进度显示
        </p>
        <div className="border-2 border-dashed rounded-lg p-12 text-center">
          <p className="text-lg mb-2">拖拽文件到此处</p>
          <p className="text-sm text-muted-foreground">或点击选择文件</p>
          <p className="text-xs text-muted-foreground mt-2">
            支持 MP3, MP4, WAV, M4A 等格式
          </p>
        </div>
      </div>
    </div>
  );
}