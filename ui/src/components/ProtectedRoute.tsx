import { Navigate } from 'react-router-dom';
import { useAutoAuth } from '../hooks/useAuth';
import Loading from './Loading';

interface ProtectedRouteProps {
  children: React.ReactNode;
}

export default function ProtectedRoute({ children }: ProtectedRouteProps) {
  const { data, isLoading } = useAutoAuth();

  if (isLoading) {
    return <Loading />;
  }

  if (!data?.isAuthenticated) {
    return <Navigate to="/login" replace />;
  }

  return <>{children}</>;
}