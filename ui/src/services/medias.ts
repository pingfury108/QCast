import { api } from '../lib/api'

export interface Media {
  id: number
  title: string
  description?: string
  file_type: string
  file_path: string
  file_size?: number
  duration?: number
  mime_type?: string
  access_token: string
  access_url?: string
  qr_code_path?: string
  qr_code_url?: string
  file_version: number
  original_filename?: string
  play_count: number
  is_public: boolean
  chapter_id?: number
  book_id: number
  user_id: number
  created_at: string
  updated_at: string
}

export interface CreateMediaParams {
  title: string
  description?: string
  book_id: number
  chapter_id?: number
}

export interface UpdateMediaParams {
  title?: string
  description?: string
  is_public?: boolean
  chapter_id?: number
}

export interface UploadMediaParams {
  title: string
  description?: string
  book_id: number
  chapter_id?: number
  file: File
}

export const mediasService = {
  // 获取用户的所有媒体
  async getMedias(): Promise<Media[]> {
    const response = await api.get('/media')
    return response.data
  },

  // 获取书籍的媒体列表
  async getBookMedias(bookId: number): Promise<Media[]> {
    const response = await api.get(`/media?book_id=${bookId}`)
    return response.data
  },

  // 获取章节的媒体列表（原有方式，兼容性保留）
  async getChapterMedias(chapterId: number): Promise<Media[]> {
    const response = await api.get(`/media?chapter_id=${chapterId}`)
    return response.data
  },

  // 获取章节的媒体列表（非递归，仅当前章节）
  async getChapterMediasOnly(chapterId: number): Promise<Media[]> {
    const response = await api.get(`/media/by-chapter?chapter_id=${chapterId}`)
    return response.data
  },

  // 获取章节的媒体列表（递归，包含所有子章节）
  async getChapterMediasRecursive(chapterId: number): Promise<Media[]> {
    const response = await api.get(`/media/by-chapter-recursive?chapter_id=${chapterId}`)
    return response.data
  },

  // 获取章节的直接子章节列表
  async getChildChapters(chapterId: number): Promise<any[]> {
    const response = await api.get(`/media/chapters/${chapterId}/children`)
    return response.data
  },

  // 搜索媒体
  async searchMedias(query: string): Promise<Media[]> {
    const response = await api.get('/media/search', {
      params: { q: query }
    })
    return response.data
  },

  // 获取媒体详情
  async getMedia(id: number): Promise<Media> {
    const response = await api.get(`/media/${id}`)
    return response.data
  },

  // 上传媒体文件
  async uploadMedia(params: UploadMediaParams, onProgress?: (progress: number) => void): Promise<Media> {
    const formData = new FormData()
    formData.append('file', params.file)
    formData.append('title', params.title)
    if (params.description) {
      formData.append('description', params.description)
    }
    formData.append('book_id', params.book_id.toString())
    if (params.chapter_id) {
      formData.append('chapter_id', params.chapter_id.toString())
    }

    // 注意：不要手动设置 Content-Type，让浏览器自动设置（包含 boundary）
    const response = await api.post('/media/upload', formData, {
      onUploadProgress: (progressEvent) => {
        if (progressEvent.total && onProgress) {
          const progress = Math.round((progressEvent.loaded * 100) / progressEvent.total)
          onProgress(progress)
        }
      }
    })
    return response.data
  },

  // 更新媒体信息
  async updateMedia(id: number, params: UpdateMediaParams): Promise<Media> {
    const response = await api.put(`/media/${id}`, params)
    return response.data
  },

  // 删除媒体
  async deleteMedia(id: number): Promise<void> {
    await api.delete(`/media/${id}`)
  },

  // 发布/取消发布媒体
  async toggleMediaPublish(id: number): Promise<Media> {
    const response = await api.post(`/media/${id}/publish`)
    return response.data
  },

  // 获取媒体二维码
  async getMediaQRCode(id: number): Promise<{ qrcode_path: string; qrcode_url: string }> {
    const response = await api.get(`/media/${id}/qrcode`)
    return response.data
  },

  // 重新生成媒体二维码
  async regenerateMediaQRCode(id: number): Promise<{ qrcode_path: string; qrcode_url: string }> {
    const response = await api.post(`/media/${id}/regenerate-qr`)
    return response.data
  },

  // 替换媒体文件
  async replaceMediaFile(id: number, file: File): Promise<Media> {
    const formData = new FormData()
    formData.append('file', file)

    // 注意：不要手动设置 Content-Type，让浏览器自动设置（包含 boundary）
    const response = await api.put(`/media/${id}/replace-file`, formData)
    return response.data
  }
}