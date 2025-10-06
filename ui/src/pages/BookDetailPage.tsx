export default function BookDetailPage() {
  return (
    <div className="container mx-auto px-4 py-8">
      <div className="mb-6">
        <button
          className="text-sm text-muted-foreground hover:text-foreground transition-colors"
          onClick={() => window.history.back()}
        >
          ← 返回书籍列表
        </button>
      </div>

      <div className="bg-card rounded-lg border p-8 text-center">
        <p className="text-muted-foreground mb-4">
          书籍详情页面将在后端 API 完成后实现
        </p>
        <p className="text-sm text-muted-foreground">
          这里将显示书籍的详细信息、章节和媒体文件
        </p>
      </div>
    </div>
  );
}