import axios from 'axios';
import { getToken, clearAuthState } from './auth';

// 创建 axios 实例
const api = axios.create({
  baseURL: '/api',
  timeout: 10000,
});

// 请求拦截器
api.interceptors.request.use(
  (config) => {
    // 添加认证 token
    const token = getToken();
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
  },
  (error) => {
    return Promise.reject(error);
  }
);

// 响应拦截器
api.interceptors.response.use(
  (response) => {
    return response;
  },
  (error) => {
    // 处理认证错误 - Loco 没有刷新令牌，直接跳转登录
    if (error.response?.status === 401) {
      // 检查是否是登录请求，如果是则不自动跳转
      const isLoginRequest = error.config?.url?.includes('/auth/login');

      if (!isLoginRequest) {
        // 清除本地存储的认证信息
        clearAuthState();
        // 跳转到登录页
        window.location.href = '/login';
      }
    }
    return Promise.reject(error);
  }
);

export { api };