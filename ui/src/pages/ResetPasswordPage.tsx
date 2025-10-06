import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { useParams } from 'react-router-dom';
import { resetPasswordSchema, type ResetPasswordFormData } from '../lib/validations/auth';
import { useResetPassword } from '../hooks/useAuth';

export default function ResetPasswordPage() {
  const { token } = useParams<{ token: string }>();
  const {
    register,
    handleSubmit,
    formState: { errors, isSubmitting },
  } = useForm<ResetPasswordFormData>({
    resolver: zodResolver(resetPasswordSchema),
  });

  const { mutate: resetPassword, isPending } = useResetPassword();

  const onSubmit = async (data: ResetPasswordFormData) => {
    if (token) {
      resetPassword({
        token,
        password: data.password,
      });
    }
  };

  return (
    <div className="w-full max-w-md mx-auto p-6">
      <div className="text-center mb-8">
        <h1 className="text-3xl font-bold mb-2"> QCast</h1>
        <p className="text-muted-foreground">设置新密码</p>
      </div>
      <div className="bg-card rounded-lg border p-6">
        <h2 className="text-xl font-semibold mb-6 text-center">重置密码</h2>
        <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
          <div>
            <label className="text-sm font-medium mb-2 block">新密码</label>
            <input
              type="password"
              {...register('password')}
              className="w-full px-3 py-2 border rounded-md bg-background focus:outline-none focus:ring-2 focus:ring-primary"
              placeholder="••••••••"
            />
            {errors.password && (
              <p className="text-sm text-red-500 mt-1">{errors.password.message}</p>
            )}
            <p className="text-xs text-muted-foreground mt-1">至少 6 位字符</p>
          </div>
          <div>
            <label className="text-sm font-medium mb-2 block">确认新密码</label>
            <input
              type="password"
              {...register('confirmPassword')}
              className="w-full px-3 py-2 border rounded-md bg-background focus:outline-none focus:ring-2 focus:ring-primary"
              placeholder="••••••••"
            />
            {errors.confirmPassword && (
              <p className="text-sm text-red-500 mt-1">
                {errors.confirmPassword.message}
              </p>
            )}
          </div>
          <button
            type="submit"
            disabled={isSubmitting || isPending}
            className="w-full py-2 px-4 bg-primary text-primary-foreground rounded-md font-medium hover:bg-primary/90 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            {isSubmitting || isPending ? '重置中...' : '重置密码'}
          </button>
        </form>
      </div>
    </div>
  );
}
