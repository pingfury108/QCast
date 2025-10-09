import { useState, useRef, useEffect } from 'react'
import ReactPlayer from 'react-player'
import { Play, Pause, Volume2, VolumeX, Maximize, Minimize, X } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Slider } from '@/components/ui/slider'
import type { Media } from '../services/medias'
import { api } from '../lib/api'

interface MediaPlayerProps {
  media: Media
  onClose?: () => void
}

export function MediaPlayer({ media, onClose }: MediaPlayerProps) {
  const [playing, setPlaying] = useState(false) // 等待加载完成后再播放
  const [volume, setVolume] = useState(0.8)
  const [muted, setMuted] = useState(false)
  const [played, setPlayed] = useState(0)
  const [duration, setDuration] = useState(0)
  const [seeking, setSeeking] = useState(false)
  const [ready, setReady] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [mediaUrl, setMediaUrl] = useState<string | null>(null)
  const [loading, setLoading] = useState(true)
  const playerRef = useRef<ReactPlayer>(null)

  // 使用 Fetch API 获取媒体文件（可以携带认证 header）
  useEffect(() => {
    let objectUrl: string | null = null

    const loadMedia = async () => {
      try {
        setLoading(true)
        console.log('开始加载媒体:', media.id)

        const response = await api.get(`/media/${media.id}/stream`, {
          responseType: 'blob',
          timeout: 60000, // 60秒超时，适用于大文件
        })

        // 创建 Blob URL
        const blob = new Blob([response.data], { type: media.mime_type || 'video/mp4' })
        objectUrl = URL.createObjectURL(blob)

        console.log('媒体加载完成，Blob URL 已创建')
        setMediaUrl(objectUrl)
        setLoading(false)
        setPlaying(true) // 加载完成后自动播放
      } catch (err) {
        console.error('加载媒体失败:', err)
        setError('加载媒体失败')
        setLoading(false)
      }
    }

    loadMedia()

    // 清理函数：释放 Blob URL
    return () => {
      if (objectUrl) {
        console.log('释放 Blob URL')
        URL.revokeObjectURL(objectUrl)
      }
    }
  }, [media.id, media.mime_type])

  // 调试：打印媒体信息
  console.log('MediaPlayer - 媒体信息:', {
    id: media.id,
    title: media.title,
    is_public: media.is_public,
    file_type: media.file_type,
    mime_type: media.mime_type,
    has_url: !!mediaUrl,
    loading
  })

  const handlePlayPause = () => {
    setPlaying(!playing)
  }

  const handleProgress = (state: { played: number; playedSeconds: number; loaded: number; loadedSeconds: number }) => {
    if (!seeking) {
      setPlayed(state.played)
    }

    // 从播放器实例获取时长
    if (playerRef.current && duration === 0) {
      const d = playerRef.current.getDuration()
      if (d && d > 0) {
        setDuration(d)
      }
    }
  }

  const handleSeekMouseDown = () => {
    setSeeking(true)
  }

  const handleSeekChange = (value: number[]) => {
    setPlayed(value[0] / 100)
  }

  const handleSeekMouseUp = (value: number[]) => {
    setSeeking(false)
    playerRef.current?.seekTo(value[0] / 100)
  }

  const handleVolumeChange = (value: number[]) => {
    setVolume(value[0] / 100)
    setMuted(value[0] === 0)
  }

  const handleToggleMute = () => {
    setMuted(!muted)
  }

  const handleReady = () => {
    console.log('播放器就绪')
    setReady(true)
    setError(null)

    // 手动获取时长
    if (playerRef.current) {
      const d = playerRef.current.getDuration()
      console.log('从 onReady 获取时长:', d)
      if (d && d > 0) {
        setDuration(d)
      }
    }
  }

  const handleError = (err: any) => {
    console.error('播放器错误:', err)
    console.error('错误详情:', {
      message: err?.message,
      type: err?.type,
      url: mediaUrl,
      media
    })
    setError('播放失败，请检查媒体文件')
    setReady(true) // 仍然显示控制条
  }

  const formatTime = (seconds: number) => {
    if (!seconds || isNaN(seconds)) return '0:00'
    const mins = Math.floor(seconds / 60)
    const secs = Math.floor(seconds % 60)
    return `${mins}:${secs.toString().padStart(2, '0')}`
  }

  const isVideo = media.file_type === 'video'

  return (
    <div className="w-full bg-black rounded-lg overflow-hidden">
      <div className="relative" style={{ aspectRatio: isVideo ? '16/9' : 'auto', minHeight: isVideo ? '300px' : '80px' }}>
        {/* 播放器 */}
        {mediaUrl && (
          <ReactPlayer
            ref={playerRef}
            url={mediaUrl}
            playing={playing}
            volume={volume}
            muted={muted}
            controls={false}
            width="100%"
            height="100%"
            onReady={handleReady}
            onProgress={handleProgress}
            onError={handleError}
            config={{
              file: {
                attributes: {
                  controlsList: 'nodownload',
                  playsInline: true,
                },
                forceVideo: isVideo,
                forceAudio: !isVideo,
              }
            }}
          />
        )}

        {/* 加载/错误提示 */}
        {loading && (
          <div className="absolute inset-0 flex items-center justify-center bg-black">
            <div className="text-white/60">加载中...</div>
          </div>
        )}
        {error && (
          <div className="absolute inset-0 flex items-center justify-center bg-black">
            <div className="text-red-400 text-center">
              <div>{error}</div>
            </div>
          </div>
        )}

        {/* 自定义控制条 */}
        <div className="absolute bottom-0 left-0 right-0 bg-gradient-to-t from-black/80 to-transparent p-4">
          {/* 进度条 */}
          <div className="mb-3">
            <Slider
              value={[played * 100]}
              onValueChange={handleSeekChange}
              onPointerDown={handleSeekMouseDown}
              onPointerUp={(e) => {
                const value = [(e.currentTarget as HTMLInputElement).valueAsNumber]
                handleSeekMouseUp(value)
              }}
              max={100}
              step={0.1}
              className="cursor-pointer"
            />
            <div className="flex justify-between text-xs text-white/70 mt-1">
              <span>{formatTime(played * duration)}</span>
              <span>{formatTime(duration)}</span>
            </div>
          </div>

          {/* 控制按钮 */}
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <Button
                variant="ghost"
                size="icon"
                onClick={handlePlayPause}
                className="text-white hover:text-white hover:bg-white/20"
              >
                {playing ? (
                  <Pause className="w-5 h-5" />
                ) : (
                  <Play className="w-5 h-5" />
                )}
              </Button>

              <div className="flex items-center gap-2 ml-2">
                <Button
                  variant="ghost"
                  size="icon"
                  onClick={handleToggleMute}
                  className="text-white hover:text-white hover:bg-white/20"
                >
                  {muted ? (
                    <VolumeX className="w-4 h-4" />
                  ) : (
                    <Volume2 className="w-4 h-4" />
                  )}
                </Button>
                <div className="w-24">
                  <Slider
                    value={[muted ? 0 : volume * 100]}
                    onValueChange={handleVolumeChange}
                    max={100}
                    step={1}
                    className="cursor-pointer"
                  />
                </div>
              </div>
            </div>

            <div className="flex items-center gap-2">
              {onClose && (
                <Button
                  variant="ghost"
                  size="icon"
                  onClick={onClose}
                  className="text-white hover:text-white hover:bg-white/20"
                  title="关闭播放器"
                >
                  <X className="w-4 h-4" />
                </Button>
              )}
            </div>
          </div>
        </div>
      </div>

      {/* 媒体信息 */}
      <div className="bg-black/90 p-3 border-t border-white/10">
        <div className="text-white font-medium text-sm">{media.title}</div>
        {media.description && (
          <div className="text-white/60 text-xs mt-1">{media.description}</div>
        )}
      </div>
    </div>
  )
}
