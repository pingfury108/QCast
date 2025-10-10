import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { chaptersService } from '../services/chapters'
import type { Chapter, ChapterTree, CreateChapterParams, CreateChildChapterParams, UpdateChapterParams, ReorderChapterParams, BatchReorderChapterParams, MoveChapterParams } from '../services/chapters'
import { toast } from 'sonner'

// 重新导出类型供其他组件使用
export type { Chapter, ChapterTree, CreateChapterParams, CreateChildChapterParams, UpdateChapterParams, ReorderChapterParams, BatchReorderChapterParams, MoveChapterParams }

export const useChapters = (bookId: number) => {
  return useQuery({
    queryKey: ['chapters', bookId],
    queryFn: () => chaptersService.getChapters(bookId),
    enabled: !!bookId,
    staleTime: 1000 * 60 * 5, // 5 minutes
  })
}

export const useSearchChapters = (bookId: number, query: string) => {
  return useQuery({
    queryKey: ['chapters', 'search', bookId, query],
    queryFn: () => chaptersService.searchChapters(bookId, query),
    enabled: !!bookId && query.length > 0,
    staleTime: 1000 * 60 * 1, // 1 minute
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
      queryClient.invalidateQueries({ queryKey: ['chapters', 'tree', bookId] })
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
      queryClient.invalidateQueries({ queryKey: ['chapters', 'tree', bookId] })
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
      queryClient.invalidateQueries({ queryKey: ['chapters', 'tree', bookId] })
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
    onSuccess: async (_, { bookId }) => {
      // 使用 refetch 而不是 invalidate，确保立即重新获取数据
      await queryClient.refetchQueries({ queryKey: ['chapters', bookId] })
      await queryClient.refetchQueries({ queryKey: ['chapters', 'tree', bookId] })
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
      queryClient.invalidateQueries({ queryKey: ['chapters', 'tree', bookId] })
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
      queryClient.invalidateQueries({ queryKey: ['chapters', 'tree', bookId] })
      toast.success('章节下移成功')
    },
    onError: (error: any) => {
      toast.error(`操作失败: ${error.response?.data?.message || error.message}`)
    }
  })
}

// 树状相关hooks
export const useChapterTree = (bookId: number) => {
  return useQuery({
    queryKey: ['chapters', 'tree', bookId],
    queryFn: () => chaptersService.getChapterTree(bookId),
    enabled: !!bookId,
    staleTime: 1000 * 60 * 5, // 5 minutes
  })
}

export const useChapterChildren = (bookId: number, id: number) => {
  return useQuery({
    queryKey: ['chapters', 'children', bookId, id],
    queryFn: () => chaptersService.getChapterChildren(bookId, id),
    enabled: !!bookId && !!id,
    staleTime: 1000 * 60 * 5, // 5 minutes
  })
}

export const useCreateChildChapter = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: ({ bookId, parentId, params }: { bookId: number; parentId: number; params: CreateChildChapterParams }) =>
      chaptersService.createChildChapter(bookId, parentId, params),
    onSuccess: async (_, { bookId }) => {
      await queryClient.refetchQueries({ queryKey: ['chapters', bookId] })
      await queryClient.refetchQueries({ queryKey: ['chapters', 'tree', bookId] })
      toast.success('子章节创建成功')
    },
    onError: (error: any) => {
      toast.error(`创建失败: ${error.response?.data?.message || error.message}`)
    }
  })
}

export const useMoveChapter = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: ({ bookId, id, params }: { bookId: number; id: number; params: MoveChapterParams }) =>
      chaptersService.moveChapter(bookId, id, params),
    onSuccess: async (_, { bookId }) => {
      await queryClient.refetchQueries({ queryKey: ['chapters', bookId] })
      await queryClient.refetchQueries({ queryKey: ['chapters', 'tree', bookId] })
      toast.success('章节移动成功')
    },
    onError: (error: any) => {
      toast.error(`移动失败: ${error.response?.data?.message || error.message}`)
    }
  })
}