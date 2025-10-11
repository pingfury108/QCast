import { useMutation, useQuery } from '@tanstack/react-query';
import { useNavigate } from 'react-router-dom';
import { toast } from 'sonner';
import * as authService from '../services/auth';
import { saveAuthState, clearAuthState, getStoredAuthState } from '../lib/auth';
import type { User } from '../types/auth';

// 登录
export function useLogin() {
  const navigate = useNavigate();

  return useMutation({
    mutationFn: authService.login,
    onSuccess: async (data) => {
      console.log('登录成功，响应数据:', data);

      try {
        // 登录后立即获取完整用户信息，确保包含最新的权限信息
        const currentUser = await authService.getCurrentUser();
        console.log('登录后获取的完整用户信息:', currentUser);

        // 使用 getCurrentUser 的响应创建完整的用户对象
        const user: User = {
          id: 0, // 如果后端不返回id，保持为0
          pid: currentUser.pid,
          name: currentUser.name,
          email: currentUser.email,
          is_staff: currentUser.is_staff,
          is_superuser: currentUser.is_superuser,
          email_verified_at: null, // 这个信息在 current 接口中也没有
          created_at: '',
          updated_at: '',
        };

        // 保存认证信息
        saveAuthState(data.token, user);
        console.log('认证信息已保存:', { token: data.token.substring(0, 20) + '...', user });

        toast.success('登录成功！');
        console.log('开始跳转到 dashboard...');
        navigate('/dashboard');
      } catch (error) {
        console.error('登录后获取用户信息失败，使用登录响应数据:', error);

        // 如果获取用户信息失败，使用登录响应的数据作为降级方案
        const user: User = {
          id: 0,
          pid: data.pid,
          name: data.name,
          email: '',
          is_staff: data.is_staff,
          is_superuser: data.is_superuser,
          email_verified_at: null,
          created_at: '',
          updated_at: '',
        };

        saveAuthState(data.token, user);
        console.log('使用登录数据保存认证信息:', { token: data.token.substring(0, 20) + '...', user });

        toast.success('登录成功！');
        console.log('开始跳转到 dashboard...');
        navigate('/dashboard');
      }
    },
    onError: (error: any) => {
      console.error('登录失败:', error);
      // 移除 toast 错误消息，让登录页面自己处理错误显示
      // const message = error.response?.data?.message || '登录失败，请检查邮箱和密码';
      // toast.error(message);
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
  const authState = getStoredAuthState();

  return useQuery({
    queryKey: ['currentUser', authState.token], // 将 token 加入依赖
    queryFn: async () => {
      try {
        const { user: storedUser } = getStoredAuthState();
        const currentUser = await authService.getCurrentUser();

        console.log('getCurrentUser API 响应:', currentUser);
        console.log('存储中的用户信息:', storedUser);

        // 更新本地存储的用户信息，包含管理员权限
        const user: User = {
          id: storedUser?.id || 0, // 保持原有的 id
          pid: currentUser.pid,
          name: currentUser.name,
          email: currentUser.email,
          is_staff: currentUser.is_staff,
          is_superuser: currentUser.is_superuser,
          email_verified_at: storedUser?.email_verified_at || null,
          created_at: storedUser?.created_at || '',
          updated_at: storedUser?.updated_at || '',
        };

        console.log('即将保存的用户信息:', user);

        // 更新本地存储的用户信息
        localStorage.setItem('qcast_user', JSON.stringify(user));
        console.log('用户信息已更新到 localStorage');

        return currentUser;
      } catch (error) {
        console.error('获取用户信息失败:', error);
        return null;
      }
    },
    enabled: true, // 启用自动更新以确保用户信息是最新的
    staleTime: 10 * 60 * 1000, // 10 分钟，减少请求频率
    retry: 1, // 减少重试次数
    refetchOnWindowFocus: false, // 窗口聚焦时不自动刷新
  });
}
