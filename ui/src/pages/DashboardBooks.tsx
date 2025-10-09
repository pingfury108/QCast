import { useState, useEffect } from 'react'
import { Link, useSearchParams } from 'react-router-dom'
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
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'

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
const TreeNode = ({ node, level = 0, onEdit, onDelete, onTogglePublic, onCreateSubBook }: {
  node: BookTree
  level?: number
  onEdit: (book: Book) => void
  onDelete: (book: Book) => void
  onTogglePublic: (book: Book) => void
  onCreateSubBook: (book: Book) => void
}) => {
  const [isExpanded, setIsExpanded] = useState(false) // 默认折叠

  const book = node.book
  const hasChildren = node.children && node.children.length > 0

  return (
    <div className="select-none">
      <Link to={`/dashboard/books/${book.id}`}>
        <div
          className={`
            flex items-center gap-2 p-3 rounded-lg border transition-all
            hover:bg-accent/50 cursor-pointer
            ${level > 0 ? 'ml-' + (level * 8) : ''
          }`}
          style={{ marginLeft: level > 0 ? `${level * 2}rem` : '0' }}
        >
        {/* 展开/收起按钮 - 始终保留占位位置 */}
          <Button
            variant="ghost"
            size="sm"
            className="h-6 w-6 p-0 flex-shrink-0"
            onClick={(e) => {
              if (hasChildren) {
                e.preventDefault()
                e.stopPropagation()
                setIsExpanded(!isExpanded)
              }
            }}
          >
            {hasChildren ? (
              isExpanded ? (
                <ChevronDown className="w-4 h-4" />
              ) : (
                <ChevronRight className="w-4 h-4" />
              )
            ) : (
              // 占位空间，没有子章节时显示空白
              <div className="w-4 h-4" />
            )}
          </Button>

          {/* 书籍图标和内容 */}
          <div className="w-8 h-8 rounded-lg bg-blue-100 flex items-center justify-center flex-shrink-0">
            <BookOpen className="w-4 h-4 text-blue-600" />
          </div>

          {/* 书籍信息 */}
          <div className="flex-1 min-w-0">
            <div className="font-medium truncate">{book.title}</div>
            <div className="text-sm text-muted-foreground flex items-center gap-3">
              {book.is_public ? (
                <span className="flex items-center gap-1">
                  <Eye className="w-3 h-3 text-green-600" />
                  公开
                </span>
              ) : (
                <span className="flex items-center gap-1">
                  <EyeOff className="w-3 h-3 text-gray-400" />
                  私密
                </span>
              )}
              <span className="flex items-center gap-1">
                <Music className="w-3 h-3" />
                {book.media_count || 0}
              </span>
              <span className="flex items-center gap-1">
                <BookOpen className="w-3 h-3" />
                {book.chapter_count || 0}
              </span>
            </div>
          </div>

          {/* 操作按钮 */}
          <div className="flex items-center gap-1">
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button
                variant="ghost"
                size="sm"
                className="h-8 w-8 p-0"
                onClick={(e) => {
                  e.preventDefault()
                  e.stopPropagation()
                }}
              >
                <MoreHorizontal className="w-4 h-4" />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end">
              <DropdownMenuItem onClick={(e) => {
                e.preventDefault()
                e.stopPropagation()
                onCreateSubBook(book)
              }}>
                <Plus className="w-4 h-4 mr-2" />
                创建子书籍
              </DropdownMenuItem>
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

      {isExpanded && hasChildren && (
        <div className="mt-1">
          {node.children.map((child, index) => (
            <TreeNode
              key={child.book.id || index}
              node={child}
              level={level + 1}
              onEdit={onEdit}
              onDelete={onDelete}
              onTogglePublic={onTogglePublic}
              onCreateSubBook={onCreateSubBook}
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
  const [parentBookId, setParentBookId] = useState<number | undefined>(undefined)
  const [searchParams, setSearchParams] = useSearchParams()

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

  // 处理URL参数，自动打开创建子书籍对话框
  useEffect(() => {
    const createSubBookParam = searchParams.get('create_sub_book')
    if (createSubBookParam) {
      const parentId = parseInt(createSubBookParam)
      if (!isNaN(parentId)) {
        handleCreateSubBook(books?.find(b => b.id === parentId)!)
        // 清除URL参数
        setSearchParams({})
      }
    }
  }, [searchParams, books])

  // 构建树形数据结构 - 使用真实的书籍树形结构
  const buildTreeData = (books: Book[]): BookTree[] => {
    if (!books) return []

    // 创建 ID 到书籍的映射
    const bookMap = new Map<number, BookTree>()
    books.forEach(book => {
      bookMap.set(book.id, { book, children: [] })
    })

    // 构建树形结构
    const rootBooks: BookTree[] = []
    books.forEach(book => {
      const treeNode = bookMap.get(book.id)!
      if (book.parent_id && bookMap.has(book.parent_id)) {
        // 添加到父书籍的children中
        bookMap.get(book.parent_id)!.children.push(treeNode)
      } else {
        // 顶级书籍
        rootBooks.push(treeNode)
      }
    })

    return rootBooks
  }

  const treeData = buildTreeData(books)

  const filteredTreeData = treeData.filter(treeNode =>
    treeNode.book.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
    treeNode.book.description?.toLowerCase().includes(searchQuery.toLowerCase())
  )

  const handleCreateBook = (data: CreateBookForm) => {
    // 如果设置了parentBookId，添加到表单数据中
    const bookData = parentBookId ? { ...data, parent_id: parentBookId } : data
    createBookMutation.mutate(bookData)
    createForm.reset()
    setCreateDialogOpen(false)
    setParentBookId(undefined)
  }

  const handleCreateSubBook = (parentBook: Book) => {
    setParentBookId(parentBook.id)
    createForm.reset({
      title: '',
      description: '',
      cover_image: '',
      is_public: false
    })
    setCreateDialogOpen(true)
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
              <DialogTitle>{parentBookId ? '创建子书籍' : '创建新书籍'}</DialogTitle>
              <DialogDescription>
                {parentBookId
                  ? `为 "${books?.find(b => b.id === parentBookId)?.title}" 创建一个子书籍`
                  : '创建一个新的书籍来组织你的媒体内容'
                }
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
                {!parentBookId && books && books.length > 0 && (
                  <FormField
                    control={createForm.control}
                    name="parent_id"
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>父书籍（可选）</FormLabel>
                        <Select
                          onValueChange={(value) => field.onChange(value ? parseInt(value) : undefined)}
                          value={field.value?.toString()}
                        >
                          <FormControl>
                            <SelectTrigger>
                              <SelectValue placeholder="选择父书籍" />
                            </SelectTrigger>
                          </FormControl>
                          <SelectContent>
                            <SelectItem value="0">无（顶级书籍）</SelectItem>
                            {books.map(book => (
                              <SelectItem key={book.id} value={book.id.toString()}>
                                {book.title}
                              </SelectItem>
                            ))}
                          </SelectContent>
                        </Select>
                        <FormMessage />
                      </FormItem>
                    )}
                  />
                )}
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
      <div className="flex items-center space-x-2 my-6">
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
                  onCreateSubBook={handleCreateSubBook}
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