import { useState, useEffect, useMemo } from 'react'
import { ChevronRight, Check } from 'lucide-react'
import { cn } from '@/lib/utils'
import { Popover, PopoverContent, PopoverTrigger } from '@/components/ui/popover'
import { Button } from '@/components/ui/button'
import type { Chapter } from '../services/chapters'

interface ChapterCascaderProps {
  chapters: Chapter[]
  value: string
  onChange: (value: string) => void
}

interface ChapterNode extends Chapter {
  children: ChapterNode[]
}

// 将扁平的章节列表转换为树形结构
function buildTree(chapters: Chapter[]): ChapterNode[] {
  const map = new Map<number, ChapterNode>()
  const roots: ChapterNode[] = []

  // 首先创建所有节点
  chapters.forEach(chapter => {
    map.set(chapter.id, { ...chapter, children: [] })
  })

  // 然后构建树形关系
  chapters.forEach(chapter => {
    const node = map.get(chapter.id)!
    if (chapter.parent_id && map.has(chapter.parent_id)) {
      map.get(chapter.parent_id)!.children.push(node)
    } else {
      roots.push(node)
    }
  })

  return roots
}

// 根据 ID 查找章节的完整路径
function findChapterPath(chapters: Chapter[], id: number): Chapter[] {
  const path: Chapter[] = []
  let currentId: number | undefined = id

  while (currentId) {
    const chapter = chapters.find(c => c.id === currentId)
    if (!chapter) break
    path.unshift(chapter)
    currentId = chapter.parent_id || undefined
  }

  return path
}

export function ChapterCascader({ chapters, value, onChange }: ChapterCascaderProps) {
  const [open, setOpen] = useState(false)
  const [activeColumns, setActiveColumns] = useState<ChapterNode[][]>([])

  // 使用 useMemo 缓存 tree，避免每次渲染都创建新对象
  const tree = useMemo(() => buildTree(chapters), [chapters])

  // 根据当前值初始化列
  useEffect(() => {
    if (value === '0') {
      setActiveColumns([tree])
      return
    }

    const selectedId = parseInt(value)
    if (!selectedId || isNaN(selectedId)) {
      setActiveColumns([tree])
      return
    }

    const path = findChapterPath(chapters, selectedId)
    const columns: ChapterNode[][] = [tree]

    path.forEach((pathChapter, index) => {
      if (index > 0) {
        const parentNode = columns[index].find(n => n.id === pathChapter.parent_id)
        if (parentNode && parentNode.children.length > 0) {
          columns.push(parentNode.children)
        }
      } else {
        const rootNode = tree.find(n => n.id === pathChapter.id)
        if (rootNode && rootNode.children.length > 0) {
          columns.push(rootNode.children)
        }
      }
    })

    setActiveColumns(columns)
  }, [value, chapters, tree])

  const handleSelect = (node: ChapterNode | null, columnIndex: number) => {
    if (!node) {
      // 选择"无章节"
      onChange('0')
      setActiveColumns([tree])
      setOpen(false)
      return
    }

    onChange(node.id.toString())

    // 更新列显示
    const newColumns = activeColumns.slice(0, columnIndex + 1)
    if (node.children.length > 0) {
      newColumns.push(node.children)
    }
    setActiveColumns(newColumns)

    // 如果是叶子节点，关闭弹窗
    if (node.children.length === 0) {
      setOpen(false)
    }
  }

  // 获取显示文本
  const getDisplayText = () => {
    if (value === '0') return '无章节'
    const selectedId = parseInt(value)
    if (!selectedId || isNaN(selectedId)) return '选择章节'

    const chapter = chapters.find(c => c.id === selectedId)
    if (!chapter) return '选择章节'

    // 如果有 path，将 ID 路径转换为名称路径
    if (chapter.path) {
      const pathIds = chapter.path.split('/').map(id => parseInt(id))
      const pathNames = pathIds
        .map(id => chapters.find(c => c.id === id)?.title)
        .filter(Boolean)
        .join(' > ')
      return pathNames || chapter.title
    }

    return chapter.title
  }

  return (
    <Popover open={open} onOpenChange={setOpen}>
      <PopoverTrigger asChild>
        <Button
          variant="outline"
          role="combobox"
          aria-expanded={open}
          className="w-full justify-between"
        >
          {getDisplayText()}
          <ChevronRight className="ml-2 h-4 w-4 shrink-0 opacity-50" />
        </Button>
      </PopoverTrigger>
      <PopoverContent className="w-auto p-0" align="start">
        <div className="flex">
          {activeColumns.map((column, columnIndex) => (
            <div
              key={columnIndex}
              className="w-48 border-r last:border-r-0 max-h-[300px] overflow-y-auto"
            >
              {columnIndex === 0 && (
                <div
                  className={cn(
                    'flex items-center justify-between px-3 py-2 text-sm cursor-pointer hover:bg-accent',
                    value === '0' && 'bg-accent'
                  )}
                  onClick={() => handleSelect(null, columnIndex)}
                >
                  <span>无章节</span>
                  {value === '0' && <Check className="h-4 w-4" />}
                </div>
              )}
              {column.map(node => {
                const isSelected = value === node.id.toString()
                return (
                  <div
                    key={node.id}
                    className={cn(
                      'flex items-center justify-between px-3 py-2 text-sm cursor-pointer hover:bg-accent',
                      isSelected && 'bg-accent'
                    )}
                    onClick={() => handleSelect(node, columnIndex)}
                  >
                    <span className="flex-1">{node.title}</span>
                    <div className="flex items-center gap-1">
                      {isSelected && <Check className="h-4 w-4" />}
                      {node.children.length > 0 && (
                        <ChevronRight className="h-4 w-4 opacity-50" />
                      )}
                    </div>
                  </div>
                )
              })}
            </div>
          ))}
        </div>
      </PopoverContent>
    </Popover>
  )
}
