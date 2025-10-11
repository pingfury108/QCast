import { useParams } from 'react-router-dom';
import { useQuery } from '@tanstack/react-query';
import { Clock, Eye, AlertCircle } from 'lucide-react';
import Footer from '../components/Footer';
import { Card, CardContent } from '@/components/ui/card';
import axios from 'axios';

interface PublicMediaInfo {
  id: number;
  title: string;
  description: string | null;
  file_type: string;
  duration: number | null;
  mime_type: string | null;
  original_filename: string | null;
  play_count: number;
  created_at: string;
}

export default function PublicMediaPage() {
  const { token } = useParams<{ token: string }>();

  // 获取媒体信息
  const { data: mediaInfo, isLoading, error } = useQuery({
    queryKey: ['public-media', token],
    queryFn: async () => {
      const response = await axios.get<PublicMediaInfo>(
        `/api/public/media/${token}/info`
      );
      return response.data;
    },
    enabled: !!token,
    retry: false,
  });

  // 构建媒体文件 URL
  const mediaUrl = token ? `/api/public/media/${token}` : '';

  // 格式化时长
  const formatDuration = (seconds: number | null) => {
    if (!seconds) return '--:--';
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins}:${secs.toString().padStart(2, '0')}`;
  };

  // 错误状态
  if (error) {
    return (
      <div className="min-h-screen flex flex-col bg-background">
        <div className="flex-1 container mx-auto px-4 py-8 max-w-2xl flex items-center justify-center">
          <Card className="w-full">
            <CardContent className="p-8 text-center">
              <AlertCircle className="h-12 w-12 text-destructive mx-auto mb-4" />
              <h2 className="text-xl font-semibold mb-2">访问失败</h2>
              <p className="text-muted-foreground mb-4">
                {axios.isAxiosError(error) && error.response?.status === 401
                  ? '此媒体未公开或访问链接已失效'
                  : '媒体不存在或访问令牌无效'}
              </p>
            </CardContent>
          </Card>
        </div>
        <Footer />
      </div>
    );
  }

  // 加载状态
  if (isLoading) {
    return (
      <div className="min-h-screen flex flex-col bg-background">
        <div className="flex-1 container mx-auto px-4 py-8 max-w-2xl flex items-center justify-center">
          <Card className="w-full">
            <CardContent className="p-8 text-center">
              <div className="animate-pulse">
                <div className="h-4 bg-muted rounded w-3/4 mx-auto mb-4"></div>
                <div className="h-4 bg-muted rounded w-1/2 mx-auto"></div>
              </div>
              <p className="text-muted-foreground mt-4">加载中...</p>
            </CardContent>
          </Card>
        </div>
        <Footer />
      </div>
    );
  }

  if (!mediaInfo) {
    return null;
  }

  const isVideo = mediaInfo.file_type === 'video';
  const isAudio = mediaInfo.file_type === 'audio';

  // 调试信息
  console.log('PublicMediaPage 调试信息:', {
    token,
    mediaUrl,
    file_type: mediaInfo.file_type,
    mime_type: mediaInfo.mime_type,
    isVideo,
    isAudio,
  });

  return (
    <div className="min-h-screen flex flex-col bg-background">
      <div className="flex-1 container mx-auto px-4 py-8 max-w-4xl">
        <Card>
          <CardContent className="p-6 space-y-6">
            {/* 媒体标题 */}
            <div className="space-y-2">
              <h1 className="text-2xl font-bold">{mediaInfo.title}</h1>
              {mediaInfo.description && (
                <p className="text-muted-foreground">{mediaInfo.description}</p>
              )}
            </div>

            {/* 媒体播放器 */}
            <div>
              {isVideo && (
                <video
                  key={mediaUrl}
                  controls
                  className="w-full rounded-lg"
                  preload="metadata"
                  controlsList="nodownload"
                  src={mediaUrl}
                  onError={(e) => {
                    console.error('视频加载错误:', e);
                    console.error('视频元素:', e.currentTarget);
                    console.error('视频 src:', e.currentTarget.src);
                    console.error('视频 error:', e.currentTarget.error);
                  }}
                  onLoadedMetadata={() => console.log('视频元数据已加载')}
                  onLoadStart={() => console.log('视频开始加载，URL:', mediaUrl)}
                />
              )}
              {isAudio && (
                <audio
                  key={mediaUrl}
                  controls
                  className="w-full"
                  preload="metadata"
                  controlsList="nodownload"
                  src={mediaUrl}
                  onError={(e) => {
                    console.error('音频加载错误:', e);
                    console.error('音频元素:', e.currentTarget);
                    console.error('音频 src:', e.currentTarget.src);
                    console.error('音频 error:', e.currentTarget.error);
                  }}
                  onLoadedMetadata={() => console.log('音频元数据已加载')}
                  onLoadStart={() => console.log('音频开始加载，URL:', mediaUrl)}
                />
              )}
              {!isVideo && !isAudio && (
                <div className="p-8 text-center text-muted-foreground border rounded-lg">
                  <p>不支持的媒体类型</p>
                </div>
              )}
            </div>

            {/* 媒体信息 */}
            <div className="flex flex-wrap gap-4 text-sm text-muted-foreground">
              {mediaInfo.duration && (
                <div className="flex items-center gap-2">
                  <Clock className="h-4 w-4" />
                  <span>{formatDuration(mediaInfo.duration)}</span>
                </div>
              )}
              <div className="flex items-center gap-2">
                <Eye className="h-4 w-4" />
                <span>{mediaInfo.play_count} 次播放</span>
              </div>
            </div>
          </CardContent>
        </Card>
      </div>
      <Footer />
    </div>
  );
}