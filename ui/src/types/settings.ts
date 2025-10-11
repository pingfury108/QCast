// 站点设置相关类型定义

export interface SiteSettings {
  id: number;
  site_url: string;
  created_at: string;
  updated_at: string;
}

export interface UpdateSiteSettingsRequest {
  site_url: string;
}
