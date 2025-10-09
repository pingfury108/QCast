import { api } from '../lib/api'

export interface DashboardStats {
  total_books: number
  total_medias: number
  total_chapters: number
  total_plays: number
}

export interface TopMedia {
  id: number
  title: string
  book_title: string
  chapter_title?: string
  play_count: number
  book_id: number
}

export interface RecentMedia {
  id: number
  title: string
  book_title: string
  chapter_title?: string
  created_at: string
  book_id: number
}

export const dashboardService = {
  // 获取统计数据
  async getStats(): Promise<DashboardStats> {
    const response = await api.get('/dashboard/stats')
    return response.data
  },

  // 获取播放排行
  async getTopMedias(): Promise<TopMedia[]> {
    const response = await api.get('/dashboard/top-medias')
    return response.data
  },

  // 获取最近上传
  async getRecentMedias(): Promise<RecentMedia[]> {
    const response = await api.get('/dashboard/recent-medias')
    return response.data
  }
}
