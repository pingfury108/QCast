export default function LoginPage() {
  return (
    <div className="w-full max-w-md mx-auto p-6">
      <div className="text-center mb-8">
        <h1 className="text-3xl font-bold mb-2">🎵 QCast</h1>
        <p className="text-muted-foreground">欢迎回来</p>
      </div>
      <div className="bg-card rounded-lg border p-6">
        <h2 className="text-xl font-semibold mb-6 text-center">登录</h2>
        <p className="text-center text-muted-foreground mb-4">
          登录功能将在后端认证 API 完成后实现
        </p>
        <div className="space-y-4">
          <div>
            <label className="text-sm font-medium mb-2 block">邮箱</label>
            <input
              type="email"
              className="w-full px-3 py-2 border rounded-md bg-background"
              placeholder="your@email.com"
              disabled
            />
          </div>
          <div>
            <label className="text-sm font-medium mb-2 block">密码</label>
            <input
              type="password"
              className="w-full px-3 py-2 border rounded-md bg-background"
              placeholder="••••••••"
              disabled
            />
          </div>
          <button
            className="w-full py-2 px-4 bg-primary text-primary-foreground rounded-md font-medium opacity-50 cursor-not-allowed"
            disabled
          >
            登录
          </button>
        </div>
      </div>
    </div>
  );
}