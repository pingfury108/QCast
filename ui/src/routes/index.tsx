import { createBrowserRouter } from 'react-router-dom';
import Layout from '../components/Layout';
import { DashboardLayout } from '../components/DashboardLayout';
import ProtectedRoute from '../components/ProtectedRoute';
import HomePage from '../pages/HomePage';
import LoginPage from '../pages/LoginPage';
import RegisterPage from '../pages/RegisterPage';
import ForgotPasswordPage from '../pages/ForgotPasswordPage';
import ResetPasswordPage from '../pages/ResetPasswordPage';
import DashboardPage from '../pages/DashboardPage';
import DashboardBooks from '../pages/DashboardBooks';
import BookDetailPage from '../pages/BookDetailPage';
import PublicMediaPage from '../pages/PublicMediaPage';
import ProfilePage from '../pages/ProfilePage';
import SettingsPage from '../pages/SettingsPage';
import { ProtectedRoute as AdminProtectedRoute } from '../components/admin/ProtectedRoute';
import { AdminDashboardPage } from '../pages/admin/AdminDashboardPage';
import { AdminUsersPage } from '../pages/admin/AdminUsersPage';
import { AdminGroupsPage } from '../pages/admin/AdminGroupsPage';
import { AdminSettingsPage } from '../pages/admin/AdminSettingsPage';

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
            <DashboardLayout>
              <DashboardBooks />
            </DashboardLayout>
          </ProtectedRoute>
        ),
      },
      {
        path: 'dashboard/books/:id',
        element: (
          <ProtectedRoute>
            <DashboardLayout>
              <BookDetailPage />
            </DashboardLayout>
          </ProtectedRoute>
        ),
      },
      {
        path: 'profile',
        element: (
          <ProtectedRoute>
            <DashboardLayout>
              <ProfilePage />
            </DashboardLayout>
          </ProtectedRoute>
        ),
      },
      {
        path: 'settings',
        element: (
          <ProtectedRoute>
            <DashboardLayout>
              <SettingsPage />
            </DashboardLayout>
          </ProtectedRoute>
        ),
      },
      {
        path: 'admin',
        element: (
          <AdminProtectedRoute>
            <AdminDashboardPage />
          </AdminProtectedRoute>
        ),
      },
      {
        path: 'admin/users',
        element: (
          <AdminProtectedRoute>
            <AdminUsersPage />
          </AdminProtectedRoute>
        ),
      },
      {
        path: 'admin/groups',
        element: (
          <AdminProtectedRoute>
            <AdminGroupsPage />
          </AdminProtectedRoute>
        ),
      },
      {
        path: 'admin/settings',
        element: (
          <AdminProtectedRoute>
            <AdminSettingsPage />
          </AdminProtectedRoute>
        ),
      },
    ],
  },
  {
    path: '/public/:token',
    element: <PublicMediaPage />,
  },
]);