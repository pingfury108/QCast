import { useState } from 'react';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { Link } from 'react-router-dom';
import { loginSchema, type LoginFormData } from '../lib/validations/auth';
import { useLogin } from '../hooks/useAuth';
import { Alert, AlertDescription } from '@/components/ui/alert';

export default function LoginPage() {
  const login = useLogin();
  const [loginError, setLoginError] = useState<string>('');

  const {
    register,
    handleSubmit,
    formState: { errors, isSubmitting },
  } = useForm<LoginFormData>({
    resolver: zodResolver(loginSchema),
    defaultValues: {
      remember: false,
    },
  });

  const onSubmit = async (data: LoginFormData) => {
    console.log('开始登录:', { email: data.email, hasPassword: !!data.password });
    setLoginError(''); // 清除之前的错误

    try {
      await login.mutateAsync({
        email: data.email,
        password: data.password,
      });
    } catch (error: any) {
      console.error('登录提交失败:', error);
      // 在页面中显示错误信息，不依赖 toast
      const errorMessage = error.response?.data?.message || '登录失败，请检查邮箱和密码';
      setLoginError(errorMessage);
    }
  };

  return (
    <div className="w-full max-w-md mx-auto p-6">
      <div className="text-center mb-8">
        <h1 className="text-3xl font-bold mb-2"> QCast</h1>
        <p className="text-muted-foreground">欢迎回来</p>
      </div>
      <div className="bg-card rounded-lg border p-6">
        <h2 className="text-xl font-semibold mb-6 text-center">登录</h2>

        {/* 错误信息显示 */}
        {loginError && (
          <div className="mb-4">
            <Alert variant="destructive">
              <AlertDescription>{loginError}</AlertDescription>
            </Alert>
          </div>
        )}

        <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
          <div>
            <label className="text-sm font-medium mb-2 block">邮箱</label>
            <input
              type="email"
              {...register('email', {
                onChange: () => setLoginError('') // 用户开始输入时清除错误消息
              })}
              className="w-full px-3 py-2 border rounded-md bg-background focus:outline-none focus:ring-2 focus:ring-primary"
              placeholder="your@email.com"
            />
            {errors.email && (
              <p className="text-sm text-red-500 mt-1">{errors.email.message}</p>
            )}
          </div>
          <div>
            <label className="text-sm font-medium mb-2 block">密码</label>
            <input
              type="password"
              {...register('password', {
                onChange: () => setLoginError('') // 用户开始输入时清除错误消息
              })}
              className="w-full px-3 py-2 border rounded-md bg-background focus:outline-none focus:ring-2 focus:ring-primary"
              placeholder="••••••••"
            />
            {errors.password && (
              <p className="text-sm text-red-500 mt-1">{errors.password.message}</p>
            )}
          </div>
          <div className="flex items-center justify-between">
            <label className="flex items-center space-x-2 cursor-pointer">
              <input
                type="checkbox"
                {...register('remember')}
                className="rounded border-gray-300"
              />
              <span className="text-sm">记住我</span>
            </label>
            <Link
              to="/forgot-password"
              className="text-sm text-primary hover:underline"
            >
              忘记密码？
            </Link>
          </div>
          <button
            type="submit"
            disabled={isSubmitting || login.isPending}
            className="w-full py-2 px-4 bg-primary text-primary-foreground rounded-md font-medium hover:bg-primary/90 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            {isSubmitting || login.isPending ? '登录中...' : '登录'}
          </button>
          <p className="text-center text-sm text-muted-foreground">
            还没有账号？{' '}
            <Link to="/register" className="text-primary hover:underline">
              立即注册
            </Link>
          </p>
        </form>
      </div>
    </div>
  );
}
