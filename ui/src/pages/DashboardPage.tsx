import { Link } from 'react-router-dom'
import { useDashboardStats, useTopMedias, useRecentMedias } from '../hooks/useDashboard'
import { BookOpen, Music, FileText, TrendingUp, Clock } from 'lucide-react'
import { formatDistanceToNow } from 'date-fns'
import { zhCN } from 'date-fns/locale'

export default function DashboardPage() {
  const { data: stats, isLoading: statsLoading, error: statsError } = useDashboardStats()
  const { data: topMediasData, isLoading: topLoading } = useTopMedias()
  const { data: recentMediasData, isLoading: recentLoading } = useRecentMedias()

  // 确保数据是数组
  const topMedias = Array.isArray(topMediasData) ? topMediasData : []
  const recentMedias = Array.isArray(recentMediasData) ? recentMediasData : []

  if (statsLoading) {
    return (
      <div className="container mx-auto px-4 py-8">
        <div className="flex justify-center items-center h-64">加载中...</div>
      </div>
    )
  }

  if (statsError) {
    return (
      <div className="container mx-auto px-4 py-8">
        <div className="text-center text-destructive">
          加载失败: {statsError instanceof Error ? statsError.message : '未知错误'}
        </div>
      </div>
    )
  }

  return (
    <div className="container mx-auto px-4 py-8 space-y-8">
      {/* 数据概览 */}
      <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
        <div className="bg-card border rounded-lg p-6">
          <div className="flex items-center justify-between mb-2">
            <BookOpen className="w-8 h-8 text-blue-500" />
          </div>
          <div className="text-3xl font-bold mb-1">{stats?.total_books || 0}</div>
          <div className="text-sm text-muted-foreground">书籍</div>
        </div>

        <div className="bg-card border rounded-lg p-6">
          <div className="flex items-center justify-between mb-2">
            <Music className="w-8 h-8 text-purple-500" />
          </div>
          <div className="text-3xl font-bold mb-1">{stats?.total_medias || 0}</div>
          <div className="text-sm text-muted-foreground">媒体</div>
        </div>

        <div className="bg-card border rounded-lg p-6">
          <div className="flex items-center justify-between mb-2">
            <FileText className="w-8 h-8 text-green-500" />
          </div>
          <div className="text-3xl font-bold mb-1">{stats?.total_chapters || 0}</div>
          <div className="text-sm text-muted-foreground">章节</div>
        </div>

        <div className="bg-card border rounded-lg p-6">
          <div className="flex items-center justify-between mb-2">
            <TrendingUp className="w-8 h-8 text-orange-500" />
          </div>
          <div className="text-3xl font-bold mb-1">
            {stats?.total_plays ? stats.total_plays.toLocaleString() : 0}
          </div>
          <div className="text-sm text-muted-foreground">播放</div>
        </div>
      </div>

      {/* 播放排行 */}
      <div className="bg-card border rounded-lg p-6">
        <div className="flex items-center gap-2 mb-4">
          <TrendingUp className="w-5 h-5 text-orange-500" />
          <h2 className="text-xl font-semibold">播放排行 TOP 10</h2>
        </div>

        {topLoading ? (
          <div className="text-center py-8 text-muted-foreground">加载中...</div>
        ) : topMedias.length === 0 ? (
          <div className="text-center py-8 text-muted-foreground">暂无数据</div>
        ) : (
          <div className="space-y-2">
            {topMedias.map((media, index) => (
              <Link
                key={media.id}
                to={`/dashboard/books/${media.book_id}`}
                className="flex items-center justify-between p-3 rounded-lg hover:bg-accent transition-colors"
              >
                <div className="flex items-center gap-3 flex-1 min-w-0">
                  <div className={`flex-shrink-0 w-8 h-8 rounded-full flex items-center justify-center font-bold text-sm ${
                    index === 0 ? 'bg-yellow-500 text-white' :
                    index === 1 ? 'bg-gray-400 text-white' :
                    index === 2 ? 'bg-orange-600 text-white' :
                    'bg-muted text-muted-foreground'
                  }`}>
                    {index + 1}
                  </div>
                  <div className="flex-1 min-w-0">
                    <div className="font-medium truncate">{media.title}</div>
                    <div className="text-sm text-muted-foreground truncate">
                      {media.book_title}
                      {media.chapter_title && ` - ${media.chapter_title}`}
                    </div>
                  </div>
                </div>
                <div className="flex items-center gap-1 text-muted-foreground flex-shrink-0">
                  <Music className="w-4 h-4" />
                  <span className="font-medium">{media.play_count.toLocaleString()}</span>
                </div>
              </Link>
            ))}
          </div>
        )}
      </div>

      {/* 最近上传 */}
      <div className="bg-card border rounded-lg p-6">
        <div className="flex items-center gap-2 mb-4">
          <Clock className="w-5 h-5 text-blue-500" />
          <h2 className="text-xl font-semibold">最近上传</h2>
        </div>

        {recentLoading ? (
          <div className="text-center py-8 text-muted-foreground">加载中...</div>
        ) : recentMedias.length === 0 ? (
          <div className="text-center py-8 text-muted-foreground">暂无数据</div>
        ) : (
          <div className="space-y-2">
            {recentMedias.map((media) => (
              <Link
                key={media.id}
                to={`/dashboard/books/${media.book_id}`}
                className="flex items-center justify-between p-3 rounded-lg hover:bg-accent transition-colors"
              >
                <div className="flex items-center gap-3 flex-1 min-w-0">
                  <Music className="w-5 h-5 text-purple-500 flex-shrink-0" />
                  <div className="flex-1 min-w-0">
                    <div className="font-medium truncate">{media.title}</div>
                    <div className="text-sm text-muted-foreground truncate">
                      {media.book_title}
                      {media.chapter_title && ` - ${media.chapter_title}`}
                    </div>
                  </div>
                </div>
                <div className="text-sm text-muted-foreground flex-shrink-0">
                  {formatDistanceToNow(new Date(media.created_at), {
                    addSuffix: true,
                    locale: zhCN
                  })}
                </div>
              </Link>
            ))}
          </div>
        )}
      </div>
    </div>
  )
}
