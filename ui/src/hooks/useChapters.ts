import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { chaptersService } from '../services/chapters'
import type { Chapter, CreateChapterParams, UpdateChapterParams, ReorderChapterParams, BatchReorderChapterParams } from '../services/chapters'
import { toast } from 'sonner'

// 重新导出类型供其他组件使用
export type { Chapter, CreateChapterParams, UpdateChapterParams, ReorderChapterParams, BatchReorderChapterParams }

export const useChapters = (bookId: number) => {
  return useQuery({
    queryKey: ['chapters', bookId],
    queryFn: () => chaptersService.getChapters(bookId),
    enabled: !!bookId,
    staleTime: 1000 * 60 * 5, // 5 minutes
  })
}

export const useChapter = (bookId: number, id: number) => {
  return useQuery({
    queryKey: ['chapters', bookId, id],
    queryFn: () => chaptersService.getChapter(bookId, id),
    enabled: !!bookId && !!id,
    staleTime: 1000 * 60 * 5, // 5 minutes
  })
}

export const useCreateChapter = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: ({ bookId, params }: { bookId: number; params: CreateChapterParams }) =>
      chaptersService.createChapter(bookId, params),
    onSuccess: (_, { bookId }) => {
      queryClient.invalidateQueries({ queryKey: ['chapters', bookId] })
      toast.success('章节创建成功')
    },
    onError: (error: any) => {
      toast.error(`创建失败: ${error.response?.data?.message || error.message}`)
    }
  })
}

export const useUpdateChapter = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: ({ bookId, id, params }: { bookId: number; id: number; params: UpdateChapterParams }) =>
      chaptersService.updateChapter(bookId, id, params),
    onSuccess: (_, { bookId, id }) => {
      queryClient.invalidateQueries({ queryKey: ['chapters', bookId] })
      queryClient.invalidateQueries({ queryKey: ['chapters', bookId, id] })
      toast.success('章节更新成功')
    },
    onError: (error: any) => {
      toast.error(`更新失败: ${error.response?.data?.message || error.message}`)
    }
  })
}

export const useDeleteChapter = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: ({ bookId, id }: { bookId: number; id: number }) =>
      chaptersService.deleteChapter(bookId, id),
    onSuccess: (_, { bookId }) => {
      queryClient.invalidateQueries({ queryKey: ['chapters', bookId] })
      toast.success('章节删除成功')
    },
    onError: (error: any) => {
      toast.error(`删除失败: ${error.response?.data?.message || error.message}`)
    }
  })
}

export const useReorderChapter = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: ({ bookId, id, params }: { bookId: number; id: number; params: ReorderChapterParams }) =>
      chaptersService.reorderChapter(bookId, id, params),
    onSuccess: (_, { bookId }) => {
      queryClient.invalidateQueries({ queryKey: ['chapters', bookId] })
      toast.success('章节顺序调整成功')
    },
    onError: (error: any) => {
      toast.error(`调整失败: ${error.response?.data?.message || error.message}`)
    }
  })
}

export const useBatchReorderChapters = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: ({ bookId, params }: { bookId: number; params: BatchReorderChapterParams }) =>
      chaptersService.batchReorderChapters(bookId, params),
    onSuccess: (_, { bookId }) => {
      queryClient.invalidateQueries({ queryKey: ['chapters', bookId] })
      toast.success('章节批量排序成功')
    },
    onError: (error: any) => {
      toast.error(`排序失败: ${error.response?.data?.message || error.message}`)
    }
  })
}

export const useMoveChapterUp = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: ({ bookId, id }: { bookId: number; id: number }) =>
      chaptersService.moveChapterUp(bookId, id),
    onSuccess: (_, { bookId }) => {
      queryClient.invalidateQueries({ queryKey: ['chapters', bookId] })
      toast.success('章节上移成功')
    },
    onError: (error: any) => {
      toast.error(`操作失败: ${error.response?.data?.message || error.message}`)
    }
  })
}

export const useMoveChapterDown = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: ({ bookId, id }: { bookId: number; id: number }) =>
      chaptersService.moveChapterDown(bookId, id),
    onSuccess: (_, { bookId }) => {
      queryClient.invalidateQueries({ queryKey: ['chapters', bookId] })
      toast.success('章节下移成功')
    },
    onError: (error: any) => {
      toast.error(`操作失败: ${error.response?.data?.message || error.message}`)
    }
  })
}