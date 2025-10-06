import { createBrowserRouter } from 'react-router-dom';
import Layout from '../components/Layout';
import ProtectedRoute from '../components/ProtectedRoute';
import HomePage from '../pages/HomePage';
import LoginPage from '../pages/LoginPage';
import RegisterPage from '../pages/RegisterPage';
import ForgotPasswordPage from '../pages/ForgotPasswordPage';
import ResetPasswordPage from '../pages/ResetPasswordPage';
import DashboardPage from '../pages/DashboardPage';
import BooksPage from '../pages/BooksPage';
import BookDetailPage from '../pages/BookDetailPage';
import UploadPage from '../pages/UploadPage';
import MediaPage from '../pages/MediaPage';
import PublicMediaPage from '../pages/PublicMediaPage';

// 创建路由配置
export const router = createBrowserRouter([
  {
    path: '/',
    element: <Layout />,
    children: [
      {
        index: true,
        element: <HomePage />,
      },
      {
        path: 'login',
        element: <LoginPage />,
      },
      {
        path: 'register',
        element: <RegisterPage />,
      },
      {
        path: 'forgot-password',
        element: <ForgotPasswordPage />,
      },
      {
        path: 'reset-password/:token',
        element: <ResetPasswordPage />,
      },
      {
        path: 'dashboard',
        element: (
          <ProtectedRoute>
            <DashboardPage />
          </ProtectedRoute>
        ),
      },
      {
        path: 'dashboard/books',
        element: (
          <ProtectedRoute>
            <BooksPage />
          </ProtectedRoute>
        ),
      },
      {
        path: 'dashboard/books/:id',
        element: (
          <ProtectedRoute>
            <BookDetailPage />
          </ProtectedRoute>
        ),
      },
      {
        path: 'dashboard/upload',
        element: (
          <ProtectedRoute>
            <UploadPage />
          </ProtectedRoute>
        ),
      },
      {
        path: 'dashboard/media',
        element: (
          <ProtectedRoute>
            <MediaPage />
          </ProtectedRoute>
        ),
      },
    ],
  },
  {
    path: '/public/:token',
    element: <PublicMediaPage />,
  },
]);