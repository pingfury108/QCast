import { useState } from 'react';
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { useAuthState } from '@/hooks/useAuthState';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
import { Badge } from '@/components/ui/badge';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import { Checkbox } from '@/components/ui/checkbox';
import { Label } from '@/components/ui/label';
import type { User } from '@/types/auth';
import { adminUserApi } from '@/services/admin';
import { MoreHorizontal, Search, Shield, Trash2 } from 'lucide-react';
import { toast } from 'sonner';
import { PermissionGate } from '@/components/admin/PermissionGate';
import { Alert, AlertDescription } from '@/components/ui/alert';

export function AdminUsersPage() {
  const queryClient = useQueryClient();
  const { user: currentUser } = useAuthState();
  const [searchQuery, setSearchQuery] = useState('');
  const [page, setPage] = useState(1);
  const [editingUser, setEditingUser] = useState<User | null>(null);
  const [deletingUser, setDeletingUser] = useState<User | null>(null);

  // 获取用户列表
  const { data: usersData, isLoading } = useQuery({
    queryKey: ['admin', 'users', page, searchQuery],
    queryFn: () =>
      adminUserApi.list({
        page,
        per_page: 20,
        search: searchQuery || undefined,
      }),
  });

  // 更新用户角色
  const updateRoleMutation = useMutation({
    mutationFn: ({ userId, data }: { userId: number; data: { is_staff: boolean; is_superuser: boolean } }) =>
      adminUserApi.updateRole(userId, data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['admin', 'users'] });
      queryClient.invalidateQueries({ queryKey: ['admin', 'users', 'stats'] });
      toast.success('用户角色已更新');
      setEditingUser(null);
    },
    onError: (error: any) => {
      toast.error(error.response?.data?.description || '更新失败');
    },
  });

  // 删除用户
  const deleteMutation = useMutation({
    mutationFn: (userId: number) => adminUserApi.delete(userId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['admin', 'users'] });
      queryClient.invalidateQueries({ queryKey: ['admin', 'users', 'stats'] });
      toast.success('用户已删除');
      setDeletingUser(null);
    },
    onError: (error: any) => {
      toast.error(error.response?.data?.description || '删除失败');
    },
  });

  const handleUpdateRole = () => {
    if (!editingUser) return;
    updateRoleMutation.mutate({
      userId: editingUser.id,
      data: {
        is_staff: editingUser.is_staff,
        is_superuser: editingUser.is_superuser,
      },
    });
  };

  const handleDelete = () => {
    if (!deletingUser) return;
    deleteMutation.mutate(deletingUser.id);
  };

  const getRoleBadge = (user: User) => {
    if (user.is_superuser) {
      return <Badge className="bg-purple-500">超级管理员</Badge>;
    }
    if (user.is_staff) {
      return <Badge variant="secondary">员工</Badge>;
    }
    return <Badge variant="outline">普通用户</Badge>;
  };

  return (
    <div className="p-6">
      {/* 页面标题区域 */}
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-gray-900">用户管理</h1>
        <p className="text-gray-600 mt-2">管理系统中的所有用户</p>
      </div>

      {/* 内容区域 */}
      <div className="space-y-6">
        {/* 搜索栏 */}
        <div className="flex items-center gap-4">
          <div className="relative flex-1">
            <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
            <Input
              placeholder="搜索邮箱或用户名..."
              value={searchQuery}
              onChange={(e) => {
                setSearchQuery(e.target.value);
                setPage(1);
              }}
              className="pl-10"
            />
          </div>
        </div>

        {/* 用户表格 */}
        <div className="rounded-md border">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>用户</TableHead>
                <TableHead>邮箱</TableHead>
                <TableHead>角色</TableHead>
                <TableHead>注册时间</TableHead>
                <TableHead className="w-[70px]"></TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {isLoading ? (
                <TableRow>
                  <TableCell colSpan={5} className="text-center">
                    加载中...
                  </TableCell>
                </TableRow>
              ) : usersData?.users.length === 0 ? (
                <TableRow>
                  <TableCell colSpan={5} className="text-center text-muted-foreground">
                    {searchQuery ? '未找到匹配的用户' : '暂无用户'}
                  </TableCell>
                </TableRow>
              ) : (
                usersData?.users.map((user) => (
                  <TableRow key={user.id}>
                    <TableCell className="font-medium">{user.name}</TableCell>
                    <TableCell>{user.email}</TableCell>
                    <TableCell>{getRoleBadge(user)}</TableCell>
                    <TableCell>
                      {new Date(user.created_at).toLocaleDateString('zh-CN')}
                    </TableCell>
                    <TableCell>
                      <DropdownMenu>
                        <DropdownMenuTrigger asChild>
                          <Button variant="ghost" size="icon">
                            <MoreHorizontal className="h-4 w-4" />
                          </Button>
                        </DropdownMenuTrigger>
                        <DropdownMenuContent align="end">
                          <PermissionGate requireSuperAdmin>
                            <DropdownMenuItem
                              onClick={() => setEditingUser(user)}
                            >
                              <Shield className="mr-2 h-4 w-4" />
                              编辑角色
                            </DropdownMenuItem>
                          </PermissionGate>
                          <PermissionGate requireSuperAdmin>
                            <DropdownMenuItem
                              onClick={() => setDeletingUser(user)}
                              disabled={user.id === currentUser?.id}
                              className="text-red-600"
                            >
                              <Trash2 className="mr-2 h-4 w-4" />
                              删除用户
                            </DropdownMenuItem>
                          </PermissionGate>
                        </DropdownMenuContent>
                      </DropdownMenu>
                    </TableCell>
                  </TableRow>
                ))
              )}
            </TableBody>
          </Table>
        </div>

        {/* 分页 */}
        {usersData && usersData.pagination.total_pages > 1 && (
          <div className="flex items-center justify-between">
            <p className="text-sm text-muted-foreground">
              第 {usersData.pagination.page} 页，共 {usersData.pagination.total_pages} 页
            </p>
            <div className="flex gap-2">
              <Button
                variant="outline"
                size="sm"
                onClick={() => setPage((p) => Math.max(1, p - 1))}
                disabled={page === 1}
              >
                上一页
              </Button>
              <Button
                variant="outline"
                size="sm"
                onClick={() => setPage((p) => p + 1)}
                disabled={page >= usersData.pagination.total_pages}
              >
                下一页
              </Button>
            </div>
          </div>
        )}

        {/* 编辑角色对话框 */}
        <Dialog open={!!editingUser} onOpenChange={() => setEditingUser(null)}>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>编辑用户角色</DialogTitle>
              <DialogDescription>
                修改 {editingUser?.name} 的权限设置
              </DialogDescription>
            </DialogHeader>

            <div className="space-y-4 py-4">
              <Alert>
                <Shield className="h-4 w-4" />
                <AlertDescription>
                  超级管理员拥有所有权限，包括删除用户和修改角色
                </AlertDescription>
              </Alert>

              <div className="flex items-center space-x-2">
                <Checkbox
                  id="is_staff"
                  checked={editingUser?.is_staff}
                  onCheckedChange={(checked) =>
                    setEditingUser((prev) =>
                      prev ? { ...prev, is_staff: checked as boolean } : null
                    )
                  }
                />
                <Label htmlFor="is_staff" className="cursor-pointer">
                  员工权限 (is_staff)
                </Label>
              </div>

              <div className="flex items-center space-x-2">
                <Checkbox
                  id="is_superuser"
                  checked={editingUser?.is_superuser}
                  onCheckedChange={(checked) =>
                    setEditingUser((prev) =>
                      prev ? { ...prev, is_superuser: checked as boolean } : null
                    )
                  }
                />
                <Label htmlFor="is_superuser" className="cursor-pointer">
                  超级管理员 (is_superuser)
                </Label>
              </div>
            </div>

            <DialogFooter>
              <Button variant="outline" onClick={() => setEditingUser(null)}>
                取消
              </Button>
              <Button onClick={handleUpdateRole} disabled={updateRoleMutation.isPending}>
                {updateRoleMutation.isPending ? '保存中...' : '保存'}
              </Button>
            </DialogFooter>
          </DialogContent>
        </Dialog>

        {/* 删除确认对话框 */}
        <Dialog open={!!deletingUser} onOpenChange={() => setDeletingUser(null)}>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>确认删除用户？</DialogTitle>
              <DialogDescription>
                您确定要删除用户 "{deletingUser?.name}" 吗？此操作不可撤销。
              </DialogDescription>
            </DialogHeader>

            <DialogFooter>
              <Button variant="outline" onClick={() => setDeletingUser(null)}>
                取消
              </Button>
              <Button
                variant="destructive"
                onClick={handleDelete}
                disabled={deleteMutation.isPending}
              >
                {deleteMutation.isPending ? '删除中...' : '确认删除'}
              </Button>
            </DialogFooter>
          </DialogContent>
        </Dialog>
      </div>
    </div>
  );
}