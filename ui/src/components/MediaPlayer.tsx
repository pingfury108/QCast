import { useState, useRef, useEffect } from 'react'
import { Download } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Dialog, DialogContent, DialogHeader, DialogTitle } from '@/components/ui/dialog'
import type { Media } from '../services/medias'

interface MediaPlayerProps {
  media: Media
  onClose?: () => void
  isOpen: boolean
}

export function MediaPlayer({ media, onClose, isOpen }: MediaPlayerProps) {
  const [error, setError] = useState<string | null>(null)
  const [mediaUrl, setMediaUrl] = useState<string | null>(null)
  const playerRef = useRef<HTMLAudioElement | HTMLVideoElement>(null)

  // 直接使用媒体URL（stream_media端点已取消认证要求）
  useEffect(() => {
    if (isOpen && media) {
      // 直接使用stream URL
      const streamUrl = `/api/media/${media.id}/stream`
      console.log('使用流媒体URL:', streamUrl)

      setMediaUrl(streamUrl)
      // 重置状态
      setError(null)
    }
  }, [media.id, isOpen])

  // 媒体事件监听器 - 简化版本
  useEffect(() => {
    const player = playerRef.current
    if (!player || !mediaUrl) return

    console.log('设置媒体事件监听器，URL:', mediaUrl)

    const handleError = (e: Event) => {
      console.error('媒体播放错误:', e)
      setError('播放出错')
    }

    // 只监听关键事件
    player.addEventListener('error', handleError)

    return () => {
      player.removeEventListener('error', handleError)
    }
  }, [mediaUrl, media.duration])

  const downloadMedia = () => {
    if (mediaUrl) {
      const link = document.createElement('a')
      link.href = mediaUrl
      link.download = media.title || 'media'
      link.download = `${link.download}.${media.file_type === 'audio' ? 'mp3' : 'mp4'}`
      document.body.appendChild(link)
      link.click()
      document.body.removeChild(link)
    }
  }

  const isVideo = media.file_type === 'video'

  return (
    <Dialog open={isOpen} onOpenChange={onClose}>
      <DialogContent className="max-w-6xl w-full min-w-[800px]">
        <DialogHeader>
          <div className="flex items-center justify-between pr-8">
            <DialogTitle className="truncate">{media.title}</DialogTitle>
            <Button
              variant="outline"
              size="sm"
              onClick={downloadMedia}
              className="flex items-center gap-2"
            >
              <Download className="w-4 h-4" />
              下载
            </Button>
          </div>
        </DialogHeader>

        {/* 原生媒体播放器 - 简化设计 */}
          {mediaUrl && (
            <>
              {isVideo ? (
                <video
                  ref={playerRef as React.RefObject<HTMLVideoElement>}
                  src={mediaUrl}
                  className="w-full rounded-lg"
                  style={{ aspectRatio: '16/9' }}
                  controls
                />
              ) : (
                <audio
                  ref={playerRef as React.RefObject<HTMLAudioElement>}
                  src={mediaUrl}
                  className="w-full"
                  controls
                />
              )}
            </>
          )}

          {/* 错误提示 */}
          {error && (
            <div className="p-4 text-center">
              <p className="text-red-600">{error}</p>
              <Button
                variant="outline"
                size="sm"
                onClick={() => setError(null)}
                className="mt-2"
              >
                重试
              </Button>
            </div>
          )}
      </DialogContent>
    </Dialog>
  )
}