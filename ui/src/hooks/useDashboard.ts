import { useQuery } from '@tanstack/react-query'
import { dashboardService } from '../services/dashboard'

export function useDashboardStats() {
  return useQuery({
    queryKey: ['dashboard', 'stats'],
    queryFn: () => dashboardService.getStats(),
    staleTime: 1000 * 60 * 5, // 5 minutes
  })
}

export function useTopMedias() {
  return useQuery({
    queryKey: ['dashboard', 'top-medias'],
    queryFn: () => dashboardService.getTopMedias(),
    staleTime: 1000 * 60 * 5, // 5 minutes
  })
}

export function useRecentMedias() {
  return useQuery({
    queryKey: ['dashboard', 'recent-medias'],
    queryFn: () => dashboardService.getRecentMedias(),
    staleTime: 1000 * 60 * 2, // 2 minutes
  })
}
