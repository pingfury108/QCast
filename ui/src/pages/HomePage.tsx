import { Link } from 'react-router-dom';
import { isAuthenticated } from '../lib/auth';

export default function HomePage() {
  const authenticated = isAuthenticated();

  // 如果已登录，自动跳转到 dashboard
  if (authenticated) {
    window.location.href = '/dashboard';
    return null;
  }

  return (
    <div className="min-h-screen">
      {/* Hero Section */}
      <section className="py-20 px-4">
        <div className="container mx-auto max-w-4xl text-center">
          <h1 className="text-4xl md:text-6xl font-bold mb-6">
            🎵 QCast
            <span className="block text-2xl md:text-3xl font-normal text-muted-foreground mt-2">
              音视频托管平台
            </span>
          </h1>
          <p className="text-xl text-muted-foreground mb-8 max-w-2xl mx-auto">
            轻松托管你的音频和视频，随时随地分享
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
              <h3 className="text-xl font-semibold mb-2">上传媒体</h3>
              <p className="text-muted-foreground">
                快速上传音频、视频
                支持批量操作
              </p>
            </div>
            <div className="text-center">
              <div className="text-2xl mb-4">🔗</div>
              <h3 className="text-xl font-semibold mb-2">永久链接</h3>
              <p className="text-muted-foreground">
                生成永久访问链接
                链接永不过期
              </p>
            </div>
            <div className="text-center">
              <div className="text-2xl mb-4">📱</div>
              <h3 className="text-xl font-semibold mb-2">二维码分享</h3>
              <p className="text-muted-foreground">
                自动生成二维码
                扫码即可播放
              </p>
            </div>
            <div className="text-center">
              <div className="text-2xl mb-4">📚</div>
              <h3 className="text-xl font-semibold mb-2">层级管理</h3>
              <p className="text-muted-foreground">
                Book/Chapter/Media
                结构化组织内容
              </p>
            </div>
          </div>
        </div>
      </section>

      {/* Use Cases Section */}
      <section className="py-16 px-4">
        <div className="container mx-auto max-w-4xl">
          <h2 className="text-3xl font-bold text-center mb-12">使用场景</h2>
          <div className="grid md:grid-cols-2 gap-8">
            <div className="text-center">
              <div className="text-2xl mb-4">🎙️</div>
              <h3 className="text-xl font-semibold mb-2">播客制作者</h3>
              <p className="text-muted-foreground">
                托管你的播客系列
                管理你的有声书章节
              </p>
            </div>
            <div className="text-center">
              <div className="text-2xl mb-4">📖</div>
              <h3 className="text-xl font-semibold mb-2">有声书作者</h3>
              <p className="text-muted-foreground">
                管理你的有声书章节
                分享给听众
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
            创建你的第一个音视频托管项目
          </p>
          <Link
            to="/register"
            className="inline-flex px-8 py-3 bg-background text-foreground rounded-lg font-medium hover:bg-background/90 transition-colors"
          >
            立即开始
          </Link>
        </div>
      </section>
    </div>
  );
}