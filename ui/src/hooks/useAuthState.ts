import { useEffect, useState } from 'react';
import { getStoredAuthState } from '../lib/auth';

export function useAuthState() {
  const [authState, setAuthState] = useState(getStoredAuthState());

  useEffect(() => {
    const checkAuthState = () => {
      const newState = getStoredAuthState();
      // 只有状态真正变化时才更新
      setAuthState(prev => {
        if (JSON.stringify(prev) === JSON.stringify(newState)) {
          return prev; // 状态没有变化，不更新
        }
        return newState;
      });
    };

    // 监听存储变化
    const handleStorageChange = (e: StorageEvent) => {
      if (e.key === 'qcast_token' || e.key === 'qcast_user') {
        checkAuthState();
      }
    };

    // 降低检查频率（处理跨标签页同步）
    const interval = setInterval(checkAuthState, 10000); // 从2秒改为10秒

    window.addEventListener('storage', handleStorageChange);

    return () => {
      window.removeEventListener('storage', handleStorageChange);
      clearInterval(interval);
    };
  }, []);

  return authState;
}