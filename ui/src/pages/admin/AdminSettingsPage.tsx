import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import { adminSettingsApi } from '@/services/admin';
import { toast } from 'sonner';
import { Save, AlertCircle } from 'lucide-react';

// 表单验证 schema
const siteSettingsSchema = z.object({
  site_url: z
    .string()
    .min(1, '站点 URL 不能为空')
    .url('请输入有效的 URL')
    .refine(
      (url) => url.startsWith('http://') || url.startsWith('https://'),
      '站点 URL 必须以 http:// 或 https:// 开头'
    ),
});

type SiteSettingsForm = z.infer<typeof siteSettingsSchema>;

export function AdminSettingsPage() {
  const queryClient = useQueryClient();

  // 获取站点设置
  const { data: settings, isLoading } = useQuery({
    queryKey: ['admin', 'settings'],
    queryFn: () => adminSettingsApi.get(),
  });

  // 表单初始化
  const {
    register,
    handleSubmit,
    formState: { errors, isDirty },
    reset,
  } = useForm<SiteSettingsForm>({
    resolver: zodResolver(siteSettingsSchema),
    values: settings
      ? {
          site_url: settings.site_url,
        }
      : undefined,
  });

  // 更新站点设置
  const updateMutation = useMutation({
    mutationFn: (data: SiteSettingsForm) => adminSettingsApi.update(data),
    onSuccess: (data) => {
      queryClient.setQueryData(['admin', 'settings'], data);
      toast.success('站点设置已更新');
      reset(data);
    },
    onError: (error: any) => {
      const message = error.response?.data?.description || '更新失败，请检查输入';
      toast.error(message);
    },
  });

  const onSubmit = (data: SiteSettingsForm) => {
    updateMutation.mutate(data);
  };

  if (isLoading) {
    return (
      <div className="p-6">
        <div className="mb-8">
          <h1 className="text-3xl font-bold text-gray-900">站点设置</h1>
        </div>
        <Card>
          <CardContent className="p-6">
            <p className="text-center text-muted-foreground">加载中...</p>
          </CardContent>
        </Card>
      </div>
    );
  }

  return (
    <div className="p-6">
      {/* 页面标题 */}
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-gray-900">站点设置</h1>
      </div>

      {/* 设置表单 */}
      <Card>
        <CardHeader>
          <CardTitle>基本配置</CardTitle>
        </CardHeader>
        <CardContent>
          <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
            <div className="space-y-2">
              <Label htmlFor="site_url">站点 URL</Label>
              <Input
                id="site_url"
                placeholder="https://example.com"
                {...register('site_url')}
                className={errors.site_url ? 'border-red-500' : ''}
              />
              {errors.site_url && (
                <p className="text-sm text-red-600 flex items-center gap-1">
                  <AlertCircle className="h-3 w-3" />
                  {errors.site_url.message}
                </p>
              )}
            </div>

            <div className="flex gap-2">
              <Button
                type="button"
                variant="outline"
                onClick={() => reset()}
                disabled={!isDirty || updateMutation.isPending}
              >
                取消
              </Button>
              <Button
                type="submit"
                disabled={!isDirty || updateMutation.isPending}
              >
                {updateMutation.isPending ? (
                  '保存中...'
                ) : (
                  <>
                    <Save className="mr-2 h-4 w-4" />
                    保存更改
                  </>
                )}
              </Button>
            </div>
          </form>
        </CardContent>
      </Card>
    </div>
  );
}
