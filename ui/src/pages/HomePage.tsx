import { Link } from 'react-router-dom';

export default function HomePage() {
  return (
    <div className="min-h-screen">
      {/* Hero Section */}
      <section className="py-20 px-4">
        <div className="container mx-auto max-w-4xl text-center">
          <h1 className="text-4xl md:text-6xl font-bold mb-6">
             QCast
            <span className="block text-2xl md:text-3xl font-normal text-muted-foreground mt-2">
              印刷物融媒体平台
            </span>
          </h1>
          <p className="text-xl text-muted-foreground mb-8 max-w-2xl mx-auto">
            为书籍、教材、杂志嵌入音视频内容，让印刷物"开口说话"
          </p>
          <div className="flex flex-col sm:flex-row gap-4 justify-center">
            <Link
              to="/register"
              className="px-8 py-3 bg-primary text-primary-foreground rounded-lg font-medium hover:bg-primary/90 transition-colors"
            >
              开始使用
            </Link>
            <Link
              to="/login"
              className="px-8 py-3 border border-input bg-background hover:bg-accent hover:text-accent-foreground rounded-lg font-medium transition-colors"
            >
              了解更多
            </Link>
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section className="py-16 px-4 bg-muted/50">
        <div className="container mx-auto max-w-4xl">
          <h2 className="text-3xl font-bold text-center mb-12">核心功能</h2>
          <div className="grid md:grid-cols-2 gap-8">
            <div className="text-center">
              <div className="text-2xl mb-4">📤</div>
              <h3 className="text-xl font-semibold mb-2">媒体托管</h3>
              <p className="text-muted-foreground">
                上传音频、视频文件
                支持批量管理
              </p>
            </div>
            <div className="text-center">
              <div className="text-2xl mb-4">📱</div>
              <h3 className="text-xl font-semibold mb-2">二维码生成</h3>
              <p className="text-muted-foreground">
                自动生成访问二维码
                批量导出到Excel
              </p>
            </div>
            <div className="text-center">
              <div className="text-2xl mb-4">🔗</div>
              <h3 className="text-xl font-semibold mb-2">永久链接</h3>
              <p className="text-muted-foreground">
                生成永不过期链接
                适配印刷品生命周期
              </p>
            </div>
            <div className="text-center">
              <div className="text-2xl mb-4">📚</div>
              <h3 className="text-xl font-semibold mb-2">层级管理</h3>
              <p className="text-muted-foreground">
                书籍/章节/媒体结构
                匹配出版物组织方式
              </p>
            </div>
          </div>
        </div>
      </section>

      {/* Use Cases Section */}
      <section className="py-16 px-4">
        <div className="container mx-auto max-w-4xl">
          <h2 className="text-3xl font-bold text-center mb-12">应用场景</h2>
          <div className="grid md:grid-cols-2 gap-8">
            <div className="text-center">
              <div className="text-2xl mb-4">📚</div>
              <h3 className="text-xl font-semibold mb-2">教育出版</h3>
              <p className="text-muted-foreground">
                教材配套音视频讲解
                英语听力练习音频
              </p>
            </div>
            <div className="text-center">
              <div className="text-2xl mb-4">📖</div>
              <h3 className="text-xl font-semibold mb-2">图书出版</h3>
              <p className="text-muted-foreground">
                儿童绘本配音朗读
                有声书章节音频
              </p>
            </div>
            <div className="text-center">
              <div className="text-2xl mb-4">📰</div>
              <h3 className="text-xl font-semibold mb-2">期刊杂志</h3>
              <p className="text-muted-foreground">
                专家访谈音频
                深度报道补充内容
              </p>
            </div>
            <div className="text-center">
              <div className="text-2xl mb-4">🎨</div>
              <h3 className="text-xl font-semibold mb-2">文创产品</h3>
              <p className="text-muted-foreground">
                艺术作品解说
                博物馆导览音频
              </p>
            </div>
          </div>
        </div>
      </section>

      {/* CTA Section */}
      <section className="py-16 px-4 bg-primary text-primary-foreground">
        <div className="container mx-auto max-w-4xl text-center">
          <h2 className="text-3xl font-bold mb-4">立即开始</h2>
          <p className="text-lg mb-8 opacity-90">
            让您的印刷物"开口说话"
          </p>
          <Link
            to="/register"
            className="inline-flex px-8 py-3 bg-background text-foreground rounded-lg font-medium hover:bg-background/90 transition-colors"
          >
            免费注册
          </Link>
        </div>
      </section>
    </div>
  );
}