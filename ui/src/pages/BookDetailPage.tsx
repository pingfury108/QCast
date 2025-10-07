import { useState } from 'react'
import { useParams, Link } from 'react-router-dom'
import { useBook, useUpdateBook } from '../hooks/useBooks'
import type { Book } from '../hooks/useBooks'
import { DashboardLayout } from '../components/DashboardLayout'
import { Button } from '@/components/ui/button'
import { ArrowLeft, Edit, Trash2, Eye, EyeOff, BookOpen, Music, Plus, Settings } from 'lucide-react'
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle, DialogTrigger } from '@/components/ui/dialog'
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import * as z from 'zod'
import { toast } from 'sonner'
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from '@/components/ui/dropdown-menu'
import { Textarea } from '@/components/ui/textarea'
import { Switch } from '@/components/ui/switch'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'

const updateBookSchema = z.object({
  title: z.string().min(1, '书名不能为空'),
  description: z.string().optional(),
  cover_image: z.string().optional(),
  is_public: z.boolean().default(false)
})

type UpdateBookForm = z.infer<typeof updateBookSchema>

export default function BookDetailPage() {
  const { id } = useParams<{ id: string }>()
  const bookId = parseInt(id)

  const [editDialogOpen, setEditDialogOpen] = useState(false)
  const [activeTab, setActiveTab] = useState('media')

  const { data: book, isLoading, error } = useBook(bookId)
  const updateBookMutation = useUpdateBook()

  const editForm = useForm<UpdateBookForm>({
    resolver: zodResolver(updateBookSchema)
  })

  const handleEditBook = () => {
    if (!book) return
    editForm.reset({
      title: book.title,
      description: book.description || '',
      cover_image: book.cover_image || '',
      is_public: book.is_public || false
    })
    setEditDialogOpen(true)
  }

  const handleUpdateBook = (data: UpdateBookForm) => {
    if (!book) return
    updateBookMutation.mutate({
      id: book.id,
      params: data
    })
    setEditDialogOpen(false)
  }

  const handleDeleteBook = () => {
    if (!book) return
    if (window.confirm(`确定要删除书籍"${book.title}"吗？`)) {
      // 这里应该调用删除逻辑，但我们的 hook 中没有这个功能
      toast.warning('删除功能暂未实现')
    }
  }

  const handleTogglePublic = () => {
    if (!book) return
    updateBookMutation.mutate({
      id: book.id,
      params: { is_public: !book.is_public }
    })
  }

  // 模拟章节数据
  const chapters = [
    { id: 1, title: '第一章：介绍', media_count: 2 },
    { id: 2, title: '第二章：主要内容', media_count: 3 },
    { id: 3, title: '第三章：总结', media_count: 1 }
  ]

  // 模拟媒体数据
  const medias = [
    { id: 1, title: '第一集音频', file_type: 'audio', duration: '20:15', play_count: 123, is_public: true },
    { id: 2, title: '第二集音频', file_type: 'audio', duration: '18:30', play_count: 45, is_public: false },
    { id: 3, title: '视频内容', file_type: 'video', duration: '22:00', play_count: 89, is_public: true }
  ]

  if (isLoading) {
    return (
      <DashboardLayout>
        <div className="flex justify-center items-center h-64">加载中...</div>
      </DashboardLayout>
    )
  }

  if (error || !book) {
    return (
      <DashboardLayout>
        <div className="text-center text-destructive">书籍不存在或加载失败</div>
      </DashboardLayout>
    )
  }

  return (
    <DashboardLayout>
      <div className="space-y-6">
        {/* Back Button */}
        <Link to="/dashboard/books">
          <Button variant="outline" className="mb-4">
            <ArrowLeft className="w-4 h-4 mr-2" />
            返回书籍列表
          </Button>
        </Link>

        {/* Book Header */}
        <div className="border rounded-lg bg-card p-6">
          <div className="flex items-start justify-between">
            <div className="flex-1">
              <div className="flex items-center gap-3 mb-2">
                <h1 className="text-2xl font-bold">{book.title}</h1>
                {book.is_public ? (
                  <Eye className="w-5 h-5 text-green-600" />
                ) : (
                  <EyeOff className="w-5 h-5 text-gray-400" />
                )}
              </div>

              {book.description && (
                <p className="text-muted-foreground mb-4">{book.description}</p>
              )}

              <div className="flex items-center gap-6 text-sm text-muted-foreground">
                <span>创建时间: {new Date(book.created_at).toLocaleDateString()}</span>
                <span>媒体数: {medias.length}</span>
                <span>状态: {book.is_public ? '👁️ 公开' : '🔒 私密'}</span>
              </div>
            </div>

            <div className="flex items-center space-x-2">
              <Button variant="outline" size="sm" onClick={handleEditBook}>
                <Edit className="w-4 h-4 mr-2" />
                编辑
              </Button>
              <Button variant="outline" size="sm" onClick={handleTogglePublic}>
                {book.is_public ? (
                  <>
                    <EyeOff className="w-4 h-4 mr-2" />
                    设为私密
                  </>
                ) : (
                  <>
                    <Eye className="w-4 h-4 mr-2" />
                    设为公开
                  </>
                )}
              </Button>
              <Button variant="outline" size="sm" onClick={handleDeleteBook} className="text-destructive">
                <Trash2 className="w-4 h-4 mr-2" />
                删除
              </Button>
            </div>
          </div>
        </div>

        {/* Tabs */}
        <Tabs value={activeTab} onValueChange={setActiveTab}>
          <TabsList className="grid w-full grid-cols-3">
            <TabsTrigger value="media" className="flex items-center gap-2">
              <Music className="w-4 h-4" />
              媒体
            </TabsTrigger>
            <TabsTrigger value="chapters" className="flex items-center gap-2">
              <BookOpen className="w-4 h-4" />
              章节
            </TabsTrigger>
            <TabsTrigger value="settings" className="flex items-center gap-2">
              <Settings className="w-4 h-4" />
              设置
            </TabsTrigger>
          </TabsList>

          {/* Media Tab */}
          <TabsContent value="media" className="space-y-4">
            <div className="flex items-center justify-between">
              <h2 className="text-lg font-semibold">媒体列表</h2>
              <Button>
                <Plus className="w-4 h-4 mr-2" />
                上传媒体
              </Button>
            </div>

            <div className="grid gap-4">
              {medias.map((media) => (
                <div key={media.id} className="flex items-center justify-between p-4 border rounded-lg">
                  <div className="flex items-center space-x-3">
                    <div className="w-10 h-10 rounded-lg bg-primary/10 flex items-center justify-center">
                      {media.file_type === 'audio' ? (
                        <Music className="w-5 h-5 text-primary" />
                      ) : (
                        <div className="w-5 h-5 bg-muted rounded" />
                      )}
                    </div>
                    <div>
                      <div className="font-medium">{media.title}</div>
                      <div className="text-sm text-muted-foreground">
                        {media.duration} • 播放: {media.play_count}次
                      </div>
                    </div>
                  </div>

                  <div className="flex items-center space-x-2">
                    {media.is_public ? (
                      <Eye className="w-4 h-4 text-green-600" />
                    ) : (
                      <EyeOff className="w-4 h-4 text-gray-400" />
                    )}
                    <DropdownMenu>
                      <DropdownMenuTrigger asChild>
                        <Button variant="ghost" size="sm">
                          <MoreHorizontal className="w-4 h-4" />
                        </Button>
                      </DropdownMenuTrigger>
                      <DropdownMenuContent align="end">
                        <DropdownMenuItem>编辑</DropdownMenuItem>
                        <DropdownMenuItem>复制链接</DropdownMenuItem>
                        <DropdownMenuItem>查看二维码</DropdownMenuItem>
                        <DropdownMenuItem className="text-destructive">删除</DropdownMenuItem>
                      </DropdownMenuContent>
                    </DropdownMenu>
                  </div>
                </div>
              ))}
            </div>
          </TabsContent>

          {/* Chapters Tab */}
          <TabsContent value="chapters" className="space-y-4">
            <div className="flex items-center justify-between">
              <h2 className="text-lg font-semibold">章节列表</h2>
              <Button>
                <Plus className="w-4 h-4 mr-2" />
                新建章节
              </Button>
            </div>

            <div className="grid gap-3">
              {chapters.map((chapter) => (
                <div key={chapter.id} className="flex items-center justify-between p-4 border rounded-lg">
                  <div className="flex items-center space-x-3">
                    <div className="w-10 h-10 rounded-lg bg-blue-100 flex items-center justify-center">
                      <BookOpen className="w-5 h-5 text-blue-600" />
                    </div>
                    <div>
                      <div className="font-medium">{chapter.title}</div>
                      <div className="text-sm text-muted-foreground">
                        {chapter.media_count} 个媒体文件
                      </div>
                    </div>
                  </div>

                  <DropdownMenu>
                    <DropdownMenuTrigger asChild>
                      <Button variant="ghost" size="sm">
                        <MoreHorizontal className="w-4 h-4" />
                      </Button>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent align="end">
                      <DropdownMenuItem>编辑</DropdownMenuItem>
                      <DropdownMenuItem>移动</DropdownMenuItem>
                      <DropdownMenuItem className="text-destructive">删除</DropdownMenuItem>
                    </DropdownMenuContent>
                  </DropdownMenu>
                </div>
              ))}
            </div>
          </TabsContent>

          {/* Settings Tab */}
          <TabsContent value="settings" className="space-y-6">
            <div className="max-w-2xl">
              <h2 className="text-lg font-semibold mb-4">书籍设置</h2>

              <div className="space-y-6">
                <div className="flex items-center justify-between p-4 border rounded-lg">
                  <div>
                    <div className="font-medium">公开状态</div>
                    <div className="text-sm text-muted-foreground">
                      {book.is_public ? '此书籍对所有人可见' : '仅自己可见'}
                    </div>
                  </div>
                  <Switch
                    checked={book.is_public}
                    onCheckedChange={handleTogglePublic}
                  />
                </div>

                <div className="p-4 border rounded-lg">
                  <div className="font-medium mb-2">书籍信息</div>
                  <div className="text-sm text-muted-foreground space-y-1">
                    <p>ID: {book.id}</p>
                    <p>用户ID: {book.user_id}</p>
                    <p>创建时间: {new Date(book.created_at).toLocaleString()}</p>
                    <p>更新时间: {new Date(book.updated_at).toLocaleString()}</p>
                  </div>
                </div>
              </div>
            </div>
          </TabsContent>
        </Tabs>

        {/* Edit Dialog */}
        <Dialog open={editDialogOpen} onOpenChange={setEditDialogOpen}>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>编辑书籍</DialogTitle>
              <DialogDescription>
                修改书籍信息
              </DialogDescription>
            </DialogHeader>
            <Form {...editForm}>
              <form onSubmit={editForm.handleSubmit(handleUpdateBook)} className="space-y-4">
                <FormField
                  control={editForm.control}
                  name="title"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>书名</FormLabel>
                      <FormControl>
                        <Input placeholder="输入书籍名称" {...field} />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />
                <FormField
                  control={editForm.control}
                  name="description"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>描述</FormLabel>
                      <FormControl>
                        <Textarea placeholder="输入书籍描述" {...field} />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />
                <FormField
                  control={editForm.control}
                  name="is_public"
                  render={({ field }) => (
                    <FormItem className="flex items-center space-x-2">
                      <FormControl>
                        <Switch
                          checked={field.value}
                          onCheckedChange={field.onChange}
                        />
                      </FormControl>
                      <FormLabel>公开书籍</FormLabel>
                      <FormMessage />
                    </FormItem>
                  )}
                />
                <div className="flex justify-end space-x-2">
                  <Button type="button" variant="outline" onClick={() => setEditDialogOpen(false)}>
                    取消
                  </Button>
                  <Button type="submit" disabled={updateBookMutation.isPending}>
                    {updateBookMutation.isPending ? '更新中...' : '更新'}
                  </Button>
                </div>
              </form>
            </Form>
          </DialogContent>
        </Dialog>
      </div>
    </DashboardLayout>
  )
}