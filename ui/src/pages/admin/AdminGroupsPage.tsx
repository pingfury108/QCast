import { useState } from 'react';
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Textarea } from '@/components/ui/textarea';
import {
  Card,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
import { Badge } from '@/components/ui/badge';
import { Label } from '@/components/ui/label';
import type { Group } from '@/services/admin';
import { adminGroupApi, adminUserApi } from '@/services/admin';
import { FolderKanban, Plus, Trash2, Users, UserPlus, UserMinus } from 'lucide-react';
import { toast } from 'sonner';
import { PermissionGate } from '@/components/admin/PermissionGate';

export function AdminGroupsPage() {
  const queryClient = useQueryClient();
  const [createDialogOpen, setCreateDialogOpen] = useState(false);
  const [selectedGroup, setSelectedGroup] = useState<Group | null>(null);
  const [deletingGroup, setDeletingGroup] = useState<Group | null>(null);
  const [addMemberDialogOpen, setAddMemberDialogOpen] = useState(false);
  const [newGroupName, setNewGroupName] = useState('');
  const [newGroupDesc, setNewGroupDesc] = useState('');
  const [searchEmail, setSearchEmail] = useState('');

  // 获取用户组列表
  const { data: groups, isLoading } = useQuery({
    queryKey: ['admin', 'groups'],
    queryFn: () => adminGroupApi.list(),
  });

  // 获取选中用户组的详情
  const { data: groupDetail } = useQuery({
    queryKey: ['admin', 'groups', selectedGroup?.id],
    queryFn: () => adminGroupApi.get(selectedGroup!.id),
    enabled: !!selectedGroup,
  });

  // 搜索用户（用于添加成员）
  const { data: searchResults, refetch: searchUsers } = useQuery({
    queryKey: ['admin', 'users', 'search', searchEmail],
    queryFn: () =>
      adminUserApi.list({
        page: 1,
        per_page: 10,
        search: searchEmail,
      }),
    enabled: false,
  });

  // 创建用户组
  const createMutation = useMutation({
    mutationFn: (data: { name: string; description?: string }) =>
      adminGroupApi.create(data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['admin', 'groups'] });
      toast.success('用户组已创建');
      setCreateDialogOpen(false);
      setNewGroupName('');
      setNewGroupDesc('');
    },
    onError: (error: any) => {
      toast.error(error.response?.data?.description || '创建失败');
    },
  });

  // 删除用户组
  const deleteMutation = useMutation({
    mutationFn: (groupId: number) => adminGroupApi.delete(groupId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['admin', 'groups'] });
      toast.success('用户组已删除');
      setDeletingGroup(null);
      setSelectedGroup(null);
    },
    onError: (error: any) => {
      toast.error(error.response?.data?.description || '删除失败');
    },
  });

  // 添加成员
  const addMemberMutation = useMutation({
    mutationFn: ({ groupId, userId }: { groupId: number; userId: number }) =>
      adminGroupApi.addMember(groupId, userId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['admin', 'groups'] });
      toast.success('成员已添加');
      setSearchEmail('');
    },
    onError: (error: any) => {
      toast.error(error.response?.data?.description || '添加失败');
    },
  });

  // 移除成员
  const removeMemberMutation = useMutation({
    mutationFn: ({ groupId, userId }: { groupId: number; userId: number }) =>
      adminGroupApi.removeMember(groupId, userId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['admin', 'groups'] });
      toast.success('成员已移除');
    },
    onError: (error: any) => {
      toast.error(error.response?.data?.description || '移除失败');
    },
  });

  const handleCreateGroup = () => {
    if (!newGroupName.trim()) {
      toast.error('请输入用户组名称');
      return;
    }
    createMutation.mutate({
      name: newGroupName,
      description: newGroupDesc || undefined,
    });
  };

  const handleDeleteGroup = () => {
    if (!deletingGroup) return;
    deleteMutation.mutate(deletingGroup.id);
  };

  const handleAddMember = (userId: number) => {
    if (!selectedGroup) return;
    addMemberMutation.mutate({ groupId: selectedGroup.id, userId });
  };

  const handleRemoveMember = (userId: number) => {
    if (!selectedGroup) return;
    removeMemberMutation.mutate({ groupId: selectedGroup.id, userId });
  };

  const handleSearchUsers = () => {
    if (!searchEmail.trim()) {
      toast.error('请输入邮箱进行搜索');
      return;
    }
    searchUsers();
  };

  return (
    <div className="space-y-6 p-6">
        {/* 标题和操作 */}
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-3xl font-bold">用户组管理</h1>
            <p className="text-muted-foreground">管理用户组和成员</p>
          </div>
          <PermissionGate requireSuperAdmin>
            <Button onClick={() => setCreateDialogOpen(true)}>
              <Plus className="mr-2 h-4 w-4" />
              创建用户组
            </Button>
          </PermissionGate>
        </div>

        {/* 用户组列表 */}
        <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
          {isLoading ? (
            <div className="col-span-full text-center py-8 text-muted-foreground">
              加载中...
            </div>
          ) : groups?.length === 0 ? (
            <div className="col-span-full text-center py-8 text-muted-foreground">
              暂无用户组
            </div>
          ) : (
            groups?.map((group) => (
              <Card
                key={group.id}
                className="cursor-pointer hover:shadow-md transition-shadow"
                onClick={() => setSelectedGroup(group)}
              >
                <CardHeader>
                  <div className="flex items-start justify-between">
                    <div className="flex items-center gap-2">
                      <FolderKanban className="h-5 w-5 text-muted-foreground" />
                      <CardTitle className="text-lg">{group.name}</CardTitle>
                    </div>
                    <Badge variant="secondary">
                      <Users className="mr-1 h-3 w-3" />
                      {group.member_count || 0}
                    </Badge>
                  </div>
                  {group.description && (
                    <CardDescription className="line-clamp-2">
                      {group.description}
                    </CardDescription>
                  )}
                </CardHeader>
              </Card>
            ))
          )}
        </div>

      {/* 创建用户组对话框 */}
      <Dialog open={createDialogOpen} onOpenChange={setCreateDialogOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>创建用户组</DialogTitle>
            <DialogDescription>创建一个新的用户组来管理用户</DialogDescription>
          </DialogHeader>

          <div className="space-y-4 py-4">
            <div className="space-y-2">
              <Label htmlFor="name">用户组名称 *</Label>
              <Input
                id="name"
                placeholder="输入用户组名称"
                value={newGroupName}
                onChange={(e) => setNewGroupName(e.target.value)}
              />
            </div>

            <div className="space-y-2">
              <Label htmlFor="description">描述（可选）</Label>
              <Textarea
                id="description"
                placeholder="输入用户组描述"
                value={newGroupDesc}
                onChange={(e) => setNewGroupDesc(e.target.value)}
                rows={3}
              />
            </div>
          </div>

          <DialogFooter>
            <Button variant="outline" onClick={() => setCreateDialogOpen(false)}>
              取消
            </Button>
            <Button onClick={handleCreateGroup} disabled={createMutation.isPending}>
              {createMutation.isPending ? '创建中...' : '创建'}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* 用户组详情对话框 */}
      <Dialog open={!!selectedGroup} onOpenChange={() => setSelectedGroup(null)}>
        <DialogContent className="max-w-2xl max-h-[80vh] overflow-y-auto">
          <DialogHeader>
            <DialogTitle className="flex items-center gap-2">
              <FolderKanban className="h-5 w-5" />
              {selectedGroup?.name}
            </DialogTitle>
            {selectedGroup?.description && (
              <DialogDescription>{selectedGroup.description}</DialogDescription>
            )}
          </DialogHeader>

          <div className="space-y-4">
            {/* 成员列表 */}
            <div>
              <div className="flex items-center justify-between mb-3">
                <h3 className="text-sm font-medium">
                  成员列表 ({groupDetail?.members.length || 0})
                </h3>
                <PermissionGate requireSuperAdmin>
                  <Button
                    size="sm"
                    variant="outline"
                    onClick={() => setAddMemberDialogOpen(true)}
                  >
                    <UserPlus className="mr-2 h-4 w-4" />
                    添加成员
                  </Button>
                </PermissionGate>
              </div>

              {groupDetail?.members.length === 0 ? (
                <div className="text-center py-4 text-sm text-muted-foreground border rounded-md">
                  暂无成员
                </div>
              ) : (
                <div className="border rounded-md">
                  <Table>
                    <TableHeader>
                      <TableRow>
                        <TableHead>用户名</TableHead>
                        <TableHead>邮箱</TableHead>
                        <TableHead className="w-[70px]"></TableHead>
                      </TableRow>
                    </TableHeader>
                    <TableBody>
                      {groupDetail?.members.map((member) => (
                        <TableRow key={member.id}>
                          <TableCell className="font-medium">{member.name}</TableCell>
                          <TableCell>{member.email}</TableCell>
                          <TableCell>
                            <PermissionGate requireSuperAdmin>
                              <Button
                                size="icon"
                                variant="ghost"
                                onClick={() => handleRemoveMember(member.id)}
                                disabled={removeMemberMutation.isPending}
                              >
                                <UserMinus className="h-4 w-4 text-red-600" />
                              </Button>
                            </PermissionGate>
                          </TableCell>
                        </TableRow>
                      ))}
                    </TableBody>
                  </Table>
                </div>
              )}
            </div>
          </div>

          <DialogFooter>
            <Button variant="outline" onClick={() => setSelectedGroup(null)}>
              关闭
            </Button>
            <PermissionGate requireSuperAdmin>
              <Button
                variant="destructive"
                onClick={() => {
                  if (selectedGroup) {
                    setDeletingGroup(selectedGroup);
                  }
                  setSelectedGroup(null);
                }}
              >
                <Trash2 className="mr-2 h-4 w-4" />
                删除用户组
              </Button>
            </PermissionGate>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* 添加成员对话框 */}
      <Dialog open={addMemberDialogOpen} onOpenChange={setAddMemberDialogOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>添加成员</DialogTitle>
            <DialogDescription>
              搜索并添加用户到 "{selectedGroup?.name}"
            </DialogDescription>
          </DialogHeader>

          <div className="space-y-4 py-4">
            <div className="flex gap-2">
              <Input
                placeholder="输入用户邮箱搜索"
                value={searchEmail}
                onChange={(e) => setSearchEmail(e.target.value)}
                onKeyDown={(e) => e.key === 'Enter' && handleSearchUsers()}
              />
              <Button onClick={handleSearchUsers}>搜索</Button>
            </div>

            {searchResults && (
              <div className="border rounded-md">
                {searchResults.users.length === 0 ? (
                  <div className="text-center py-4 text-sm text-muted-foreground">
                    未找到匹配的用户
                  </div>
                ) : (
                  <Table>
                    <TableBody>
                      {searchResults.users.map((user) => {
                        const isMember = groupDetail?.members.some(
                          (m) => m.id === user.id
                        );
                        return (
                          <TableRow key={user.id}>
                            <TableCell className="font-medium">{user.name}</TableCell>
                            <TableCell>{user.email}</TableCell>
                            <TableCell className="text-right">
                              <Button
                                size="sm"
                                variant={isMember ? 'outline' : 'default'}
                                onClick={() => handleAddMember(user.id)}
                                disabled={isMember || addMemberMutation.isPending}
                              >
                                {isMember ? '已添加' : '添加'}
                              </Button>
                            </TableCell>
                          </TableRow>
                        );
                      })}
                    </TableBody>
                  </Table>
                )}
              </div>
            )}
          </div>

          <DialogFooter>
            <Button variant="outline" onClick={() => setAddMemberDialogOpen(false)}>
              关闭
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* 删除确认对话框 */}
      <Dialog open={!!deletingGroup} onOpenChange={() => setDeletingGroup(null)}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>确认删除用户组？</DialogTitle>
            <DialogDescription>
              您确定要删除用户组 "{deletingGroup?.name}" 吗？此操作不可撤销。
            </DialogDescription>
          </DialogHeader>

          <DialogFooter>
            <Button variant="outline" onClick={() => setDeletingGroup(null)}>
              取消
            </Button>
            <Button
              variant="destructive"
              onClick={handleDeleteGroup}
              disabled={deleteMutation.isPending}
            >
              {deleteMutation.isPending ? '删除中...' : '确认删除'}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
      </div>
    );
}
