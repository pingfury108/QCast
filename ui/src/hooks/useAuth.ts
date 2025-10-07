import { useMutation, useQuery } from '@tanstack/react-query';
import { useNavigate } from 'react-router-dom';
import { toast } from 'sonner';
import * as authService from '../services/auth';
import { saveAuthState, clearAuthState, getStoredAuthState } from '../lib/auth';
import type { LoginRequest, RegisterRequest, ForgotPasswordRequest, ResetPasswordRequest, User } from '../types/auth';

// 登录
export function useLogin() {
  const navigate = useNavigate();

  return useMutation({
    mutationFn: authService.login,
    onSuccess: (data) => {
      console.log('登录成功，响应数据:', data);

      // 直接使用登录接口返回的信息
      const user: User = {
        pid: data.pid,
        name: data.name,
        email: data.email || '', // 如果登录接口返回了邮箱就使用，否则为空
      };

      // 保存认证信息
      saveAuthState(data.token, user);
      console.log('认证信息已保存:', { token: data.token.substring(0, 20) + '...', user });

      toast.success('登录成功！');

      // 直接跳转，不使用延迟
      console.log('开始跳转到 dashboard...');
      navigate('/dashboard');
    },
    onError: (error: any) => {
      console.error('登录失败:', error);
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
  return useQuery({
    queryKey: ['autoAuth'],
    queryFn: async () => {
      const token = localStorage.getItem('qcast_token');
      const storedUser = localStorage.getItem('qcast_user');

      if (!token) {
        // 没有 token，直接返回未认证状态
        return { isAuthenticated: false };
      }

      // 从本地存储获取用户信息
      let user = null;
      if (storedUser) {
        try {
          user = JSON.parse(storedUser);
        } catch {
          // 解析失败，清除无效数据
          clearAuthState();
          return { isAuthenticated: false };
        }
      }

      // 如果有基本用户信息，直接返回认证状态
      if (user && user.pid && user.name) {
        return { isAuthenticated: true, user };
      }

      // 如果没有有效的用户信息，清除认证状态
      clearAuthState();
      return { isAuthenticated: false };
    },
    staleTime: Infinity,
    retry: false,
  });
}

// 获取完整用户信息的 hook（在后台异步调用）
export function useCurrentUser() {
  return useQuery({
    queryKey: ['currentUser'],
    queryFn: async () => {
      try {
        const currentUser = await authService.getCurrentUser();
        console.log('获取到完整用户信息:', currentUser);

        // 更新本地存储的用户信息
        const user = {
          pid: currentUser.pid,
          name: currentUser.name,
          email: currentUser.email,
        };

        const { user: storedUser } = getStoredAuthState();
        if (storedUser && storedUser.pid === user.pid) {
          // 只有当 PID 匹配时才更新
          localStorage.setItem('qcast_user', JSON.stringify(user));
        }

        return currentUser;
      } catch (error) {
        console.error('获取用户信息失败:', error);
        return null;
      }
    },
    enabled: getStoredAuthState().isAuthenticated, // 只有在已认证时才调用
    staleTime: 5 * 60 * 1000, // 5 分钟
    retry: 2, // 失败时重试 2 次
  });
}
