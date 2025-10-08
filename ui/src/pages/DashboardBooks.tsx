import { useState } from 'react'
import { Link } from 'react-router-dom'
import { useBooks, useCreateBook, useUpdateBook, useDeleteBook, useBookTree } from '../hooks/useBooks'
import type { Book, BookTree } from '../hooks/useBooks'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Plus, Search, Edit, Trash2, Eye, EyeOff, MoreHorizontal, ChevronRight, ChevronDown, QrCode, BookOpen, Music } from 'lucide-react'
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle, DialogTrigger } from '@/components/ui/dialog'
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import * as z from 'zod'
import { toast } from 'sonner'
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from '@/components/ui/dropdown-menu'
import { Textarea } from '@/components/ui/textarea'
import { Switch } from '@/components/ui/switch'
import { Label } from '@/components/ui/label'

const createBookSchema = z.object({
  title: z.string().min(1, '书名不能为空'),
  description: z.string().optional(),
  cover_image: z.string().optional(),
  parent_id: z.number().optional(),
  sort_order: z.number().optional(),
  is_public: z.boolean().default(false)
})

type CreateBookForm = z.infer<typeof createBookSchema>

// 树形节点组件
const TreeNode = ({ node, level = 0, onEdit, onDelete, onTogglePublic }: {
  node: BookTree
  level?: number
  onEdit: (book: Book) => void
  onDelete: (book: Book) => void
  onTogglePublic: (book: Book) => void
}) => {
  const [isExpanded, setIsExpanded] = useState(level === 0)

  const book = node.book

  return (
    <div className="select-none">
      <Link to={`/dashboard/books/${book.id}`}>
        <div
          className={`flex items-center justify-between p-3 hover:bg-muted/50 rounded-lg cursor-pointer transition-colors ${
            level > 0 ? 'ml-' + (level * 8) : ''
          }`}
          style={{ marginLeft: level > 0 ? `${level * 2}rem` : '0' }}
        >
        <div className="flex items-center space-x-3">
          {node.children && node.children.length > 0 && (
            <button
              onClick={(e) => {
                e.preventDefault()
                e.stopPropagation()
                setIsExpanded(!isExpanded)
              }}
              className="p-1 hover:bg-muted rounded"
            >
              {isExpanded ? (
                <ChevronDown className="w-4 h-4" />
              ) : (
                <ChevronRight className="w-4 h-4" />
              )}
            </button>
          )}

          <div className="flex items-center space-x-2">
            <BookOpen className="w-5 h-5 text-blue-600" />

            <span className="font-medium">{book.title}</span>
            {book.is_public ? (
              <Eye className="w-4 h-4 text-green-600" />
            ) : (
              <EyeOff className="w-4 h-4 text-gray-400" />
            )}
          </div>
        </div>

        <div className="flex items-center space-x-2">
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button
                variant="ghost"
                size="sm"
                onClick={(e) => {
                  e.preventDefault()
                  e.stopPropagation()
                }}
              >
                <MoreHorizontal className="w-4 h-4" />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end">
              <DropdownMenuItem onClick={() => onEdit(book)}>
                <Edit className="w-4 h-4 mr-2" />
                编辑
              </DropdownMenuItem>
              <DropdownMenuItem onClick={() => onTogglePublic(book)}>
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
              </DropdownMenuItem>
              <DropdownMenuItem
                onClick={() => onDelete(book)}
                className="text-destructive"
              >
                <Trash2 className="w-4 h-4 mr-2" />
                删除
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        </div>
        </div>
      </Link>

      {isExpanded && node.children && (
        <div className="mt-1">
          {node.children.map((child, index) => (
            <TreeNode
              key={child.book.id || index}
              node={child}
              level={level + 1}
              onEdit={onEdit}
              onDelete={onDelete}
              onTogglePublic={onTogglePublic}
            />
          ))}
        </div>
      )}
    </div>
  )
}

export default function DashboardBooks() {
  const [searchQuery, setSearchQuery] = useState('')
  const [createDialogOpen, setCreateDialogOpen] = useState(false)
  const [editingBook, setEditingBook] = useState<Book | null>(null)
  const [editDialogOpen, setEditDialogOpen] = useState(false)

  const { data: books, isLoading, error } = useBooks()
  const createBookMutation = useCreateBook()
  const updateBookMutation = useUpdateBook()
  const deleteBookMutation = useDeleteBook()

  const createForm = useForm<CreateBookForm>({
    resolver: zodResolver(createBookSchema),
    defaultValues: {
      title: '',
      description: '',
      cover_image: '',
      is_public: false
    }
  })

  const editForm = useForm<CreateBookForm>({
    resolver: zodResolver(createBookSchema)
  })

  // 构建树形数据结构 - 使用真实的书籍树形结构
  const buildTreeData = (books: Book[]) => {
    // 只返回顶级书籍，不需要模拟的子章节
    const treeData = books?.filter(book => !book.parent_id).map(book => ({
      book: book,
      children: [] // 初始为空，后续可以通过 useBookTree 获取子书籍
    })) || []

    return treeData
  }

  const treeData = buildTreeData(books)

  const filteredTreeData = treeData.filter(treeNode =>
    treeNode.book.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
    treeNode.book.description?.toLowerCase().includes(searchQuery.toLowerCase())
  )

  const handleCreateBook = (data: CreateBookForm) => {
    createBookMutation.mutate(data)
    createForm.reset()
    setCreateDialogOpen(false)
  }

  const handleEditBook = (book: Book) => {
    setEditingBook(book)
    editForm.reset({
      title: book.title,
      description: book.description || '',
      cover_image: book.cover_image || '',
      is_public: book.is_public || false
    })
    setEditDialogOpen(true)
  }

  const handleUpdateBook = (data: CreateBookForm) => {
    if (!editingBook) return
    updateBookMutation.mutate({
      id: editingBook.id,
      params: data
    })
    setEditDialogOpen(false)
    setEditingBook(null)
  }

  const handleDeleteBook = (book: Book) => {
    if (window.confirm(`确定要删除书籍"${book.title}"吗？`)) {
      deleteBookMutation.mutate(book.id)
    }
  }

  const handleTogglePublic = (book: Book) => {
    updateBookMutation.mutate({
      id: book.id,
      params: { is_public: !book.is_public }
    })
  }

  if (isLoading) {
    return (
      <div className="flex justify-center items-center h-64">加载中...</div>
    )
  }

  if (error) {
    return (
      <div className="text-center text-destructive">加载失败</div>
    )
  }

  return (
    <>
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold">我的书籍</h1>
          <p className="text-muted-foreground">管理你的播客、有声书和媒体内容</p>
        </div>
        <Dialog open={createDialogOpen} onOpenChange={setCreateDialogOpen}>
          <DialogTrigger asChild>
            <Button>
              <Plus className="w-4 h-4 mr-2" />
              新建书籍
            </Button>
          </DialogTrigger>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>创建新书籍</DialogTitle>
              <DialogDescription>
                创建一个新的书籍来组织你的媒体内容
              </DialogDescription>
            </DialogHeader>
            <Form {...createForm}>
              <form onSubmit={createForm.handleSubmit(handleCreateBook)} className="space-y-4">
                <FormField
                  control={createForm.control}
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
                  control={createForm.control}
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
                  control={createForm.control}
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
                  <Button type="button" variant="outline" onClick={() => setCreateDialogOpen(false)}>
                    取消
                  </Button>
                  <Button type="submit" disabled={createBookMutation.isPending}>
                    {createBookMutation.isPending ? '创建中...' : '创建'}
                  </Button>
                </div>
              </form>
            </Form>
          </DialogContent>
        </Dialog>
      </div>

      {/* Search */}
      <div className="flex items-center space-x-2">
        <div className="relative flex-1 max-w-sm">
          <Search className="absolute left-2 top-2.5 h-4 w-4 text-muted-foreground" />
          <Input
            placeholder="搜索书籍..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="pl-8"
          />
        </div>
      </div>

        {/* Tree Structure */}
        <div className="border rounded-lg bg-card">
          {filteredTreeData.length === 0 ? (
            <div className="flex flex-col items-center justify-center py-12">
              <div className="text-muted-foreground text-center">
                <h3 className="text-lg font-medium mb-2">暂无书籍</h3>
                <p>创建你的第一个书籍来开始管理内容</p>
              </div>
            </div>
          ) : (
            <div className="p-4 space-y-1">
              {filteredTreeData.map((node) => (
                <TreeNode
                  key={node.book.id}
                  node={node}
                  onEdit={handleEditBook}
                  onDelete={handleDeleteBook}
                  onTogglePublic={handleTogglePublic}
                />
              ))}
            </div>
          )}
        </div>

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
    </>
  )
}