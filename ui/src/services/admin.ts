import axios from 'axios';
import type {
  User,
  UserListResponse,
  UserStatsResponse,
  Group,
  GroupWithMembers,
  GroupDetailResponse,
  LoginRequest,
  LoginResponse,
} from '../types/admin';

// 管理后台 API 客户端 - 使用 Vite 代理
const adminApi = axios.create({
  baseURL: '/api/admin',
  headers: {
    'Content-Type': 'application/json',
  },
});

// 请求拦截器：添加 token
adminApi.interceptors.request.use((config) => {
  const token = localStorage.getItem('admin_token');
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

// 响应拦截器：处理错误
adminApi.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response?.status === 401) {
      // 未授权，清除 token 并跳转到登录页
      localStorage.removeItem('admin_token');
      window.location.href = '/admin/login';
    }
    return Promise.reject(error);
  }
);

// ========== 用户管理 API ==========

export async function listUsers(params?: { page?: number; per_page?: number; search?: string }): Promise<UserListResponse> {
  const response = await adminApi.get<UserListResponse>('/users', { params });
  return response.data;
}

export async function getUserStats(): Promise<UserStatsResponse> {
  const response = await adminApi.get<UserStatsResponse>('/users/stats');
  return response.data;
}

export async function updateUserRole(userId: number, data: { is_staff: boolean; is_superuser: boolean }): Promise<User> {
  const response = await adminApi.put<User>(`/users/${userId}/role`, data);
  return response.data;
}

export async function deleteUser(userId: number): Promise<void> {
  await adminApi.delete(`/users/${userId}`);
}

// ========== 用户组管理 API ==========

export async function listGroups(): Promise<GroupWithMembers[]> {
  const response = await adminApi.get<GroupWithMembers[]>('/groups');
  return response.data;
}

export async function createGroup(data: { name: string; description?: string }): Promise<Group> {
  const response = await adminApi.post<Group>('/groups', data);
  return response.data;
}

export async function getGroup(groupId: number): Promise<GroupDetailResponse> {
  const response = await adminApi.get<GroupDetailResponse>(`/groups/${groupId}`);
  return response.data;
}

export async function addGroupMember(groupId: number, userId: number): Promise<void> {
  await adminApi.post(`/groups/${groupId}/members`, { user_id: userId });
}

export async function removeGroupMember(groupId: number, userId: number): Promise<void> {
  await adminApi.delete(`/groups/${groupId}/members/${userId}`);
}

export async function deleteGroup(groupId: number): Promise<void> {
  await adminApi.delete(`/groups/${groupId}`);
}

// 导出对象形式的 API
export const adminUserApi = {
  list: listUsers,
  stats: getUserStats,
  updateRole: updateUserRole,
  delete: deleteUser,
};

export const adminGroupApi = {
  list: listGroups,
  create: createGroup,
  get: getGroup,
  addMember: addGroupMember,
  removeMember: removeGroupMember,
  delete: deleteGroup,
};

// 重新导出类型（仅管理相关的）
export type {
  UserListResponse,
  UserStatsResponse,
  Group,
  GroupWithMembers,
  GroupDetailResponse,
};

export default adminApi;
