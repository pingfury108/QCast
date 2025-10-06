import { useMutation, useQuery } from '@tanstack/react-query';
import { useNavigate } from 'react-router-dom';
import { toast } from 'sonner';
import * as authService from '../services/auth';
import { saveAuthState, clearAuthState } from '../lib/auth';
import type { LoginRequest, RegisterRequest, ForgotPasswordRequest, ResetPasswordRequest, User } from '../types/auth';

// 登录
export function useLogin() {
  const navigate = useNavigate();

  return useMutation({
    mutationFn: authService.login,
    onSuccess: (data) => {
      // 保存认证信息
      const user: User = {
        pid: data.pid,
        name: data.name,
        email: '', // 登录接口不返回 email，之后从 current 接口获取
      };
      saveAuthState(data.token, user);

      toast.success('登录成功！');
      navigate('/dashboard');
    },
    onError: (error: any) => {
      const message = error.response?.data?.message || '登录失败，请检查邮箱和密码';
      toast.error(message);
    },
  });
}

// 注册
export function useRegister() {
  const navigate = useNavigate();

  return useMutation({
    mutationFn: authService.register,
    onSuccess: () => {
      toast.success('注册成功！请检查邮箱进行验证');
      navigate('/login');
    },
    onError: (error: any) => {
      const message = error.response?.data?.message || '注册失败，请稍后重试';
      toast.error(message);
    },
  });
}

// 忘记密码
export function useForgotPassword() {
  return useMutation({
    mutationFn: authService.forgotPassword,
    onSuccess: () => {
      toast.success('重置密码链接已发送到您的邮箱');
    },
    onError: (error: any) => {
      const message = error.response?.data?.message || '发送失败，请稍后重试';
      toast.error(message);
    },
  });
}

// 重置密码
export function useResetPassword() {
  const navigate = useNavigate();

  return useMutation({
    mutationFn: authService.resetPassword,
    onSuccess: () => {
      toast.success('密码重置成功！请使用新密码登录');
      navigate('/login');
    },
    onError: (error: any) => {
      const message = error.response?.data?.message || '重置失败，请稍后重试';
      toast.error(message);
    },
  });
}

// 获取当前用户
export function useCurrentUser() {
  return useQuery({
    queryKey: ['currentUser'],
    queryFn: authService.getCurrentUser,
    retry: false,
    staleTime: 5 * 60 * 1000, // 5 分钟
  });
}

// 登出
export function useLogout() {
  const navigate = useNavigate();

  return () => {
    clearAuthState();
    toast.success('已退出登录');
    navigate('/login');
  };
}

// 自动认证检查（在应用启动时调用）- 适用于 Loco 框架（无刷新令牌）
export function useAutoAuth() {
  const navigate = useNavigate();

  return useQuery({
    queryKey: ['autoAuth'],
    queryFn: async () => {
      const token = localStorage.getItem('qcast_token');

      if (!token) {
        // 没有 token，直接返回未认证状态
        return { isAuthenticated: false };
      }

      try {
        // 尝试获取当前用户信息
        const user = await authService.getCurrentUser();
        return { isAuthenticated: true, user };
      } catch (error) {
        // token 无效，清除认证信息
        clearAuthState();
        return { isAuthenticated: false };
      }
    },
    staleTime: Infinity, // 只在需要时重新检查
    retry: false,
    onError: () => {
      // 认证检查失败，清除认证信息
      clearAuthState();
      navigate('/login');
    },
  });
}
