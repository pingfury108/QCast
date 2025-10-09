import { useState } from 'react'
import { ChevronRight, ChevronDown } from 'lucide-react'
import { cn } from '@/lib/utils'
import type { Chapter } from '../services/chapters'

interface ChapterTreeSelectProps {
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

interface ChapterTreeNodeProps {
  node: ChapterNode
  level: number
  value: string
  onChange: (value: string) => void
}

function ChapterTreeNode({ node, level, value, onChange }: ChapterTreeNodeProps) {
  const [isExpanded, setIsExpanded] = useState(true)
  const hasChildren = node.children.length > 0
  const isSelected = value === node.id.toString()

  return (
    <div>
      <div
        className={cn(
          'flex items-center py-2 px-2 rounded hover:bg-accent cursor-pointer transition-colors',
          isSelected && 'bg-accent'
        )}
        style={{ paddingLeft: `${level * 1.5 + 0.5}rem` }}
        onClick={() => onChange(node.id.toString())}
      >
        {hasChildren ? (
          <button
            className="mr-1 p-0.5 hover:bg-muted rounded"
            onClick={(e) => {
              e.stopPropagation()
              setIsExpanded(!isExpanded)
            }}
          >
            {isExpanded ? (
              <ChevronDown className="w-4 h-4" />
            ) : (
              <ChevronRight className="w-4 h-4" />
            )}
          </button>
        ) : (
          <span className="w-5 mr-1" />
        )}
        <span className={cn('text-sm', isSelected && 'font-medium')}>
          {node.title}
        </span>
      </div>
      {hasChildren && isExpanded && (
        <div>
          {node.children.map(child => (
            <ChapterTreeNode
              key={child.id}
              node={child}
              level={level + 1}
              value={value}
              onChange={onChange}
            />
          ))}
        </div>
      )}
    </div>
  )
}

export function ChapterTreeSelect({ chapters, value, onChange }: ChapterTreeSelectProps) {
  const tree = buildTree(chapters)

  return (
    <div className="border rounded-md max-h-[300px] overflow-y-auto">
      <div
        className={cn(
          'flex items-center py-2 px-2 rounded hover:bg-accent cursor-pointer transition-colors',
          value === '0' && 'bg-accent'
        )}
        onClick={() => onChange('0')}
      >
        <span className="w-5 mr-1" />
        <span className={cn('text-sm', value === '0' && 'font-medium')}>
          无章节
        </span>
      </div>
      {tree.map(node => (
        <ChapterTreeNode
          key={node.id}
          node={node}
          level={0}
          value={value}
          onChange={onChange}
        />
      ))}
    </div>
  )
}
