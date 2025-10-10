import { useState, useRef, useEffect } from 'react'
import { Play, Pause, Volume2, VolumeX, X, Download } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Slider } from '@/components/ui/slider'
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogDescription } from '@/components/ui/dialog'
import type { Media } from '../services/medias'

interface MediaPlayerProps {
  media: Media
  onClose?: () => void
  isOpen: boolean
}

export function MediaPlayer({ media, onClose, isOpen }: MediaPlayerProps) {
  const [playing, setPlaying] = useState(false)
  const [volume, setVolume] = useState(0.8)
  const [muted, setMuted] = useState(false)
  const [played, setPlayed] = useState(0)
  const [duration, setDuration] = useState(0)
  const [seeking, setSeeking] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [mediaUrl, setMediaUrl] = useState<string | null>(null)
  const [loading, setLoading] = useState(true)
  const playerRef = useRef<HTMLAudioElement | HTMLVideoElement>(null)

  // 直接使用媒体URL（stream_media端点已取消认证要求）
  useEffect(() => {
    if (isOpen && media) {
      // 直接使用stream URL
      const streamUrl = `/api/media/${media.id}/stream`
      console.log('使用流媒体URL:', streamUrl)

      setMediaUrl(streamUrl)
      // 重置状态
      setDuration(0)
      setPlayed(0)
      setPlaying(false)
      setError(null)
      setLoading(false) // 直接设为false，不显示加载状态
    }
  }, [media.id, isOpen])

  // 媒体事件监听器 - 简化版本
  useEffect(() => {
    const player = playerRef.current
    if (!player || !mediaUrl) return

    console.log('设置媒体事件监听器，URL:', mediaUrl)

    const handleLoadedMetadata = () => {
      console.log('媒体元数据加载完成，时长:', player.duration)
      if (player.duration && player.duration > 0) {
        setDuration(player.duration)
      }
    }

    const handleTimeUpdate = () => {
      if (player.duration && player.duration > 0) {
        setPlayed(player.currentTime / player.duration)
      }
    }

    const handleEnded = () => {
      setPlaying(false)
    }

    const handleError = (e: Event) => {
      console.error('媒体播放错误:', e)
      setError('播放出错')
      setPlaying(false)
    }

    // 只监听关键事件
    player.addEventListener('loadedmetadata', handleLoadedMetadata)
    player.addEventListener('timeupdate', handleTimeUpdate)
    player.addEventListener('ended', handleEnded)
    player.addEventListener('error', handleError)

    // 延迟检查，给浏览器时间解析媒体
    const checkTimer = setTimeout(() => {
      if (player.duration && player.duration > 0) {
        console.log('延迟检查发现时长:', player.duration)
        setDuration(player.duration)
      } else {
        console.log('延迟检查未发现时长，使用数据库值:', media.duration)
        if (media.duration && media.duration > 0) {
          setDuration(media.duration as number)
        }
      }
    }, 1000)

    return () => {
      player.removeEventListener('loadedmetadata', handleLoadedMetadata)
      player.removeEventListener('timeupdate', handleTimeUpdate)
      player.removeEventListener('ended', handleEnded)
      player.removeEventListener('error', handleError)
      clearTimeout(checkTimer)
    }
  }, [mediaUrl, media.duration])

  const handlePlayPause = () => {
    const player = playerRef.current
    if (!player) return

    if (playing) {
      player.pause()
    } else {
      player.play().catch(err => {
        console.error('播放失败:', err)
        setError('播放失败')
      })
    }
    setPlaying(!playing)
  }

  const handleSeekChange = (value: number[]) => {
    setPlayed(value[0])
    setSeeking(true)
  }

  const handleSeekMouseUp = (value: number[]) => {
    const player = playerRef.current
    if (!player) return

    setSeeking(false)
    const newTime = value[0] * player.duration
    if (!isNaN(newTime) && isFinite(newTime)) {
      player.currentTime = newTime
    }
  }

  const handleVolumeChange = (value: number[]) => {
    const player = playerRef.current
    if (!player) return

    setVolume(value[0])
    player.volume = value[0]
  }

  const handleMute = () => {
    const player = playerRef.current
    if (!player) return

    setMuted(!muted)
    player.muted = !muted
  }

  const formatTime = (seconds: number) => {
    if (!seconds || isNaN(seconds) || !isFinite(seconds) || seconds < 0) {
      return '0:00'
    }

    const totalSeconds = Math.floor(seconds)
    const hours = Math.floor(totalSeconds / 3600)
    const minutes = Math.floor((totalSeconds % 3600) / 60)
    const secs = totalSeconds % 60

    if (hours > 0) {
      return `${hours}:${minutes.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`
    }
    return `${minutes}:${secs.toString().padStart(2, '0')}`
  }

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
          <DialogTitle className="flex items-center justify-between pr-8">
            <span className="truncate">{media.title}</span>
            <div className="flex gap-4 items-center">
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
          </DialogTitle>
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