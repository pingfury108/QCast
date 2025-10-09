import { useMedias } from '../hooks/useMedias'
import { useChapters } from '../hooks/useChapters'
import { Music, Video, Eye, EyeOff, MoreHorizontal, Copy, QrCode, Edit, Trash, Upload, RefreshCw } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger, DropdownMenuSeparator } from '@/components/ui/dropdown-menu'
import { toast } from 'sonner'
import type { Media } from '../services/medias'
import { useMemo, useState } from 'react'
import { EditMediaDialog } from '../components/EditMediaDialog'
import { ReplaceMediaFileDialog } from '../components/ReplaceMediaFileDialog'
import { mediasService } from '../services/medias'
import { useQueryClient } from '@tanstack/react-query'

export default function MediaPage() {
  const { data: medias = [], isLoading, error } = useMedias()
  const queryClient = useQueryClient()

  const [editingMedia, setEditingMedia] = useState<Media | null>(null)
  const [editDialogOpen, setEditDialogOpen] = useState(false)
  const [replacingMedia, setReplacingMedia] = useState<Media | null>(null)
  const [replaceDialogOpen, setReplaceDialogOpen] = useState(false)

  // 获取所有涉及的 book_id
  const bookIds = useMemo(() => {
    const ids = new Set(medias.map(m => m.book_id))
    return Array.from(ids)
  }, [medias])

  // 获取所有书籍的章节数据
  const chapterQueries = bookIds.map(bookId =>
    // eslint-disable-next-line react-hooks/rules-of-hooks
    useChapters(bookId)
  )

  // 构建章节 ID 到章节路径的映射
  const chapterPathMap = useMemo(() => {
    const map = new Map<number, string>()
    chapterQueries.forEach(query => {
      if (query.data) {
        query.data.forEach(chapter => {
          map.set(chapter.id, chapter.path || chapter.title)
        })
      }
    })
    return map
  }, [chapterQueries])

  const handleCopyLink = (media: Media) => {
    if (media.access_url) {
      navigator.clipboard.writeText(media.access_url)
        .then(() => toast.success('链接已复制到剪贴板'))
        .catch(() => toast.error('复制链接失败'))
    }
  }

  const handleEdit = (media: Media) => {
    setEditingMedia(media)
    setEditDialogOpen(true)
  }

  const handleReplaceFile = (media: Media) => {
    setReplacingMedia(media)
    setReplaceDialogOpen(true)
  }

  const handleReplaceSuccess = async () => {
    await queryClient.invalidateQueries({ queryKey: ['medias'] })
  }

  const handleSaveEdit = async (mediaId: number, data: any) => {
    await mediasService.updateMedia(mediaId, data)
    await queryClient.invalidateQueries({ queryKey: ['medias'] })
  }

  const handleDelete = async (media: Media) => {
    if (!confirm(`确定要删除"${media.title}"吗？此操作无法撤销。`)) {
      return
    }

    try {
      await mediasService.deleteMedia(media.id)
      await queryClient.invalidateQueries({ queryKey: ['medias'] })
      toast.success('删除成功')
    } catch (error: any) {
      console.error('删除失败:', error)
      toast.error(error?.response?.data?.error || '删除失败')
    }
  }

  // 获取当前编辑媒体所属书籍的章节列表
  const editMediaChapters = useMemo(() => {
    if (!editingMedia) return []
    const query = chapterQueries[bookIds.indexOf(editingMedia.book_id)]
    return query?.data || []
  }, [editingMedia, chapterQueries, bookIds])

  const formatDuration = (seconds?: number) => {
    if (!seconds) return '--:--'
    const mins = Math.floor(seconds / 60)
    const secs = seconds % 60
    return `${mins}:${secs.toString().padStart(2, '0')}`
  }

  if (isLoading) {
    return (
      <div className="container mx-auto px-4 py-8">
        <div className="text-center py-8">加载中...</div>
      </div>
    )
  }

  if (error) {
    return (
      <div className="container mx-auto px-4 py-8">
        <div className="text-center text-destructive">加载失败</div>
      </div>
    )
  }

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="flex items-center justify-between mb-8">
        <h1 className="text-3xl font-bold">所有媒体</h1>
        <Button asChild>
          <a href="/dashboard/upload">
            + 上传媒体
          </a>
        </Button>
      </div>

      {medias.length === 0 ? (
        <div className="bg-card rounded-lg border p-8 text-center">
          <Music className="w-12 h-12 mx-auto mb-4 opacity-50" />
          <p className="text-muted-foreground mb-4">暂无媒体文件</p>
          <Button asChild>
            <a href="/dashboard/upload">上传第一个媒体</a>
          </Button>
        </div>
      ) : (
        <div className="bg-card rounded-lg border">
          <div className="grid gap-4 p-4">
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
                      {media.chapter_id && chapterPathMap.has(media.chapter_id) && (
                        <> • {chapterPathMap.get(media.chapter_id)}</>
                      )}
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
                      <DropdownMenuItem onClick={() => handleEdit(media)}>
                        <Edit className="w-4 h-4 mr-2" />
                        编辑
                      </DropdownMenuItem>
                      <DropdownMenuItem onClick={() => handleReplaceFile(media)}>
                        <RefreshCw className="w-4 h-4 mr-2" />
                        替换文件
                      </DropdownMenuItem>
                      <DropdownMenuItem onClick={() => handleCopyLink(media)}>
                        <Copy className="w-4 h-4 mr-2" />
                        复制链接
                      </DropdownMenuItem>
                      {media.qr_code_path && (
                        <DropdownMenuItem>
                          <QrCode className="w-4 h-4 mr-2" />
                          查看二维码
                        </DropdownMenuItem>
                      )}
                      <DropdownMenuSeparator />
                      <DropdownMenuItem
                        onClick={() => handleDelete(media)}
                        className="text-destructive focus:text-destructive"
                      >
                        <Trash className="w-4 h-4 mr-2" />
                        删除
                      </DropdownMenuItem>
                    </DropdownMenuContent>
                  </DropdownMenu>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      <EditMediaDialog
        media={editingMedia}
        open={editDialogOpen}
        onOpenChange={setEditDialogOpen}
        onSave={handleSaveEdit}
        chapters={editMediaChapters}
      />

      <ReplaceMediaFileDialog
        media={replacingMedia}
        open={replaceDialogOpen}
        onOpenChange={setReplaceDialogOpen}
        onSuccess={handleReplaceSuccess}
      />
    </div>
  );
}