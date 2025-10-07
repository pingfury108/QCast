import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { booksService } from '../services/books'
import type { Book, CreateBookParams, UpdateBookParams, ReorderBookParams, BookTree } from '../services/books'

// 重新导出类型供其他组件使用
export type { Book, CreateBookParams, UpdateBookParams, ReorderBookParams, BookTree }
import { toast } from 'sonner'

export const useBooks = () => {
  return useQuery({
    queryKey: ['books'],
    queryFn: () => booksService.getBooks(),
    staleTime: 1000 * 60 * 5, // 5 minutes
  })
}

export const useSearchBooks = (query: string) => {
  return useQuery({
    queryKey: ['books', 'search', query],
    queryFn: () => booksService.searchBooks(query),
    enabled: query.length > 0,
    staleTime: 1000 * 60 * 1, // 1 minute
  })
}

export const useBook = (id: number) => {
  return useQuery({
    queryKey: ['books', id],
    queryFn: () => booksService.getBook(id),
    enabled: !!id,
    staleTime: 1000 * 60 * 5, // 5 minutes
  })
}

export const useBookTree = (id: number) => {
  return useQuery({
    queryKey: ['books', id, 'tree'],
    queryFn: () => booksService.getBookTree(id),
    enabled: !!id,
    staleTime: 1000 * 60 * 2, // 2 minutes
  })
}

export const useCreateBook = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (params: CreateBookParams) => booksService.createBook(params),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['books'] })
      toast.success('书籍创建成功')
    },
    onError: (error: any) => {
      toast.error(`创建失败: ${error.response?.data?.message || error.message}`)
    }
  })
}

export const useUpdateBook = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: ({ id, params }: { id: number; params: UpdateBookParams }) =>
      booksService.updateBook(id, params),
    onSuccess: (_, { id }) => {
      queryClient.invalidateQueries({ queryKey: ['books'] })
      queryClient.invalidateQueries({ queryKey: ['books', id] })
      toast.success('书籍更新成功')
    },
    onError: (error: any) => {
      toast.error(`更新失败: ${error.response?.data?.message || error.message}`)
    }
  })
}

export const useDeleteBook = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (id: number) => booksService.deleteBook(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['books'] })
      toast.success('书籍删除成功')
    },
    onError: (error: any) => {
      toast.error(`删除失败: ${error.response?.data?.message || error.message}`)
    }
  })
}

export const useReorderBook = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: ({ id, params }: { id: number; params: ReorderBookParams }) =>
      booksService.reorderBook(id, params),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['books'] })
      toast.success('排序调整成功')
    },
    onError: (error: any) => {
      toast.error(`排序调整失败: ${error.response?.data?.message || error.message}`)
    }
  })
}