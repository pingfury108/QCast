import { api } from '../lib/api';
import type {
  LoginRequest,
  LoginResponse,
  RegisterRequest,
  ForgotPasswordRequest,
  ResetPasswordRequest,
  CurrentUserResponse,
} from '../types/auth';

// 登录
export async function login(data: LoginRequest): Promise<LoginResponse> {
  const response = await api.post<LoginResponse>('/auth/login', data);
  return response.data;
}

// 注册
export async function register(data: RegisterRequest): Promise<void> {
  await api.post('/auth/register', data);
}

// 忘记密码
export async function forgotPassword(data: ForgotPasswordRequest): Promise<void> {
  await api.post('/auth/forgot', data);
}

// 重置密码
export async function resetPassword(data: ResetPasswordRequest): Promise<void> {
  await api.post('/auth/reset', data);
}

// 获取当前用户
export async function getCurrentUser(): Promise<CurrentUserResponse> {
  const response = await api.get<CurrentUserResponse>('/auth/current');
  return response.data;
}
