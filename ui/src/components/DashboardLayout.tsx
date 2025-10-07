import { ReactNode } from 'react'
import Header from './Header'
import Footer from './Footer'
import { useCurrentUser } from '../hooks/useAuth'

interface DashboardLayoutProps {
  children: ReactNode
}

export function DashboardLayout({ children }: DashboardLayoutProps) {
  // 在后台异步获取完整用户信息
  useCurrentUser();

  return (
    <div className="container mx-auto px-4 py-8">
      {children}
    </div>
  )
}