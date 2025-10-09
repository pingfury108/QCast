import { useState } from 'react'
import { useParams, Link } from 'react-router-dom'
import { useBook, useUpdateBook } from '../hooks/useBooks'
import {
  useChapters,
  useCreateChapter,
  useUpdateChapter,
  useDeleteChapter,
  useReorderChapter,
  useChapterTree,
  useCreateChildChapter,
  useMoveChapter
} from '../hooks/useChapters'
import { useBookMedias, useToggleMediaPublish, useDeleteMedia, useUploadMedia } from '../hooks/useMedias'
import type { Book } from '../hooks/useBooks'
import type { Chapter, ChapterTree } from '../services/chapters'
import type { Media } from '../services/medias'
import { DashboardLayout } from '../components/DashboardLayout'
import { Button } from '@/components/ui/button'
import { api } from '../lib/api'
import { ArrowLeft, Edit, Trash2, Eye, EyeOff, BookOpen, Music, Plus, Settings, MoreHorizontal, ChevronUp, ChevronDown, Copy, QrCode, Upload as UploadIcon, X } from 'lucide-react'
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle, DialogTrigger } from '@/components/ui/dialog'
import ChapterTree from '../components/ChapterTree'
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import * as z from 'zod'
import { toast } from 'sonner'
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from '@/components/ui/dropdown-menu'
import { Textarea } from '@/components/ui/textarea'
import { Switch } from '@/components/ui/switch'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { Input } from '@/components/ui/input'
import { Progress } from '@/components/ui/progress'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'

const updateBookSchema = z.object({
  title: z.string().min(1, '书名不能为空'),
  description: z.string().optional(),
  cover_image: z.string().optional(),
  is_public: z.boolean().default(false)
})

const createChapterSchema = z.object({
  title: z.string().min(1, '章节标题不能为空'),
  description: z.string().optional()
})

const updateChapterSchema = z.object({
  title: z.string().min(1, '章节标题不能为空'),
  description: z.string().optional()
})

const uploadMediaSchema = z.object({
  title: z.string().min(1, '媒体标题不能为空'),
  description: z.string().optional(),
  chapter_id: z.string().optional(),
  file: z.any().refine((file) => file instanceof File, '请选择文件')
})

type UpdateBookForm = z.infer<typeof updateBookSchema>
type CreateChapterForm = z.infer<typeof createChapterSchema>
type UpdateChapterForm = z.infer<typeof updateChapterSchema>
type UploadMediaForm = z.infer<typeof uploadMediaSchema>

export default function BookDetailPage() {
  const { id } = useParams<{ id: string }>()
  const bookId = parseInt(id)

  const [editDialogOpen, setEditDialogOpen] = useState(false)
  const [chapterDialogOpen, setChapterDialogOpen] = useState(false)
  const [editChapterDialogOpen, setEditChapterDialogOpen] = useState(false)
  const [uploadMediaDialogOpen, setUploadMediaDialogOpen] = useState(false)
  const [uploadProgress, setUploadProgress] = useState(0)
  const [selectedFile, setSelectedFile] = useState<File | null>(null)
  const [qrCodeDialogOpen, setQrCodeDialogOpen] = useState(false)
  const [qrCodeUrl, setQrCodeUrl] = useState<string | null>(null)
  const [qrCodeMedia, setQrCodeMedia] = useState<Media | null>(null)
  const [activeTab, setActiveTab] = useState('media')
  const [editingChapter, setEditingChapter] = useState<Chapter | null>(null)
  const [expandedChapters, setExpandedChapters] = useState<Set<number>>(new Set())

  const { data: book, isLoading, error } = useBook(bookId)
  const { data: chapters = [], isLoading: chaptersLoading } = useChapters(bookId)
  const { data: chapterTree = [], isLoading: chapterTreeLoading } = useChapterTree(bookId)
  const { data: medias = [], isLoading: mediasLoading } = useBookMedias(bookId)
  const updateBookMutation = useUpdateBook()
  const createChapterMutation = useCreateChapter()
  const updateChapterMutation = useUpdateChapter()
  const deleteChapterMutation = useDeleteChapter()
  const reorderChapterMutation = useReorderChapter()
  const createChildChapterMutation = useCreateChildChapter()
  const moveChapterMutation = useMoveChapter()
  const toggleMediaPublishMutation = useToggleMediaPublish()
  const deleteMediaMutation = useDeleteMedia()
  const uploadMediaMutation = useUploadMedia()

  const editForm = useForm<UpdateBookForm>({
    resolver: zodResolver(updateBookSchema)
  })

  const chapterForm = useForm<CreateChapterForm>({
    resolver: zodResolver(createChapterSchema)
  })

  const editChapterForm = useForm<UpdateChapterForm>({
    resolver: zodResolver(updateChapterSchema)
  })

  const uploadMediaForm = useForm<UploadMediaForm>({
    resolver: zodResolver(uploadMediaSchema)
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

  const handleCreateChapter = (data: CreateChapterForm) => {
    if (!book) return
    createChapterMutation.mutate({
      bookId: book.id,
      params: data
    })
    setChapterDialogOpen(false)
    chapterForm.reset()
  }

  const handleEditChapter = (chapter: Chapter) => {
    setEditingChapter(chapter)
    editChapterForm.reset({
      title: chapter.title,
      description: chapter.description || ''
    })
    setEditChapterDialogOpen(true)
  }

  const handleUpdateChapter = (data: UpdateChapterForm) => {
    if (!book || !editingChapter) return
    updateChapterMutation.mutate({
      bookId: book.id,
      id: editingChapter.id,
      params: data
    })
    setEditChapterDialogOpen(false)
    setEditingChapter(null)
  }

  const handleDeleteChapter = (chapter: Chapter | ChapterTree) => {
    if (window.confirm(`确定要删除章节"${chapter.title}"吗？`)) {
      if (!book) return
      deleteChapterMutation.mutate({
        bookId: book.id,
        id: chapter.id
      })
    }
  }

  const handleCreateChildChapter = (parentId: number, params: { title: string; description?: string }) => {
    if (!book) return
    createChildChapterMutation.mutate({
      bookId: book.id,
      parentId,
      params
    })
  }

  const handleMoveChapter = (chapterId: number, newParentId?: number) => {
    if (!book) return
    moveChapterMutation.mutate({
      bookId: book.id,
      id: chapterId,
      params: {
        new_parent_id: newParentId
      }
    })
  }

  const handleDropChapter = (draggedId: number, targetId: number) => {
    if (!book) return

    // 检查是否是同级拖拽
    const draggedChapter = findChapterInTree(chapterTree, draggedId)
    const targetChapter = findChapterInTree(chapterTree, targetId)

    if (draggedChapter && targetChapter) {
      if (draggedChapter.parent_id === targetChapter.parent_id) {
        // 同级排序 - 使用reorder API
        const targetSortOrder = targetChapter.sort_order || 0
        reorderChapterMutation.mutate({
          bookId: book.id,
          id: draggedId,
          params: { sort_order: targetSortOrder }
        })
      } else {
        // 跨级移动 - 成为子章节
        moveChapterMutation.mutate({
          bookId: book.id,
          id: draggedId,
          params: {
            new_parent_id: targetId
          }
        })
      }
    }
  }

  // 辅助函数：在树中查找章节
  const findChapterInTree = (tree: ChapterTree[], id: number): ChapterTree | null => {
    for (const chapter of tree) {
      if (chapter.id === id) return chapter
      if (chapter.children) {
        const found = findChapterInTree(chapter.children, id)
        if (found) return found
      }
    }
    return null
  }

  const handleMoveChapterUp = (chapter: Chapter) => {
    if (!book) return
    moveChapterUpMutation.mutate({
      bookId: book.id,
      id: chapter.id
    })
  }

  const handleMoveChapterDown = (chapter: Chapter) => {
    if (!book) return
    moveChapterDownMutation.mutate({
      bookId: book.id,
      id: chapter.id
    })
  }

  const handleReorderChapter = (draggedId: number, targetId: number, position: 'before' | 'after') => {
    // 同级内重新排序
    if (!book) return

    const targetChapter = findChapterInTree(chapterTree, targetId)
    if (!targetChapter) return

    let newSortOrder: number
    if (position === 'before') {
      // 插入到目标前面，使用目标的sort_order
      newSortOrder = targetChapter.sort_order || 0
    } else {
      // 插入到目标后面，使用目标的sort_order + 1
      newSortOrder = (targetChapter.sort_order || 0) + 1
    }

    reorderChapterMutation.mutate({
      bookId: book.id,
      id: draggedId,
      params: { sort_order: newSortOrder }
    })
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

  const handleToggleMediaPublish = (media: Media) => {
    toggleMediaPublishMutation.mutate(media.id)
  }

  const handleDeleteMedia = (media: Media) => {
    if (window.confirm(`确定要删除媒体"${media.title}"吗？`)) {
      deleteMediaMutation.mutate(media.id)
    }
  }

  const handleFileSelect = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0]
    if (file) {
      setSelectedFile(file)
      uploadMediaForm.setValue('file', file)
      // 自动填充文件名作为标题
      if (!uploadMediaForm.getValues('title')) {
        uploadMediaForm.setValue('title', file.name.replace(/\.[^/.]+$/, ''))
      }
    }
  }

  const handleUploadMedia = async (data: UploadMediaForm) => {
    if (!book || !selectedFile) return

    setUploadProgress(0)

    try {
      await uploadMediaMutation.mutateAsync({
        params: {
          title: data.title,
          description: data.description,
          book_id: book.id,
          chapter_id: data.chapter_id && data.chapter_id !== "0" ? parseInt(data.chapter_id) : undefined,
          file: selectedFile
        },
        onProgress: setUploadProgress
      })

      // 重置表单和状态
      uploadMediaForm.reset()
      setSelectedFile(null)
      setUploadProgress(0)
      setUploadMediaDialogOpen(false)
    } catch (error) {
      // 错误已经在 hook 中处理
      setUploadProgress(0)
    }
  }

  const handleCopyLink = (media: Media) => {
    if (media.access_url) {
      navigator.clipboard.writeText(media.access_url)
        .then(() => toast.success('链接已复制到剪贴板'))
        .catch(() => toast.error('复制链接失败'))
    }
  }

  const handleViewQRCode = async (media: Media) => {
    setQrCodeMedia(media)
    setQrCodeDialogOpen(true)
    setQrCodeUrl(null)

    try {
      // 使用配置好的 api 实例请求二维码数据，自动携带认证 token
      const response = await api.get(`/media/${media.id}/qrcode`, {
        responseType: 'blob' // 接收二进制数据
      })

      // 创建 blob URL
      const blob = new Blob([response.data], { type: 'image/svg+xml' })
      const url = URL.createObjectURL(blob)
      setQrCodeUrl(url)

      // 清理 URL 对象
      setTimeout(() => {
        URL.revokeObjectURL(url)
      }, 60000) // 1分钟后清理
    } catch (error) {
      console.error('获取二维码失败:', error)
      setQrCodeUrl(null)
    }
  }

  const formatDuration = (seconds?: number) => {
    if (!seconds) return '--:--'
    const mins = Math.floor(seconds / 60)
    const secs = seconds % 60
    return `${mins}:${secs.toString().padStart(2, '0')}`
  }

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
              <Button variant="outline" size="sm" onClick={() => {
                // TODO: 导航到创建书籍页面并设置parent_id
                window.location.href = `/dashboard/books?create_sub_book=${book.id}`
              }}>
                <Plus className="w-4 h-4 mr-2" />
                创建子书籍
              </Button>
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
              <Button onClick={() => setUploadMediaDialogOpen(true)}>
                <Plus className="w-4 h-4 mr-2" />
                上传媒体
              </Button>
            </div>

            {mediasLoading ? (
              <div className="text-center py-8">加载媒体中...</div>
            ) : medias.length === 0 ? (
              <div className="text-center py-8 text-muted-foreground">
                <Music className="w-12 h-12 mx-auto mb-4 opacity-50" />
                <p>暂无媒体文件</p>
                <p className="text-sm">点击上方按钮上传第一个媒体文件</p>
              </div>
            ) : (
              <div className="grid gap-4">
                {medias.map((media) => (
                  <div key={media.id} className="flex items-center justify-between p-4 border rounded-lg">
                    <div className="flex items-center space-x-3">
                      <div className="w-10 h-10 rounded-lg bg-primary/10 flex items-center justify-center">
                        {media.file_type === 'audio' ? (
                          <Music className="w-5 h-5 text-primary" />
                        ) : media.file_type === 'video' ? (
                          <div className="w-5 h-5 bg-muted rounded flex items-center justify-center">
                            <div className="w-3 h-3 bg-red-500 rounded-full"></div>
                          </div>
                        ) : (
                          <div className="w-5 h-5 bg-muted rounded" />
                        )}
                      </div>
                      <div>
                        <div className="font-medium">{media.title}</div>
                        <div className="text-sm text-muted-foreground">
                          {formatDuration(media.duration)} • 播放: {media.play_count}次
                          {media.chapter_id && ' • 关联章节'}
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
                          <DropdownMenuItem onClick={() => handleCopyLink(media)}>
                            <Copy className="w-4 h-4 mr-2" />
                            复制链接
                          </DropdownMenuItem>
                          {media.qr_code_path && (
                            <DropdownMenuItem onClick={() => handleViewQRCode(media)}>
                              <QrCode className="w-4 h-4 mr-2" />
                              查看二维码
                            </DropdownMenuItem>
                          )}
                          <DropdownMenuItem onClick={() => handleToggleMediaPublish(media)}>
                            {media.is_public ? (
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
                          </DropdownMenuItem>
                          <DropdownMenuItem className="text-destructive" onClick={() => handleDeleteMedia(media)}>
                            <Trash2 className="w-4 h-4 mr-2" />
                            删除
                          </DropdownMenuItem>
                        </DropdownMenuContent>
                      </DropdownMenu>
                    </div>
                  </div>
                ))}
              </div>
            )}
          </TabsContent>

          {/* Chapters Tab */}
          <TabsContent value="chapters" className="space-y-4">
            <div className="flex items-center justify-between">
              <h2 className="text-lg font-semibold">章节列表</h2>
              <Button onClick={() => setChapterDialogOpen(true)}>
                <Plus className="w-4 h-4 mr-2" />
                新建章节
              </Button>
            </div>

            {chapterTreeLoading ? (
              <div className="text-center py-8">加载章节中...</div>
            ) : (
              <ChapterTree
                chapters={chapterTree}
                bookId={book.id}
                onEdit={(chapter) => {
                  // 转换为Chapter类型用于编辑
                  const chapterForEdit: Chapter = {
                    ...chapter,
                    parent_id: chapter.parent_id || undefined,
                    level: chapter.level || undefined,
                    path: chapter.path || undefined
                  }
                  setEditingChapter(chapterForEdit)
                  editChapterForm.reset({
                    title: chapterForEdit.title,
                    description: chapterForEdit.description || ''
                  })
                  setEditChapterDialogOpen(true)
                }}
                onDelete={handleDeleteChapter}
                onCreateChild={handleCreateChildChapter}
                onMove={handleMoveChapter}
                onDrop={handleDropChapter}
                onReorder={handleReorderChapter}
              />
            )}
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

        {/* Create Chapter Dialog */}
        <Dialog open={chapterDialogOpen} onOpenChange={setChapterDialogOpen}>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>新建章节</DialogTitle>
              <DialogDescription>
                为这本书创建新章节
              </DialogDescription>
            </DialogHeader>
            <Form {...chapterForm}>
              <form onSubmit={chapterForm.handleSubmit(handleCreateChapter)} className="space-y-4">
                <FormField
                  control={chapterForm.control}
                  name="title"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>章节标题</FormLabel>
                      <FormControl>
                        <Input placeholder="输入章节标题" {...field} />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />
                <FormField
                  control={chapterForm.control}
                  name="description"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>章节描述</FormLabel>
                      <FormControl>
                        <Textarea placeholder="输入章节描述（可选）" {...field} />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />
                <div className="flex justify-end space-x-2">
                  <Button type="button" variant="outline" onClick={() => setChapterDialogOpen(false)}>
                    取消
                  </Button>
                  <Button type="submit" disabled={createChapterMutation.isPending}>
                    {createChapterMutation.isPending ? '创建中...' : '创建'}
                  </Button>
                </div>
              </form>
            </Form>
          </DialogContent>
        </Dialog>

        {/* Edit Chapter Dialog */}
        <Dialog open={editChapterDialogOpen} onOpenChange={setEditChapterDialogOpen}>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>编辑章节</DialogTitle>
              <DialogDescription>
                修改章节信息
              </DialogDescription>
            </DialogHeader>
            <Form {...editChapterForm}>
              <form onSubmit={editChapterForm.handleSubmit(handleUpdateChapter)} className="space-y-4">
                <FormField
                  control={editChapterForm.control}
                  name="title"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>章节标题</FormLabel>
                      <FormControl>
                        <Input placeholder="输入章节标题" {...field} />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />
                <FormField
                  control={editChapterForm.control}
                  name="description"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>章节描述</FormLabel>
                      <FormControl>
                        <Textarea placeholder="输入章节描述（可选）" {...field} />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />
                <div className="flex justify-end space-x-2">
                  <Button type="button" variant="outline" onClick={() => setEditChapterDialogOpen(false)}>
                    取消
                  </Button>
                  <Button type="submit" disabled={updateChapterMutation.isPending}>
                    {updateChapterMutation.isPending ? '更新中...' : '更新'}
                  </Button>
                </div>
              </form>
            </Form>
          </DialogContent>
        </Dialog>

        {/* Upload Media Dialog */}
        <Dialog open={uploadMediaDialogOpen} onOpenChange={setUploadMediaDialogOpen}>
          <DialogContent className="max-w-2xl">
            <DialogHeader>
              <DialogTitle>上传媒体</DialogTitle>
              <DialogDescription>
                上传音频、视频或其他媒体文件到当前书籍
              </DialogDescription>
            </DialogHeader>
            <Form {...uploadMediaForm}>
              <form onSubmit={uploadMediaForm.handleSubmit(handleUploadMedia)} className="space-y-4">
                <FormField
                  control={uploadMediaForm.control}
                  name="file"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>选择文件</FormLabel>
                      <FormControl>
                        <div className="space-y-2">
                          <Input
                            type="file"
                            accept="audio/*,video/*,image/*"
                            onChange={handleFileSelect}
                          />
                          {selectedFile && (
                            <div className="text-sm text-muted-foreground">
                              已选择: {selectedFile.name} ({(selectedFile.size / 1024 / 1024).toFixed(2)} MB)
                            </div>
                          )}
                        </div>
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />

                <FormField
                  control={uploadMediaForm.control}
                  name="title"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>媒体标题</FormLabel>
                      <FormControl>
                        <Input placeholder="输入媒体标题" {...field} />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />

                <FormField
                  control={uploadMediaForm.control}
                  name="description"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>媒体描述</FormLabel>
                      <FormControl>
                        <Textarea placeholder="输入媒体描述（可选）" {...field} />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />

                <FormField
                  control={uploadMediaForm.control}
                  name="chapter_id"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>关联章节（可选）</FormLabel>
                      <Select onValueChange={field.onChange} defaultValue={field.value}>
                        <FormControl>
                          <SelectTrigger>
                            <SelectValue placeholder="选择关联的章节" />
                          </SelectTrigger>
                        </FormControl>
                        <SelectContent>
                          <SelectItem value="0">不关联章节</SelectItem>
                          {chapters.map((chapter) => (
                            <SelectItem key={chapter.id} value={chapter.id.toString()}>
                              {chapter.title}
                            </SelectItem>
                          ))}
                        </SelectContent>
                      </Select>
                      <FormMessage />
                    </FormItem>
                  )}
                />

                {uploadProgress > 0 && uploadProgress < 100 && (
                  <div className="space-y-2">
                    <div className="flex justify-between text-sm">
                      <span>上传进度</span>
                      <span>{uploadProgress}%</span>
                    </div>
                    <Progress value={uploadProgress} />
                  </div>
                )}

                <div className="flex justify-end space-x-2">
                  <Button
                    type="button"
                    variant="outline"
                    onClick={() => {
                      setUploadMediaDialogOpen(false)
                      uploadMediaForm.reset()
                      setSelectedFile(null)
                      setUploadProgress(0)
                    }}
                  >
                    取消
                  </Button>
                  <Button type="submit" disabled={uploadMediaMutation.isPending || !selectedFile}>
                    {uploadMediaMutation.isPending ? '上传中...' : '上传'}
                  </Button>
                </div>
              </form>
            </Form>
          </DialogContent>
        </Dialog>

        {/* QR Code Dialog */}
        <Dialog open={qrCodeDialogOpen} onOpenChange={setQrCodeDialogOpen}>
          <DialogContent className="max-w-md">
            <DialogHeader>
              <DialogTitle>媒体二维码</DialogTitle>
              <DialogDescription>
                扫描二维码访问媒体文件
              </DialogDescription>
            </DialogHeader>
            <div className="flex flex-col items-center space-y-4">
              {qrCodeMedia && (
                <>
                  <div className="text-center">
                    <p className="font-medium">{qrCodeMedia.title}</p>
                    <p className="text-sm text-muted-foreground">{qrCodeMedia.file_name}</p>
                  </div>

                  {qrCodeUrl ? (
                    <div className="relative">
                      <img
                        src={qrCodeUrl}
                        alt="媒体二维码"
                        className="w-64 h-64 border rounded-lg"
                      />
                      {qrCodeMedia.access_url && (
                        <Button
                          variant="outline"
                          size="sm"
                          className="absolute -bottom-2 -right-2"
                          onClick={() => navigator.clipboard.writeText(qrCodeMedia.access_url!)}
                        >
                          <Copy className="w-4 h-4 mr-1" />
                          复制链接
                        </Button>
                      )}
                    </div>
                  ) : (
                    <div className="w-64 h-64 bg-gray-100 flex items-center justify-center">
                      <p className="text-muted-foreground">二维码加载失败</p>
                    </div>
                  )}
                </>
              )}

              <div className="flex justify-end w-full pt-4">
                <Button
                  variant="outline"
                  onClick={() => {
                    // 清理 blob URL
                    if (qrCodeUrl && qrCodeUrl.startsWith('blob:')) {
                      URL.revokeObjectURL(qrCodeUrl)
                    }
                    setQrCodeDialogOpen(false)
                    setQrCodeUrl(null)
                    setQrCodeMedia(null)
                  }}
                >
                  关闭
                </Button>
              </div>
            </div>
          </DialogContent>
        </Dialog>
      </div>
    </DashboardLayout>
  )
}