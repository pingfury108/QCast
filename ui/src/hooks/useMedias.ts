import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { mediasService } from '../services/medias'
import type { Media, CreateMediaParams, UpdateMediaParams, UploadMediaParams } from '../services/medias'
import { toast } from 'sonner'

// 重新导出类型供其他组件使用
export type { Media, CreateMediaParams, UpdateMediaParams, UploadMediaParams }

export const useMedias = () => {
  return useQuery({
    queryKey: ['medias'],
    queryFn: () => mediasService.getMedias(),
    staleTime: 1000 * 60 * 5, // 5 minutes
  })
}

export const useBookMedias = (bookId: number) => {
  return useQuery({
    queryKey: ['medias', 'book', bookId],
    queryFn: () => mediasService.getBookMedias(bookId),
    enabled: !!bookId,
    staleTime: 1000 * 60 * 5, // 5 minutes
  })
}

export const useChapterMedias = (chapterId: number) => {
  return useQuery({
    queryKey: ['medias', 'chapter', chapterId],
    queryFn: () => mediasService.getChapterMedias(chapterId),
    enabled: !!chapterId,
    staleTime: 1000 * 60 * 5, // 5 minutes
  })
}

export const useMedia = (id: number) => {
  return useQuery({
    queryKey: ['medias', id],
    queryFn: () => mediasService.getMedia(id),
    enabled: !!id,
    staleTime: 1000 * 60 * 5, // 5 minutes
  })
}

export const useSearchMedias = (query: string) => {
  return useQuery({
    queryKey: ['medias', 'search', query],
    queryFn: () => mediasService.searchMedias(query),
    enabled: query.length > 0,
    staleTime: 1000 * 60 * 1, // 1 minute
  })
}

export const useUploadMedia = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (params: UploadMediaParams) => mediasService.uploadMedia(params),
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({ queryKey: ['medias'] })
      queryClient.invalidateQueries({ queryKey: ['medias', 'book', variables.book_id] })
      if (variables.chapter_id) {
        queryClient.invalidateQueries({ queryKey: ['medias', 'chapter', variables.chapter_id] })
      }
      toast.success('媒体上传成功')
    },
    onError: (error: any) => {
      toast.error(`上传失败: ${error.response?.data?.message || error.message}`)
    }
  })
}

export const useUpdateMedia = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: ({ id, params }: { id: number; params: UpdateMediaParams }) =>
      mediasService.updateMedia(id, params),
    onSuccess: (_, { id }) => {
      queryClient.invalidateQueries({ queryKey: ['medias'] })
      queryClient.invalidateQueries({ queryKey: ['medias', id] })
      toast.success('媒体更新成功')
    },
    onError: (error: any) => {
      toast.error(`更新失败: ${error.response?.data?.message || error.message}`)
    }
  })
}

export const useDeleteMedia = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (id: number) => mediasService.deleteMedia(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['medias'] })
      toast.success('媒体删除成功')
    },
    onError: (error: any) => {
      toast.error(`删除失败: ${error.response?.data?.message || error.message}`)
    }
  })
}

export const useToggleMediaPublish = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (id: number) => mediasService.toggleMediaPublish(id),
    onSuccess: (_, id) => {
      queryClient.invalidateQueries({ queryKey: ['medias'] })
      queryClient.invalidateQueries({ queryKey: ['medias', id] })
      toast.success('发布状态更新成功')
    },
    onError: (error: any) => {
      toast.error(`操作失败: ${error.response?.data?.message || error.message}`)
    }
  })
}

export const useGetMediaQRCode = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (id: number) => mediasService.getMediaQRCode(id),
    onSuccess: (_, id) => {
      queryClient.invalidateQueries({ queryKey: ['medias', id] })
    },
    onError: (error: any) => {
      toast.error(`获取二维码失败: ${error.response?.data?.message || error.message}`)
    }
  })
}

export const useRegenerateMediaQRCode = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (id: number) => mediasService.regenerateMediaQRCode(id),
    onSuccess: (_, id) => {
      queryClient.invalidateQueries({ queryKey: ['medias', id] })
      toast.success('二维码重新生成成功')
    },
    onError: (error: any) => {
      toast.error(`重新生成失败: ${error.response?.data?.message || error.message}`)
    }
  })
}

export const useReplaceMediaFile = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: ({ id, file }: { id: number; file: File }) =>
      mediasService.replaceMediaFile(id, file),
    onSuccess: (_, { id }) => {
      queryClient.invalidateQueries({ queryKey: ['medias'] })
      queryClient.invalidateQueries({ queryKey: ['medias', id] })
      toast.success('文件替换成功')
    },
    onError: (error: any) => {
      toast.error(`替换失败: ${error.response?.data?.message || error.message}`)
    }
  })
}