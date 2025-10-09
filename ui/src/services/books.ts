import { api } from '../lib/api'

export interface Book {
  id: number
  title: string
  description?: string
  cover_image?: string
  parent_id?: number
  sort_order?: number
  is_public?: boolean
  user_id: number
  created_at: string
  updated_at: string
  media_count?: number
  chapter_count?: number
}

export interface BookTree {
  book: Book
  children: BookTree[]
}

export interface CreateBookParams {
  title: string
  description?: string
  cover_image?: string
  parent_id?: number
  sort_order?: number
  is_public?: boolean
}

export interface UpdateBookParams {
  title?: string
  description?: string
  cover_image?: string
  parent_id?: number
  sort_order?: number
  is_public?: boolean
}

export interface ReorderBookParams {
  sort_order: number
}

export const booksService = {
  // 获取书籍列表
  async getBooks(): Promise<Book[]> {
    const response = await api.get('/books')
    return response.data
  },

  // 搜索书籍
  async searchBooks(query: string): Promise<Book[]> {
    const response = await api.get('/books/search', {
      params: { q: query }
    })
    return response.data
  },

  // 获取书籍详情
  async getBook(id: number): Promise<Book> {
    const response = await api.get(`/books/${id}`)
    return response.data
  },

  // 创建书籍
  async createBook(params: CreateBookParams): Promise<Book> {
    const response = await api.post('/books', params)
    return response.data
  },

  // 更新书籍
  async updateBook(id: number, params: UpdateBookParams): Promise<Book> {
    const response = await api.put(`/books/${id}`, params)
    return response.data
  },

  // 删除书籍
  async deleteBook(id: number): Promise<void> {
    await api.delete(`/books/${id}`)
  },

  // 获取书籍树形结构
  async getBookTree(id: number): Promise<BookTree> {
    const response = await api.get(`/books/${id}/tree`)
    return response.data
  },

  // 调整书籍顺序
  async reorderBook(id: number, params: ReorderBookParams): Promise<Book> {
    const response = await api.post(`/books/${id}/reorder`, params)
    return response.data
  }
}