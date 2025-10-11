import { Outlet, useLocation } from 'react-router-dom';
import Header from './Header';
import Footer from './Footer';
import { cn } from '@/lib/utils';
import { clearAuthState } from '@/lib/auth';
import { useAuthState } from '@/hooks/useAuthState';
import {
  LayoutDashboard,
  Users,
  FolderKanban,
  LogOut,
  Home,
} from 'lucide-react';
import { Link } from 'react-router-dom';
import { Separator } from '@/components/ui/separator';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { Avatar, AvatarFallback } from '@/components/ui/avatar';

const adminNavigation = [
  { name: '仪表盘', href: '/admin', icon: LayoutDashboard },
  { name: '用户管理', href: '/admin/users', icon: Users },
  { name: '用户组管理', href: '/admin/groups', icon: FolderKanban },
];

// Admin 布局组件
function AdminLayout({ children }: { children: React.ReactNode }) {
  const { user } = useAuthState();

  return (
    <div className="min-h-screen bg-gray-50">
      <div className="flex h-screen">
        {/* 侧边栏 */}
        <aside className="hidden w-64 flex-col border-r bg-white md:flex">
          {/* Logo */}
          <div className="flex h-16 items-center border-b px-6">
            <Link to="/admin" className="font-semibold text-lg">
              QCast 管理后台
            </Link>
          </div>

          {/* 导航 */}
          <nav className="flex-1 space-y-1 px-3 py-4">
            {adminNavigation.map((item) => (
              <Link
                key={item.name}
                to={item.href}
                className={cn(
                  'flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium transition-colors',
                  location.pathname === item.href
                    ? 'bg-primary text-primary-foreground'
                    : 'text-muted-foreground hover:bg-gray-100 hover:text-foreground'
                )}
              >
                <item.icon className="h-4 w-4" />
                {item.name}
              </Link>
            ))}
          </nav>

          <Separator />

          {/* 用户信息 */}
          <div className="p-4">
            <DropdownMenu>
              <DropdownMenuTrigger asChild>
                <button className="flex items-center space-x-3 hover:bg-accent rounded-lg p-3 transition-colors w-full">
                  <Avatar className="w-8 h-8">
                    <AvatarFallback className="bg-primary/10 text-primary">
                      {user?.name?.charAt(0).toUpperCase() || 'A'}
                    </AvatarFallback>
                  </Avatar>
                  <div className="flex-1 text-left">
                    <p className="text-sm font-medium truncate">{user?.name || '管理员'}</p>
                    <p className="text-xs text-muted-foreground truncate">
                      {user?.is_superuser ? '超级管理员' : user?.is_staff ? '管理员' : '普通用户'}
                    </p>
                  </div>
                </button>
              </DropdownMenuTrigger>
              <DropdownMenuContent align="end" className="w-56" side="top">
                <div className="flex items-center justify-start gap-2 p-2">
                  <Avatar className="w-8 h-8">
                    <AvatarFallback className="bg-primary/10 text-primary">
                      {user?.name?.charAt(0).toUpperCase() || 'A'}
                    </AvatarFallback>
                  </Avatar>
                  <div className="flex flex-col space-y-1 leading-none">
                    <p className="font-medium">{user?.name || '管理员'}</p>
                    <p className="w-[200px] truncate text-sm text-muted-foreground">
                      {user?.email || '邮箱未设置'}
                    </p>
                    <p className="text-xs text-primary">
                      {user?.is_superuser ? '超级管理员' : user?.is_staff ? '管理员' : '普通用户'}
                    </p>
                  </div>
                </div>
                <DropdownMenuSeparator />
                <DropdownMenuItem asChild className="cursor-pointer">
                  <Link to="/" className="flex items-center w-full">
                    <Home className="mr-2 h-4 w-4" />
                    <span>返回主站</span>
                  </Link>
                </DropdownMenuItem>
                <DropdownMenuSeparator />
                <DropdownMenuItem
                  onClick={() => {
                    clearAuthState();
                    window.location.href = '/login';
                  }}
                  className="cursor-pointer text-red-600 focus:text-red-600"
                >
                  <LogOut className="mr-2 h-4 w-4" />
                  <span>退出登录</span>
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
          </div>
        </aside>

        {/* 主内容区 */}
        <main className="flex-1 overflow-auto">
          {children}
        </main>
      </div>
    </div>
  );
}

export default function Layout() {
  const location = useLocation();
  const isPublicPage = location.pathname.startsWith('/public');
  const isAuthPage = ['/login', '/register', '/forgot-password', '/reset-password'].includes(location.pathname);
  const isAdminLogin = location.pathname === '/admin/login';
  const isAdminPage = location.pathname.startsWith('/admin') && !isAdminLogin;

  // 公开页面和认证页面使用不同的布局
  if (isPublicPage) {
    return (
      <div className="min-h-screen flex flex-col bg-background">
        <Header variant="public" />
        <main className="flex-1">
          <Outlet />
        </main>
        <Footer />
      </div>
    );
  }

  if (isAuthPage || isAdminLogin) {
    return (
      <div className="min-h-screen flex flex-col bg-background">
        <Header variant="auth" />
        <main className="flex-1 flex items-center justify-center py-8">
          <Outlet />
        </main>
        <Footer />
      </div>
    );
  }

  // Admin 布局 - 需要在 AdminAuthProvider 内部使用
  if (isAdminPage) {
    return <AdminLayout><Outlet /></AdminLayout>;
  }

  // 主应用布局
  return (
    <div className="min-h-screen flex flex-col bg-background">
      <Header variant="dashboard" />
      <main className="flex-1">
        <Outlet />
      </main>
      <Footer />
    </div>
  );
}