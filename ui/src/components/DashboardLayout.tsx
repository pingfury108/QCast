import { memo } from 'react'
import type { ReactNode } from 'react'
import { useCurrentUser } from '../hooks/useAuth'

interface DashboardLayoutProps {
  children: ReactNode
}

const DashboardLayoutComponent = ({ children }: DashboardLayoutProps) => {
  // 在后台异步获取完整用户信息
  useCurrentUser();

  return (
    <div className="container mx-auto px-4 py-8">
      {children}
    </div>
  )
}

export const DashboardLayout = memo(DashboardLayoutComponent);