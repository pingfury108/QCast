import { api } from '../lib/api'

export interface Chapter {
  id: number
  title: string
  description?: string
  book_id: number
  sort_order?: number
  media_count: number
  created_at: string
  updated_at: string
}

export interface CreateChapterParams {
  title: string
  description?: string
  sort_order?: number
}

export interface UpdateChapterParams {
  title?: string
  description?: string
  sort_order?: number
}

export interface ReorderChapterParams {
  sort_order: number
}

export interface BatchReorderChapterParams {
  chapter_ids: number[]
}

export const chaptersService = {
  // 获取书籍的章节列表
  async getChapters(bookId: number): Promise<Chapter[]> {
    const response = await api.get(`/books/${bookId}/chapters`)
    return response.data
  },

  // 获取章节详情
  async getChapter(bookId: number, id: number): Promise<Chapter> {
    const response = await api.get(`/books/${bookId}/chapters/${id}`)
    return response.data
  },

  // 创建章节
  async createChapter(bookId: number, params: CreateChapterParams): Promise<Chapter> {
    const response = await api.post(`/books/${bookId}/chapters`, params)
    return response.data
  },

  // 更新章节
  async updateChapter(bookId: number, id: number, params: UpdateChapterParams): Promise<Chapter> {
    const response = await api.put(`/books/${bookId}/chapters/${id}`, params)
    return response.data
  },

  // 删除章节
  async deleteChapter(bookId: number, id: number): Promise<void> {
    await api.delete(`/books/${bookId}/chapters/${id}`)
  },

  // 调整章节顺序
  async reorderChapter(bookId: number, id: number, params: ReorderChapterParams): Promise<Chapter> {
    const response = await api.post(`/books/${bookId}/chapters/${id}/reorder`, params)
    return response.data
  },

  // 批量调整章节顺序
  async batchReorderChapters(bookId: number, params: BatchReorderChapterParams): Promise<void> {
    await api.post(`/books/${bookId}/chapters/batch-reorder`, params)
  },

  // 章节上移
  async moveChapterUp(bookId: number, id: number): Promise<Chapter> {
    const response = await api.post(`/books/${bookId}/chapters/${id}/move-up`)
    return response.data
  },

  // 章节下移
  async moveChapterDown(bookId: number, id: number): Promise<Chapter> {
    const response = await api.post(`/books/${bookId}/chapters/${id}/move-down`)
    return response.data
  }
}