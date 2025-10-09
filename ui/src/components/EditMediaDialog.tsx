import { useState, useEffect } from 'react'
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogFooter } from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Textarea } from '@/components/ui/textarea'
import { Switch } from '@/components/ui/switch'
import { toast } from 'sonner'
import type { Media } from '../services/medias'
import type { Chapter } from '../services/chapters'
import { ChapterCascader } from './ChapterCascader'

interface EditMediaDialogProps {
  media: Media | null
  open: boolean
  onOpenChange: (open: boolean) => void
  onSave: (mediaId: number, data: {
    title?: string
    description?: string
    is_public?: boolean
    chapter_id?: number
  }) => Promise<void>
  chapters: Chapter[]
}

export function EditMediaDialog({
  media,
  open,
  onOpenChange,
  onSave,
  chapters
}: EditMediaDialogProps) {
  const [title, setTitle] = useState('')
  const [description, setDescription] = useState('')
  const [isPublic, setIsPublic] = useState(false)
  const [chapterId, setChapterId] = useState<string>('0')
  const [saving, setSaving] = useState(false)

  useEffect(() => {
    if (media) {
      setTitle(media.title)
      setDescription(media.description || '')
      setIsPublic(media.is_public)
      setChapterId(media.chapter_id?.toString() || '0')
    }
  }, [media])

  const handleSave = async () => {
    if (!media) return

    if (!title.trim()) {
      toast.error('请输入标题')
      return
    }

    setSaving(true)
    try {
      const chapterIdNum = parseInt(chapterId)
      await onSave(media.id, {
        title: title.trim(),
        description: description.trim() || undefined,
        is_public: isPublic,
        chapter_id: chapterIdNum > 0 ? chapterIdNum : 0
      })
      toast.success('保存成功')
      onOpenChange(false)
    } catch (error: any) {
      console.error('保存失败:', error)
      toast.error(error?.response?.data?.error || '保存失败')
    } finally {
      setSaving(false)
    }
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-[500px]">
        <DialogHeader>
          <DialogTitle>编辑媒体</DialogTitle>
        </DialogHeader>

        <div className="grid gap-4 py-4">
          <div className="grid gap-2">
            <Label htmlFor="title">标题 *</Label>
            <Input
              id="title"
              value={title}
              onChange={(e) => setTitle(e.target.value)}
              placeholder="请输入标题"
            />
          </div>

          <div className="grid gap-2">
            <Label htmlFor="description">描述</Label>
            <Textarea
              id="description"
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              placeholder="请输入描述（可选）"
              rows={3}
            />
          </div>

          <div className="grid gap-2">
            <Label htmlFor="chapter">关联章节</Label>
            <ChapterCascader
              chapters={chapters}
              value={chapterId}
              onChange={setChapterId}
            />
          </div>

          <div className="flex items-center justify-between">
            <Label htmlFor="is-public">公开访问</Label>
            <Switch
              id="is-public"
              checked={isPublic}
              onCheckedChange={setIsPublic}
            />
          </div>
        </div>

        <DialogFooter>
          <Button variant="outline" onClick={() => onOpenChange(false)}>
            取消
          </Button>
          <Button onClick={handleSave} disabled={saving}>
            {saving ? '保存中...' : '保存'}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
