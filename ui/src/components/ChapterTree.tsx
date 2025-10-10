import React, { useState } from 'react'
import { ChevronRight, ChevronDown, Plus, Edit, Trash2, MoreHorizontal, BookOpen, GripVertical } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from '@/components/ui/dropdown-menu'
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle } from '@/components/ui/dialog'
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form'
import { Input } from '@/components/ui/input'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import * as z from 'zod'
import { Textarea } from '@/components/ui/textarea'
import type { ChapterTree } from '../hooks/useChapters'

interface ChapterTreeProps {
  chapters: ChapterTree[]
  bookId: number
  onEdit?: (chapter: ChapterTree) => void
  onDelete?: (chapter: ChapterTree) => void
  onCreateChild?: (parentId: number, params: CreateChildChapterForm) => void
  onMove?: (chapterId: number, newParentId?: number) => void
  onDrop?: (draggedId: number, targetId: number) => void
  onReorder?: (draggedId: number, targetId: number, position: 'before' | 'after') => void
  level?: number
}

const createChildChapterSchema = z.object({
  title: z.string().min(1, '章节标题不能为空'),
  description: z.string().optional()
})

type CreateChildChapterForm = z.infer<typeof createChildChapterSchema>

interface ChapterItemProps {
  chapter: ChapterTree
  bookId: number
  allChapters: ChapterTree[]
  siblings: ChapterTree[]
  level?: number
  onEdit?: (chapter: ChapterTree) => void
  onDelete?: (chapter: ChapterTree) => void
  onCreateChild?: (parentId: number, params: CreateChildChapterForm) => void
  onMove?: (chapterId: number, newParentId?: number) => void
  onDrop?: (draggedId: number, targetId: number) => void
  onReorder?: (draggedId: number, targetId: number, position: 'before' | 'after') => void
}

function ChapterItem({
  chapter,
  bookId,
  allChapters,
  siblings,
  level = 0,
  onEdit,
  onDelete,
  onCreateChild,
  onMove,
  onDrop,
  onReorder
}: ChapterItemProps) {
  const [isExpanded, setIsExpanded] = useState(true)
  const [showCreateDialog, setShowCreateDialog] = useState(false)
  const [showMoveDialog, setShowMoveDialog] = useState(false)
  const [isDragging, setIsDragging] = useState(false)
  const [isDragOver, setIsDragOver] = useState(false)
  const [dropPosition, setDropPosition] = useState<'before' | 'after' | 'inside' | null>(null)

  const form = useForm<CreateChildChapterForm>({
    resolver: zodResolver(createChildChapterSchema),
    defaultValues: {
      title: '',
      description: ''
    }
  })

  const handleCreateChild = (data: CreateChildChapterForm) => {
    // 直接在这里创建子章节，不需要通过外部回调
    onCreateChild?.(chapter.id, data)
    setShowCreateDialog(false)
    form.reset()
  }

  const handleExpand = () => {
    setIsExpanded(!isExpanded)
  }

  const handleDragStart = (e: React.DragEvent) => {
    setIsDragging(true)
    e.dataTransfer.effectAllowed = 'move'
    e.dataTransfer.setData('text/plain', chapter.id.toString())
  }

  const handleDragEnd = () => {
    setIsDragging(false)
  }

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault()
    e.dataTransfer.dropEffect = 'move'

    const rect = e.currentTarget.getBoundingClientRect()
    const y = e.clientY - rect.top
    const midY = rect.height / 2

    // 检查是否是同级拖拽
    const draggedId = parseInt(e.dataTransfer.getData('text/plain'))
    const isSameLevel = siblings.some(sibling => sibling.id === draggedId)

    setIsDragOver(true)

    if (isSameLevel) {
      // 同级排序：判断插入位置
      if (y < midY) {
        setDropPosition('before')
      } else {
        setDropPosition('after')
      }
    } else {
      // 跨级移动：作为子章节
      setDropPosition('inside')
    }
  }

  const handleDragLeave = (e: React.DragEvent) => {
    // 只有当真正离开元素时才重置状态
    const rect = e.currentTarget.getBoundingClientRect()
    const x = e.clientX
    const y = e.clientY

    if (x < rect.left || x > rect.right || y < rect.top || y > rect.bottom) {
      setIsDragOver(false)
      setDropPosition(null)
    }
  }

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault()
    setIsDragOver(false)
    setDropPosition(null)

    const draggedId = parseInt(e.dataTransfer.getData('text/plain'))
    if (draggedId === chapter.id) return

    const isSameLevel = siblings.some(sibling => sibling.id === draggedId)

    if (isSameLevel && onReorder && (dropPosition === 'before' || dropPosition === 'after')) {
      // 同级排序
      onReorder(draggedId, chapter.id, dropPosition)
    } else if (!isSameLevel && onDrop) {
      // 跨级移动（成为子章节）
      onDrop(draggedId, chapter.id)
    }
  }

  const hasChildren = chapter.children && chapter.children.length > 0

  // 生成所有可能的父章节选项（排除自己和子章节）
  const getParentOptions = (): { id: number; title: string; path: string }[] => {
    const options: { id: number; title: string; path: string }[] = []

    // 添加根级选项
    options.push({ id: 0, title: '根级', path: '' })

    const addChapters = (chapters: ChapterTree[], currentPath = '') => {
      chapters.forEach(ch => {
        // 排除当前章节和它的子章节
        if (ch.id !== chapter.id) {
          const path = currentPath ? `${currentPath} / ${ch.title}` : ch.title
          options.push({ id: ch.id, title: ch.title, path })

          // 递归添加子章节
          if (ch.children && ch.children.length > 0) {
            addChapters(ch.children, path)
          }
        }
      })
    }

    addChapters(allChapters)
    return options
  }

  const handleMoveToParent = (parentId: number) => {
    onMove?.(chapter.id, parentId === 0 ? undefined : parentId)
    setShowMoveDialog(false)
  }

  return (
    <div className="select-none">
      {/* 上方插入指示器 */}
      {isDragOver && dropPosition === 'before' && (
        <div
          className="h-0.5 bg-blue-500 rounded-full mx-3 mb-1 transition-all"
          style={{ marginLeft: `${level * 24 + 12}px` }}
        />
      )}

      <div
        className={`
          flex items-center gap-2 p-3 rounded-lg border transition-all
          hover:bg-accent/50 cursor-pointer
          ${isDragging ? 'opacity-50 border-dashed bg-muted/30 scale-95' : ''}
          ${isDragOver ?
            dropPosition === 'inside'
              ? 'border-blue-500 bg-blue-50/50 border-2 shadow-sm'
              : 'border-blue-300 bg-blue-50/30 border'
            : ''
          }
        `}
        style={{ marginLeft: `${level * 24}px` }}
        draggable
        onDragStart={handleDragStart}
        onDragEnd={handleDragEnd}
        onDragOver={handleDragOver}
        onDragLeave={handleDragLeave}
        onDrop={handleDrop}
      >
        {/* 展开/收起按钮 */}
        <Button
          variant="ghost"
          size="sm"
          className="h-6 w-6 p-0"
          onClick={handleExpand}
        >
          {hasChildren ? (
            isExpanded ? (
              <ChevronDown className="w-4 h-4" />
            ) : (
              <ChevronRight className="w-4 h-4" />
            )
          ) : (
            <div className="w-4 h-4" />
          )}
        </Button>

        {/* 增强的拖拽手柄 */}
        <div
          className={`
            cursor-grab active:cursor-grabbing transition-all
            ${isDragging ? 'opacity-30' : 'opacity-60 hover:opacity-100'}
            p-1 rounded hover:bg-accent
          `}
          draggable
          onDragStart={handleDragStart}
          onDragEnd={handleDragEnd}
          title="拖拽重新排序或移动章节"
        >
          <GripVertical className="w-4 h-4 text-muted-foreground" />
        </div>

        {/* 章节图标 */}
        <div className="w-8 h-8 rounded-lg bg-blue-100 flex items-center justify-center">
          <BookOpen className="w-4 h-4 text-blue-600" />
        </div>

        {/* 章节信息 */}
        <div className="flex-1 min-w-0">
          <div className="font-medium truncate">{chapter.title}</div>
          <div className="text-sm text-muted-foreground">
            {chapter.media_count} 个媒体文件
            {chapter.level !== undefined && ` • 层级 ${chapter.level}`}
          </div>
        </div>

        {/* 操作按钮 */}
        <div className="flex items-center gap-1">
          <Button
            variant="ghost"
            size="sm"
            className="h-8 w-8 p-0"
            onClick={() => setShowCreateDialog(true)}
          >
            <Plus className="w-4 h-4" />
          </Button>

          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button variant="ghost" size="sm" className="h-8 w-8 p-0">
                <MoreHorizontal className="w-4 h-4" />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end">
              <DropdownMenuItem onClick={() => onEdit?.(chapter)}>
                <Edit className="w-4 h-4 mr-2" />
                编辑
              </DropdownMenuItem>
              <DropdownMenuItem onClick={() => onMove?.(chapter.id, undefined)}>
                <ChevronRight className="w-4 h-4 mr-2" />
                移动到根级
              </DropdownMenuItem>
              <DropdownMenuItem onClick={() => setShowMoveDialog(true)}>
                <ChevronRight className="w-4 h-4 mr-2" />
                移动到父章节
              </DropdownMenuItem>
              <DropdownMenuItem
                className="text-destructive"
                onClick={() => onDelete?.(chapter)}
              >
                <Trash2 className="w-4 h-4 mr-2" />
                删除
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        </div>
      </div>

      {/* 子章节 */}
      {isExpanded && hasChildren && (
        <div className="mt-1">
          {chapter.children.map((child) => (
            <ChapterItem
              key={child.id}
              chapter={child}
              bookId={bookId}
              allChapters={allChapters}
              siblings={chapter.children || []}
              level={level + 1}
              onEdit={onEdit}
              onDelete={onDelete}
              onCreateChild={onCreateChild}
              onMove={onMove}
              onDrop={onDrop}
              onReorder={onReorder}
            />
          ))}
        </div>
      )}

      {/* 创建子章节对话框 */}
      <Dialog open={showCreateDialog} onOpenChange={setShowCreateDialog}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>创建子章节</DialogTitle>
            <DialogDescription>
              为章节 "{chapter.title}" 创建子章节
            </DialogDescription>
          </DialogHeader>
          <Form {...form}>
            <form onSubmit={form.handleSubmit(handleCreateChild)} className="space-y-4">
              <FormField
                control={form.control}
                name="title"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>章节标题</FormLabel>
                    <FormControl>
                      <Input
                        placeholder="输入章节标题"
                        {...field}
                      />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
              <FormField
                control={form.control}
                name="description"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>章节描述</FormLabel>
                    <FormControl>
                      <Textarea
                        placeholder="输入章节描述（可选）"
                        {...field}
                      />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
              <div className="flex justify-end space-x-2">
                <Button type="button" variant="outline" onClick={() => setShowCreateDialog(false)}>
                  取消
                </Button>
                <Button type="submit">
                  创建
                </Button>
              </div>
            </form>
          </Form>
        </DialogContent>
      </Dialog>

      {/* 移动到父章节对话框 */}
      <Dialog open={showMoveDialog} onOpenChange={setShowMoveDialog}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>移动章节</DialogTitle>
            <DialogDescription>
              将章节 "{chapter.title}" 移动到新的父章节
            </DialogDescription>
          </DialogHeader>
          <div className="space-y-4">
            <div className="space-y-2">
              <label className="text-sm font-medium">选择父章节</label>
              <div className="max-h-60 overflow-y-auto space-y-1">
                {getParentOptions().map((option) => (
                  <button
                    key={option.id}
                    onClick={() => handleMoveToParent(option.id)}
                    className="w-full text-left p-2 rounded hover:bg-accent border transition-colors"
                  >
                    <div className="font-medium">{option.title}</div>
                    {option.path && (
                      <div className="text-xs text-muted-foreground">
                        路径: {option.path}
                      </div>
                    )}
                  </button>
                ))}
              </div>
            </div>
            <div className="flex justify-end space-x-2">
              <Button variant="outline" onClick={() => setShowMoveDialog(false)}>
                取消
              </Button>
            </div>
          </div>
        </DialogContent>
      </Dialog>

      {/* 下方插入指示器 */}
      {isDragOver && dropPosition === 'after' && (
        <div
          className="h-0.5 bg-blue-500 rounded-full mx-3 mt-1 transition-all"
          style={{ marginLeft: `${level * 24 + 12}px` }}
        />
      )}
    </div>
  )
}

export default function ChapterTree({
  chapters,
  bookId,
  onEdit,
  onDelete,
  onCreateChild,
  onMove,
  onDrop,
  onReorder
}: ChapterTreeProps) {
  if (chapters.length === 0) {
    return (
      <div className="text-center py-12 text-muted-foreground">
        <BookOpen className="w-16 h-16 mx-auto mb-4 opacity-30" />
        <h3 className="text-lg font-medium mb-2">暂无章节</h3>
        <p>创建您的第一个章节开始组织内容</p>
      </div>
    )
  }

  return (
    <div className="space-y-1">
      {chapters.map((chapter) => (
        <ChapterItem
          key={chapter.id}
          chapter={chapter}
          bookId={bookId}
          allChapters={chapters}
          siblings={chapters}
          level={0}
          onEdit={onEdit}
          onDelete={onDelete}
          onCreateChild={onCreateChild}
          onMove={onMove}
          onDrop={onDrop}
          onReorder={onReorder}
        />
      ))}
    </div>
  )
}