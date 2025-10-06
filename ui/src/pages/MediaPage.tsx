export default function MediaPage() {
  return (
    <div className="container mx-auto px-4 py-8">
      <div className="flex items-center justify-between mb-8">
        <h1 className="text-3xl font-bold">所有媒体</h1>
        <button
          className="px-4 py-2 bg-primary text-primary-foreground rounded-md font-medium opacity-50 cursor-not-allowed"
          disabled
        >
          + 上传媒体
        </button>
      </div>

      <div className="bg-card rounded-lg border p-8 text-center">
        <p className="text-muted-foreground mb-4">
          媒体管理功能将在后端 API 完成后实现
        </p>
        <p className="text-sm text-muted-foreground">
          这里将显示所有媒体文件的列表，支持搜索和筛选
        </p>
      </div>
    </div>
  );
}