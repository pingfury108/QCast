import type { AuthState, User } from '../types/auth';

const TOKEN_KEY = 'qcast_token';
const USER_KEY = 'qcast_user';

// 获取存储的认证状态
export function getStoredAuthState(): AuthState {
  const token = localStorage.getItem(TOKEN_KEY);
  const userStr = localStorage.getItem(USER_KEY);

  let user: User | null = null;
  if (userStr) {
    try {
      user = JSON.parse(userStr);
    } catch {
      // 忽略解析错误
    }
  }

  return {
    user,
    token,
    isAuthenticated: !!(token && user),
  };
}

// 保存认证状态
export function saveAuthState(token: string, user: User): void {
  localStorage.setItem(TOKEN_KEY, token);
  localStorage.setItem(USER_KEY, JSON.stringify(user));
}

// 清除认证状态
export function clearAuthState(): void {
  localStorage.removeItem(TOKEN_KEY);
  localStorage.removeItem(USER_KEY);
}

// 检查是否已登录
export function isAuthenticated(): boolean {
  const { isAuthenticated } = getStoredAuthState();
  return isAuthenticated;
}
