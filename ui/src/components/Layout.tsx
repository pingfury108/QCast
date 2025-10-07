import { Outlet, useLocation } from 'react-router-dom';
import Header from './Header';
import Footer from './Footer';

export default function Layout() {
  const location = useLocation();
  const isPublicPage = location.pathname.startsWith('/public');
  const isAuthPage = ['/login', '/register', '/forgot-password', '/reset-password'].includes(location.pathname);

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

  if (isAuthPage) {
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