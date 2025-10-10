import { api } from '../lib/api'

export interface Chapter {
  id: number
  title: string
  description?: string
  book_id: number
  parent_id?: number | null
  level?: number | null
  path?: string | null
  sort_order?: number
  media_count: number
  created_at: string
  updated_at: string
}

export interface ChapterTree {
  id: number
  title: string
  description?: string
  book_id: number
  parent_id?: number | null
  level?: number | null
  path?: string | null
  sort_order?: number
  media_count: number
  created_at: string
  updated_at: string
  children: ChapterTree[]
}

export interface CreateChapterParams {
  title: string
  description?: string
  parent_id?: number
  sort_order?: number
}

export interface CreateChildChapterParams {
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

export interface MoveChapterParams {
  new_parent_id?: number
  new_sort_order?: number
}

export const chaptersService = {
  // 获取书籍的章节列表
  async getChapters(bookId: number): Promise<Chapter[]> {
    const response = await api.get(`/books/${bookId}/chapters`)
    return response.data
  },

  // 搜索章节
  async searchChapters(bookId: number, query: string): Promise<Chapter[]> {
    const response = await api.get(`/books/${bookId}/chapters/search`, {
      params: { q: query }
    })
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
  },

  // 获取书籍的章节树
  async getChapterTree(bookId: number): Promise<ChapterTree[]> {
    const response = await api.get(`/books/${bookId}/chapters/tree`)
    return response.data
  },

  // 获取章节的子章节
  async getChapterChildren(bookId: number, id: number): Promise<Chapter[]> {
    const response = await api.get(`/books/${bookId}/chapters/${id}/children`)
    return response.data
  },

  // 创建子章节
  async createChildChapter(bookId: number, parentId: number, params: CreateChildChapterParams): Promise<Chapter> {
    const response = await api.post(`/books/${bookId}/chapters/${parentId}/children`, params)
    return response.data
  },

  // 移动章节到新的父级
  async moveChapter(bookId: number, id: number, params: MoveChapterParams): Promise<Chapter> {
    const response = await api.post(`/books/${bookId}/chapters/${id}/move`, params)
    return response.data
  }
}