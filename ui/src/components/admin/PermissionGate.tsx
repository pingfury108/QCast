import type { ReactNode } from 'react';
import { useAuthState } from '@/hooks/useAuthState';

interface PermissionGateProps {
  children: ReactNode;
  requireSuperAdmin?: boolean;
  requireAdmin?: boolean;
  fallback?: ReactNode;
}

export function PermissionGate({
  children,
  requireSuperAdmin = false,
  requireAdmin = false,
  fallback = null,
}: PermissionGateProps) {
  const { user } = useAuthState();

  if (!user) {
    return <>{fallback}</>;
  }

  if (requireSuperAdmin && !user.is_superuser) {
    return <>{fallback}</>;
  }

  if (requireAdmin && !user.is_staff && !user.is_superuser) {
    return <>{fallback}</>;
  }

  return <>{children}</>;
}
