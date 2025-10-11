export interface User {
  id: number;
  pid: string;
  name: string;
  email: string;
  is_staff: boolean;
  is_superuser: boolean;
  email_verified_at: string | null;
  created_at: string;
  updated_at: string;
}

export interface LoginRequest {
  email: string;
  password: string;
}

export interface LoginResponse {
  token: string;
  pid: string;
  name: string;
  is_verified: boolean;
  is_staff: boolean;
  is_superuser: boolean;
}

export interface RegisterRequest {
  name: string;
  email: string;
  password: string;
}

export interface ForgotPasswordRequest {
  email: string;
}

export interface ResetPasswordRequest {
  token: string;
  password: string;
}

export interface CurrentUserResponse {
  pid: string;
  name: string;
  email: string;
  is_staff: boolean;
  is_superuser: boolean;
}

export interface AuthState {
  user: User | null;
  token: string | null;
  isAuthenticated: boolean;
}
