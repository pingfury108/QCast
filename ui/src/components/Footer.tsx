export default function Footer() {
  return (
    <footer className="border-t bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
      <div className="container mx-auto px-4 py-6">
        <div className="flex flex-col md:flex-row items-center justify-between space-y-4 md:space-y-0">
          <div className="flex items-center space-x-2">
            <span className="text-lg">🎵</span>
            <span className="font-semibold">QCast</span>
          </div>
          <div className="flex items-center space-x-6 text-sm text-muted-foreground">
            <span>© 2025 QCast</span>
            <span>·</span>
            <a href="#" className="hover:text-foreground transition-colors">
              关于我们
            </a>
            <span>·</span>
            <a href="#" className="hover:text-foreground transition-colors">
              使用条款
            </a>
            <span>·</span>
            <a href="#" className="hover:text-foreground transition-colors">
              隐私政策
            </a>
          </div>
        </div>
      </div>
    </footer>
  );
}