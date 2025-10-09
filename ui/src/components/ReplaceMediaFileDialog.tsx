import { useState } from 'react'
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogFooter } from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Progress } from '@/components/ui/progress'
import { toast } from 'sonner'
import type { Media } from '../services/medias'

interface ReplaceMediaFileDialogProps {
  media: Media | null
  open: boolean
  onOpenChange: (open: boolean) => void
  onSuccess: () => void
}

export function ReplaceMediaFileDialog({
  media,
  open,
  onOpenChange,
  onSuccess
}: ReplaceMediaFileDialogProps) {
  const [file, setFile] = useState<File | null>(null)
  const [uploading, setUploading] = useState(false)
  const [progress, setProgress] = useState(0)

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const selectedFile = e.target.files?.[0]
    if (selectedFile) {
      // 验证文件类型
      const isAudio = selectedFile.type.startsWith('audio/')
      const isVideo = selectedFile.type.startsWith('video/')

      if (!isAudio && !isVideo) {
        toast.error('请选择音频或视频文件')
        return
      }

      // 验证文件大小（2GB限制）
      const maxSize = 2 * 1024 * 1024 * 1024
      if (selectedFile.size > maxSize) {
        toast.error('文件大小不能超过 2GB')
        return
      }

      setFile(selectedFile)
    }
  }

  const handleReplace = async () => {
    if (!media || !file) {
      toast.error('请选择要上传的文件')
      return
    }

    setUploading(true)
    setProgress(0)

    try {
      const formData = new FormData()
      formData.append('file', file)

      const { api } = await import('../lib/api')
      await api.put(`/media/${media.id}/replace-file`, formData, {
        onUploadProgress: (progressEvent) => {
          if (progressEvent.total) {
            const percentCompleted = Math.round((progressEvent.loaded * 100) / progressEvent.total)
            setProgress(percentCompleted)
          }
        }
      })

      toast.success('文件替换成功')
      onSuccess()
      onOpenChange(false)
      setFile(null)
      setProgress(0)
    } catch (error: any) {
      console.error('替换文件失败:', error)
      toast.error(error?.response?.data?.error || '替换文件失败')
    } finally {
      setUploading(false)
    }
  }

  const handleClose = () => {
    if (!uploading) {
      onOpenChange(false)
      setFile(null)
      setProgress(0)
    }
  }

  return (
    <Dialog open={open} onOpenChange={handleClose}>
      <DialogContent className="sm:max-w-[500px]">
        <DialogHeader>
          <DialogTitle>替换媒体文件</DialogTitle>
        </DialogHeader>

        <div className="grid gap-4 py-4">
          {media && (
            <div className="text-sm text-muted-foreground">
              当前文件: <span className="font-medium">{media.original_filename || media.title}</span>
            </div>
          )}

          <div className="grid gap-2">
            <Label htmlFor="file">选择新文件 *</Label>
            <Input
              id="file"
              type="file"
              accept="audio/*,video/*"
              onChange={handleFileChange}
              disabled={uploading}
            />
            {file && (
              <div className="text-sm text-muted-foreground">
                已选择: {file.name} ({(file.size / 1024 / 1024).toFixed(2)} MB)
              </div>
            )}
          </div>

          {uploading && (
            <div className="space-y-2">
              <div className="flex justify-between text-sm">
                <span>上传中...</span>
                <span>{progress}%</span>
              </div>
              <Progress value={progress} />
            </div>
          )}

          <div className="text-sm text-muted-foreground">
            <p>注意事项：</p>
            <ul className="list-disc list-inside space-y-1 mt-1">
              <li>新文件将完全替换原有文件</li>
              <li>访问链接保持不变</li>
              <li>文件版本号将自动增加</li>
              <li>支持音频和视频文件，最大 2GB</li>
            </ul>
          </div>
        </div>

        <DialogFooter>
          <Button variant="outline" onClick={handleClose} disabled={uploading}>
            取消
          </Button>
          <Button onClick={handleReplace} disabled={!file || uploading}>
            {uploading ? `上传中 (${progress}%)` : '开始替换'}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
