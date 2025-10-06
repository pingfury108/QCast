export default function ForgotPasswordPage() {
  return (
    <div className="w-full max-w-md mx-auto p-6">
      <div className="text-center mb-8">
        <h1 className="text-3xl font-bold mb-2">🎵 QCast</h1>
        <p className="text-muted-foreground">重置密码</p>
      </div>
      <div className="bg-card rounded-lg border p-6">
        <h2 className="text-xl font-semibold mb-6 text-center">找回密码</h2>
        <p className="text-center text-muted-foreground mb-4">
          请输入你的注册邮箱，我们将发送重置密码链接到你的邮箱。
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
          <button
            className="w-full py-2 px-4 bg-primary text-primary-foreground rounded-md font-medium opacity-50 cursor-not-allowed"
            disabled
          >
            发送重置链接
          </button>
        </div>
      </div>
    </div>
  );
}