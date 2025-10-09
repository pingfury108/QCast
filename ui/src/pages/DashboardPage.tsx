import { Link } from 'react-router-dom';

export default function DashboardPage() {
  return (
    <div className="container mx-auto px-4 py-8">
      <div className="text-center mb-8">
        <h1 className="text-3xl font-bold mb-4">控制台</h1>
        <p className="text-muted-foreground">
          欢迎来到 QCast 控制台
        </p>
      </div>

      <div className="grid md:grid-cols-2 gap-6 max-w-4xl mx-auto">
        <Link
          to="/dashboard/books"
          className="p-6 border rounded-lg hover:border-primary transition-colors"
        >
          <div className="text-2xl mb-4">📚</div>
          <h3 className="text-lg font-semibold mb-2">我的书籍</h3>
          <p className="text-sm text-muted-foreground">
            管理你的书籍、专辑和播客系列
          </p>
        </Link>

        <Link
          to="/dashboard/upload"
          className="p-6 border rounded-lg hover:border-primary transition-colors"
        >
          <div className="text-2xl mb-4">📤</div>
          <h3 className="text-lg font-semibold mb-2">上传媒体</h3>
          <p className="text-sm text-muted-foreground">
            上传音频和视频文件
          </p>
        </Link>
      </div>
    </div>
  );
}