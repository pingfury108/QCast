import { LucideIcon } from 'lucide-react';

interface EmptyStateProps {
  icon?: LucideIcon;
  title: string;
  description?: string;
  action?: React.ReactNode;
}

export default function EmptyState({
  icon: Icon,
  title,
  description,
  action,
}: EmptyStateProps) {
  return (
    <div className="flex flex-col items-center justify-center text-center p-8">
      {Icon && (
        <div className="flex items-center justify-center w-16 h-16 rounded-full bg-muted mb-4">
          <Icon className="h-8 w-8 text-muted-foreground" />
        </div>
      )}
      <h3 className="text-lg font-semibold mb-2">{title}</h3>
      {description && (
        <p className="text-sm text-muted-foreground mb-4 max-w-sm">
          {description}
        </p>
      )}
      {action && <div>{action}</div>}
    </div>
  );
}

// 预定义的常用空状态
export function EmptyBooks() {
  return (
    <EmptyState
      title="还没有书籍"
      description="创建你的第一个书籍来开始管理音视频内容"
    />
  );
}

export function EmptyMedia() {
  return (
    <EmptyState
      title="还没有媒体文件"
      description="上传你的第一个音视频文件来开始分享"
    />
  );
}

export function EmptySearch() {
  return (
    <EmptyState
      title="没有找到结果"
      description="尝试使用不同的关键词或筛选条件"
    />
  );
}