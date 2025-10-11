import React, { useState } from 'react'
import { FolderOpen, Music, Video, Play, Download } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent } from '@/components/ui/card'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Badge } from '@/components/ui/badge'
import { MediaPlayer } from './MediaPlayer'
import { useChapterMedias } from '../hooks/useMedias'
import type { Media } from '../services/medias'
import type { ChapterTree } from '../hooks/useChapters'

interface MediaPreviewProps {
  chapters: ChapterTree[]
  onChapterSelect?: (chapter: ChapterTree) => void
}

interface ChapterMediaItemProps {
  media: Media
  onPlay: (media: Media) => void
  onDownload: (media: Media) => void
}

function ChapterMediaItem({ media, onPlay, onDownload }: ChapterMediaItemProps) {
  return (
    <Card className="mb-3 hover:shadow-md transition-shadow cursor-pointer">
      <CardContent className="p-4">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-3 flex-1 min-w-0">
            <div className="w-10 h-10 rounded-lg bg-primary/10 flex items-center justify-center flex-shrink-0">
              {media.file_type === 'audio' ? (
                <Music className="w-5 h-5 text-primary" />
              ) : media.file_type === 'video' ? (
                <Video className="w-5 h-5 text-primary" />
              ) : (
                <div className="w-5 h-5 bg-muted rounded" />
              )}
            </div>
            <div className="flex-1 min-w-0">
              <h4 className="font-medium text-sm truncate">{media.title}</h4>
              {media.description && (
                <p className="text-xs text-gray-500 mt-1 truncate">{media.description}</p>
              )}
              <div className="flex items-center gap-2 mt-2">
                <Badge variant="outline" className="text-xs">
                  {media.mime_type}
                </Badge>
                {media.file_size && (
                  <span className="text-xs text-gray-400">
                    {(media.file_size / 1024 / 1024).toFixed(1)} MB
                  </span>
                )}
                {media.duration && (
                  <span className="text-xs text-gray-400">
                    {Math.floor(media.duration / 60)}:{(media.duration % 60).toString().padStart(2, '0')}
                  </span>
                )}
              </div>
            </div>
          </div>
          <div className="flex items-center gap-2 ml-4">
            <Button
              variant="outline"
              size="sm"
              onClick={(e) => {
                e.stopPropagation()
                onPlay(media)
              }}
              className="flex items-center gap-1"
            >
              <Play className="w-3 h-3" />
              播放
            </Button>
            <Button
              variant="ghost"
              size="sm"
              onClick={(e) => {
                e.stopPropagation()
                onDownload(media)
              }}
              className="flex items-center gap-1"
            >
              <Download className="w-3 h-3" />
            </Button>
          </div>
        </div>
      </CardContent>
    </Card>
  )
}

function ChapterSidebar({
  chapters,
  selectedChapter,
  onChapterSelect,
  chapterMediaCounts
}: {
  chapters: ChapterTree[]
  selectedChapter?: ChapterTree
  onChapterSelect: (chapter: ChapterTree) => void
  chapterMediaCounts: Record<number, number>
}) {
  const [expandedChapters, setExpandedChapters] = useState<Set<number>>(new Set())

  const toggleExpand = (chapterId: number) => {
    const newExpanded = new Set(expandedChapters)
    if (newExpanded.has(chapterId)) {
      newExpanded.delete(chapterId)
    } else {
      newExpanded.add(chapterId)
    }
    setExpandedChapters(newExpanded)
  }

  const renderChapter = (chapter: ChapterTree, level: number = 0) => {
    const isExpanded = expandedChapters.has(chapter.id)
    const isSelected = selectedChapter?.id === chapter.id
    const mediaCount = chapterMediaCounts[chapter.id] || 0
    const hasChildren = chapter.children && chapter.children.length > 0

    return (
      <div key={chapter.id}>
        <div
          className={`flex items-center gap-2 py-2 px-3 rounded-lg cursor-pointer transition-colors ${
            isSelected ? 'bg-blue-50 border-l-4 border-blue-500' : ''
          }`}
          style={{ paddingLeft: `${level * 20 + 12}px` }}
          onClick={() => onChapterSelect(chapter)}
        >
          {hasChildren && (
            <button
              onClick={(e) => {
                e.stopPropagation()
                toggleExpand(chapter.id)
              }}
              className="p-1 hover:bg-gray-200 rounded"
            >
              {isExpanded ? (
                <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
                </svg>
              ) : (
                <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
                </svg>
              )}
            </button>
          )}
          <FolderOpen className="w-4 h-4 text-gray-600" />
          <span className="flex-1 text-sm truncate">{chapter.title}</span>
          {mediaCount > 0 && (
            <Badge variant="secondary" className="text-xs">
              {mediaCount}
            </Badge>
          )}
        </div>
        {isExpanded && hasChildren && (
          <div>
            {chapter.children!.map((child) => renderChapter(child, level + 1))}
          </div>
        )}
      </div>
    )
  }

  return (
    <div className="h-full">
      <div className="p-4 border-b">
        <h3 className="font-medium text-sm text-gray-700">章节目录</h3>
      </div>
      <ScrollArea className="flex-1">
        <div className="p-2">
          {chapters.map((chapter) => renderChapter(chapter))}
        </div>
      </ScrollArea>
    </div>
  )
}

export function MediaPreview({ chapters, onChapterSelect }: MediaPreviewProps) {
  const [selectedChapter, setSelectedChapter] = useState<ChapterTree | null>(null)
  const [selectedMedia, setSelectedMedia] = useState<Media | null>(null)
  const [isPlayerOpen, setIsPlayerOpen] = useState(false)
  const [chapterMediaCounts] = useState<Record<number, number>>({})

  // 使用hook获取章节媒体
  const { data: chapterMedias = [], isLoading, error } = useChapterMedias(
    selectedChapter?.id || 0
  )

  // 初始化时选择第一个章节
  React.useEffect(() => {
    if (chapters.length > 0 && !selectedChapter) {
      setSelectedChapter(chapters[0])
    }
  }, [chapters, selectedChapter])

  // 通知父组件章节选择
  React.useEffect(() => {
    if (selectedChapter) {
      onChapterSelect?.(selectedChapter)
    }
  }, [selectedChapter, onChapterSelect])

  const handlePlayMedia = (media: Media) => {
    setSelectedMedia(media)
    setIsPlayerOpen(true)
  }

  const handleDownloadMedia = (media: Media) => {
    // 实现下载功能
    const link = document.createElement('a')
    link.href = `/api/media/${media.id}/stream`
    link.download = `${media.title}.${media.file_type === 'audio' ? 'mp3' : 'mp4'}`
    document.body.appendChild(link)
    link.click()
    document.body.removeChild(link)
  }

  return (
    <div className="flex h-full bg-white">
      {/* 左侧章节树 */}
      <div className="w-80 border-r">
        <ChapterSidebar
          chapters={chapters}
          selectedChapter={selectedChapter || undefined}
          onChapterSelect={setSelectedChapter}
          chapterMediaCounts={chapterMediaCounts}
        />
      </div>

      {/* 右侧媒体内容 */}
      <div className="flex-1 flex flex-col">
        {selectedChapter && (
          <>
            {/* 章节标题 */}
            <div className="p-6 border-b bg-white">
              <h2 className="text-xl font-semibold text-gray-900">
                {selectedChapter.title}
              </h2>
              {selectedChapter.description && (
                <p className="text-gray-600 mt-2">{selectedChapter.description}</p>
              )}
            </div>

            {/* 媒体列表 */}
            <div className="flex-1 p-6 overflow-auto">
              {isLoading ? (
                <div className="flex items-center justify-center h-32">
                  <div className="text-gray-500">加载中...</div>
                </div>
              ) : error ? (
                <div className="flex flex-col items-center justify-center h-64 text-red-500">
                  <FolderOpen className="w-12 h-12 mb-4 text-red-300" />
                  <p>加载媒体内容失败</p>
                </div>
              ) : chapterMedias.length > 0 ? (
                <div>
                  <div className="mb-4">
                    <h3 className="text-lg font-medium text-gray-900">
                      媒体内容 ({chapterMedias.length})
                    </h3>
                  </div>
                  {chapterMedias.map((media) => (
                    <ChapterMediaItem
                      key={media.id}
                      media={media}
                      onPlay={handlePlayMedia}
                      onDownload={handleDownloadMedia}
                    />
                  ))}
                </div>
              ) : (
                <div className="flex flex-col items-center justify-center h-64 text-gray-500">
                  <FolderOpen className="w-12 h-12 mb-4 text-gray-300" />
                  <p>该章节暂无媒体内容</p>
                </div>
              )}
            </div>
          </>
        )}

        {!selectedChapter && (
          <div className="flex-1 flex items-center justify-center text-gray-500">
            <div className="text-center">
              <FolderOpen className="w-12 h-12 mx-auto mb-4 text-gray-300" />
              <p>请选择一个章节查看媒体内容</p>
            </div>
          </div>
        )}
      </div>

      {/* 媒体播放器对话框 */}
      {selectedMedia && (
        <MediaPlayer
          media={selectedMedia}
          isOpen={isPlayerOpen}
          onClose={() => {
            setIsPlayerOpen(false)
            setSelectedMedia(null)
          }}
        />
      )}
    </div>
  )
}