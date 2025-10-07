import { useEffect, useState } from 'react';
import { getStoredAuthState } from '../lib/auth';

export function useAuthState() {
  const [authState, setAuthState] = useState(getStoredAuthState());

  useEffect(() => {
    const checkAuthState = () => {
      const newState = getStoredAuthState();
      console.log('认证状态检查:', {
        isAuthenticated: newState.isAuthenticated,
        hasToken: !!newState.token,
        hasUser: !!newState.user,
        user: newState.user
      });
      setAuthState(newState);
    };

    // 监听存储变化
    const handleStorageChange = (e: StorageEvent) => {
      console.log('存储变化事件:', e.key);
      if (e.key === 'qcast_token' || e.key === 'qcast_user') {
        checkAuthState();
      }
    };

    // 定期检查认证状态（处理跨标签页同步）
    const interval = setInterval(checkAuthState, 2000);

    window.addEventListener('storage', handleStorageChange);

    return () => {
      window.removeEventListener('storage', handleStorageChange);
      clearInterval(interval);
    };
  }, []);

  return authState;
}