import { z } from 'zod';

// 登录表单验证
export const loginSchema = z.object({
  email: z.string().email('请输入有效的邮箱地址'),
  password: z.string().min(6, '密码至少 6 位'),
  remember: z.boolean().optional(),
});

export type LoginFormData = z.infer<typeof loginSchema>;

// 注册表单验证
export const registerSchema = z
  .object({
    name: z.string().min(2, '用户名至少 2 位').max(50, '用户名最多 50 位'),
    email: z.string().email('请输入有效的邮箱地址'),
    password: z.string().min(6, '密码至少 6 位').max(100, '密码最多 100 位'),
    confirmPassword: z.string(),
  })
  .refine((data) => data.password === data.confirmPassword, {
    message: '两次密码不一致',
    path: ['confirmPassword'],
  });

export type RegisterFormData = z.infer<typeof registerSchema>;

// 忘记密码表单验证
export const forgotPasswordSchema = z.object({
  email: z.string().email('请输入有效的邮箱地址'),
});

export type ForgotPasswordFormData = z.infer<typeof forgotPasswordSchema>;

// 重置密码表单验证
export const resetPasswordSchema = z
  .object({
    password: z.string().min(6, '密码至少 6 位').max(100, '密码最多 100 位'),
    confirmPassword: z.string(),
  })
  .refine((data) => data.password === data.confirmPassword, {
    message: '两次密码不一致',
    path: ['confirmPassword'],
  });

export type ResetPasswordFormData = z.infer<typeof resetPasswordSchema>;
