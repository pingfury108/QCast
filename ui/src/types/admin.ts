export interface User {
  id: number;
  pid: string;
  email: string;
  name: string;
  is_staff: boolean;
  is_superuser: boolean;
  email_verified_at: string | null;
  created_at: string;
  updated_at: string;
}

export interface UserListResponse {
  users: User[];
  pagination: {
    page: number;
    per_page: number;
    total_pages: number;
  };
}

export interface UserStatsResponse {
  total_users: number;
  total_admins: number;
}

export interface Group {
  id: number;
  name: string;
  description: string | null;
  created_at: string;
  updated_at: string;
}

export interface GroupWithMembers extends Group {
  member_count: number;
}

export interface GroupDetailResponse {
  group: Group;
  members: User[];
  member_count: number;
}

export interface LoginRequest {
  email: string;
  password: string;
}

export interface LoginResponse {
  token: string;
  user: User;
}