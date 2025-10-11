import { useQuery } from '@tanstack/react-query';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { adminUserApi, adminGroupApi } from '@/services/admin';
import { Users, Shield, FolderKanban } from 'lucide-react';
import { useAuthState } from '@/hooks/useAuthState';
import { Link } from 'react-router-dom';

interface StatCardProps {
  title: string;
  value: number;
  icon: React.ReactNode;
  description?: string;
}

function StatCard({ title, value, icon, description }: StatCardProps) {
  return (
    <Card>
      <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
        <CardTitle className="text-sm font-medium">{title}</CardTitle>
        {icon}
      </CardHeader>
      <CardContent>
        <div className="text-2xl font-bold">{value}</div>
        {description && (
          <p className="text-xs text-muted-foreground">{description}</p>
        )}
      </CardContent>
    </Card>
  );
}

export function AdminDashboardPage() {
  const { user } = useAuthState();

  // 获取用户统计
  const { data: userStats } = useQuery({
    queryKey: ['admin', 'users', 'stats'],
    queryFn: () => adminUserApi.stats(),
  });

  // 获取用户组列表（计算数量）
  const { data: groups } = useQuery({
    queryKey: ['admin', 'groups'],
    queryFn: () => adminGroupApi.list(),
  });

  const groupCount = groups?.length || 0;

  return (
    <div className="p-6">
      {/* 页面标题区域 */}
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-gray-900">仪表盘</h1>
        <p className="text-gray-600 mt-2">
          欢迎回来，{user?.name}
        </p>
      </div>

      {/* 内容区域 */}
      <div className="space-y-6">
        {/* 统计卡片 */}
        <div className="grid gap-6 md:grid-cols-3">
          <StatCard
            title="总用户数"
            value={userStats?.total_users || 0}
            icon={<Users className="h-6 w-6 text-blue-600" />}
            description="系统中的所有用户"
          />

          <StatCard
            title="管理员"
            value={userStats?.total_admins || 0}
            icon={<Shield className="h-6 w-6 text-green-600" />}
            description="具有管理权限的用户"
          />

          <StatCard
            title="用户组"
            value={groupCount}
            icon={<FolderKanban className="h-6 w-6 text-purple-600" />}
            description="已创建的用户组"
          />
        </div>

        {/* 快速操作 */}
        <Card>
          <CardHeader>
            <CardTitle className="text-xl">快速操作</CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="grid gap-4 md:grid-cols-2">
              <Link
                to="/admin/users"
                className="flex items-center gap-4 rounded-lg border p-4 transition-colors hover:bg-gray-50 hover:shadow-sm"
              >
                <div className="flex h-12 w-12 items-center justify-center rounded-lg bg-blue-100">
                  <Users className="h-6 w-6 text-blue-600" />
                </div>
                <div>
                  <p className="font-medium text-lg">管理用户</p>
                  <p className="text-sm text-gray-600">
                    查看和管理所有用户
                  </p>
                </div>
              </Link>

              <Link
                to="/admin/groups"
                className="flex items-center gap-4 rounded-lg border p-4 transition-colors hover:bg-gray-50 hover:shadow-sm"
              >
                <div className="flex h-12 w-12 items-center justify-center rounded-lg bg-purple-100">
                  <FolderKanban className="h-6 w-6 text-purple-600" />
                </div>
                <div>
                  <p className="font-medium text-lg">管理用户组</p>
                  <p className="text-sm text-gray-600">
                    创建和管理用户组
                  </p>
                </div>
              </Link>
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  );
}