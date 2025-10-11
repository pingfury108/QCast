import { useAuthState } from '../hooks/useAuthState'
import { Card, CardContent } from '@/components/ui/card'
import { Avatar, AvatarFallback } from '@/components/ui/avatar'
import { User, Mail } from 'lucide-react'
import { Label } from '@/components/ui/label'

export default function ProfilePage() {
  const { user } = useAuthState()

  if (!user) {
    return (
      <div className="container mx-auto px-4 py-8">
        <div className="text-center text-muted-foreground">请先登录</div>
      </div>
    )
  }

  return (
    <div className="container mx-auto px-4 py-8 max-w-4xl">
      <div className="grid gap-6">
        {/* 基本信息卡片 */}
        <Card>
          <CardContent className="space-y-6 pt-6">
            {/* 头像 */}
            <div className="flex items-center gap-4">
              <Avatar className="w-20 h-20">
                <AvatarFallback className="bg-primary/10 text-primary text-2xl">
                  {user.name.charAt(0).toUpperCase()}
                </AvatarFallback>
              </Avatar>
              <div>
                <p className="text-sm font-medium">头像</p>
                <p className="text-sm text-muted-foreground">显示在您的个人资料中</p>
              </div>
            </div>

            {/* 用户名 */}
            <div className="space-y-2">
              <Label className="flex items-center gap-2">
                <User className="w-4 h-4" />
                用户名
              </Label>
              <p className="text-sm text-muted-foreground pl-6">{user.name}</p>
            </div>

            {/* 邮箱 */}
            <div className="space-y-2">
              <Label className="flex items-center gap-2">
                <Mail className="w-4 h-4" />
                邮箱
              </Label>
              <p className="text-sm text-muted-foreground pl-6">
                {user.email || '未设置'}
              </p>
            </div>
          </CardContent>
        </Card>

      </div>
    </div>
  )
}
