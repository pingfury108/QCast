export default function ResetPasswordPage() {
  return (
    <div className="w-full max-w-md mx-auto p-6">
      <div className="text-center mb-8">
        <h1 className="text-3xl font-bold mb-2">🎵 QCast</h1>
        <p className="text-muted-foreground">设置新密码</p>
      </div>
      <div className="bg-card rounded-lg border p-6">
        <h2 className="text-xl font-semibold mb-6 text-center">重置密码</h2>
        <div className="space-y-4">
          <div>
            <label className="text-sm font-medium mb-2 block">新密码</label>
            <input
              type="password"
              className="w-full px-3 py-2 border rounded-md bg-background"
              placeholder="••••••••"
              disabled
            />
            <p className="text-xs text-muted-foreground mt-1">至少 6 位字符</p>
          </div>
          <div>
            <label className="text-sm font-medium mb-2 block">确认新密码</label>
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
            重置密码
          </button>
        </div>
      </div>
    </div>
  );
}