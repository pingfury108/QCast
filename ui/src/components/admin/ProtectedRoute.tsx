import { ReactNode } from 'react';
import { Navigate } from 'react-router-dom';
import { useAuthState } from '@/hooks/useAuthState';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { AlertTriangle, Shield } from 'lucide-react';

interface ProtectedRouteProps {
  children: ReactNode;
}

export function ProtectedRoute({ children }: ProtectedRouteProps) {
  const { user, isLoading } = useAuthState();

  if (isLoading) {
    return (
      <div className="flex h-screen items-center justify-center">
        <div className="text-center">
          <div className="h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent mx-auto mb-4"></div>
          <p className="text-sm text-muted-foreground">加载中...</p>
        </div>
      </div>
    );
  }

  // 如果用户未登录，跳转到登录页面
  if (!user) {
    return <Navigate to="/login" replace />;
  }

  // 如果用户已登录但没有管理员权限，显示权限不足页面
  if (!user.is_staff && !user.is_superuser) {
    return (
      <div className="min-h-screen flex items-center justify-center bg-gray-50 p-4">
        <div className="w-full max-w-md">
          <Card>
            <CardHeader className="text-center">
              <div className="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-full bg-yellow-100">
                <AlertTriangle className="h-6 w-6 text-yellow-600" />
              </div>
              <CardTitle className="text-xl">访问受限</CardTitle>
              <CardDescription>
                您没有权限访问管理后台
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="text-center text-sm text-muted-foreground">
                <p>当前登录用户：<span className="font-medium">{user.name}</span></p>
                <p>用户权限：<span className="font-medium">普通用户</span></p>
              </div>

              <div className="space-y-2">
                <Button
                  asChild
                  className="w-full"
                >
                  <a href="/">返回首页</a>
                </Button>

                <Button
                  variant="outline"
                  asChild
                  className="w-full"
                >
                  <a href="/dashboard">前往用户仪表盘</a>
                </Button>
              </div>

              <div className="text-center text-xs text-muted-foreground">
                <p>如需管理权限，请联系系统管理员</p>
              </div>
            </CardContent>
          </Card>
        </div>
      </div>
    );
  }

  // 用户有管理员权限，正常显示内容
  return <>{children}</>;
}
