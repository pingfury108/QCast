import { Link, useLocation } from 'react-router-dom';
import { isAuthenticated, getStoredAuthState } from '../lib/auth';

interface HeaderProps {
  variant?: 'public' | 'auth' | 'dashboard';
}

export default function Header({ variant = 'public' }: HeaderProps) {
  const location = useLocation();
  const { user } = getStoredAuthState();

  if (variant === 'public') {
    return (
      <header className="border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
        <div className="container mx-auto px-4 py-4">
          <div className="flex items-center justify-between">
            <Link to="/" className="flex items-center space-x-2">
              <span className="text-2xl font-bold">🎵 QCast</span>
            </Link>
          </div>
        </div>
      </header>
    );
  }

  if (variant === 'auth') {
    return (
      <header className="border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
        <div className="container mx-auto px-4 py-4">
          <div className="flex items-center justify-between">
            <Link to="/" className="flex items-center space-x-2">
              <span className="text-2xl font-bold">🎵 QCast</span>
            </Link>
            {location.pathname !== '/' && (
              <Link
                to="/"
                className="text-sm text-muted-foreground hover:text-foreground transition-colors"
              >
                返回首页
              </Link>
            )}
          </div>
        </div>
      </header>
    );
  }

  // Dashboard header
  return (
    <header className="border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
      <div className="container mx-auto px-4 py-4">
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-8">
            <Link to="/dashboard" className="flex items-center space-x-2">
              <span className="text-2xl font-bold">🎵 QCast</span>
            </Link>
            <nav className="hidden md:flex items-center space-x-6">
              <Link
                to="/dashboard/books"
                className="text-sm font-medium transition-colors hover:text-foreground/80 text-foreground/60"
              >
                我的书籍
              </Link>
              <Link
                to="/dashboard/upload"
                className="text-sm font-medium transition-colors hover:text-foreground/80 text-foreground/60"
              >
                上传
              </Link>
            </nav>
          </div>
          <div className="flex items-center space-x-4">
            {user && (
              <div className="flex items-center space-x-2">
                <div className="w-8 h-8 rounded-full bg-primary/10 flex items-center justify-center">
                  <span className="text-sm font-medium text-primary">
                    {user.name.charAt(0).toUpperCase()}
                  </span>
                </div>
                <span className="text-sm font-medium">{user.name}</span>
              </div>
            )}
          </div>
        </div>
      </div>
    </header>
  );
}