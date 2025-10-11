import { memo, useEffect } from 'react'
import type { ReactNode } from 'react'
import { useCurrentUser } from '../hooks/useAuth'
import { useAuthState } from '../hooks/useAuthState'

interface DashboardLayoutProps {
  children: ReactNode
}

const DashboardLayoutComponent = ({ children }: DashboardLayoutProps) => {
  const { user } = useAuthState();
  const { data: currentUser, isError } = useCurrentUser();

  // 当获取到新的用户信息时，更新本地存储
  useEffect(() => {
    if (currentUser) {
      console.log('DashboardLayout - 获取到新的用户信息:', currentUser);
    }
  }, [currentUser]);

  // 处理获取用户信息失败的情况
  useEffect(() => {
    if (isError) {
      console.error('DashboardLayout - 获取用户信息失败');
    }
  }, [isError]);

  return (
    <div className="container mx-auto px-4 py-8">
      {children}
    </div>
  )
}

export const DashboardLayout = memo(DashboardLayoutComponent);