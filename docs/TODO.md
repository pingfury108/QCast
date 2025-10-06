# QCast 开发任务清单

## Phase 1: 前端基础框架搭建

### 1.1 项目初始化
- [ ] 创建 Vite + React + TypeScript 项目
- [ ] 配置 Tailwind CSS
- [ ] 初始化 shadcn/ui
- [ ] 安装必要依赖（React Router、TanStack Query、Axios 等）
- [ ] 配置 Vite 代理（开发环境指向后端 5150 端口）
- [ ] 配置构建输出到 `../assets/static`

### 1.2 全局配置
- [ ] 创建全局样式和主题配置
- [ ] 配置 React Router 路由
- [ ] 配置 TanStack Query Provider
- [ ] 创建 Axios 实例和拦截器
- [ ] 创建全局 Toast 组件

### 1.3 通用组件
- [ ] 创建 Layout 组件（全局布局）
- [ ] 创建 Header 组件（顶部导航栏）
- [ ] 创建 Footer 组件
- [ ] 创建 Loading 组件（Skeleton）
- [ ] 创建 EmptyState 组件

---

## Phase 2: 认证相关页面（基于 Loco 现有 API）

### 2.1 登录注册页面
- [ ] 创建登录页面（/login）
- [ ] 创建注册页面（/register）
- [ ] 创建忘记密码页面（/forgot-password）
- [ ] 创建重置密码页面（/reset-password/:token）
- [ ] 实现表单验证（React Hook Form + Zod）

### 2.2 认证逻辑
- [ ] 创建认证 API Service（login, register, forgot, reset）
- [ ] 创建认证状态管理（TanStack Query）
- [ ] 实现 JWT Token 存储和管理
- [ ] 实现路由守卫（ProtectedRoute）
- [ ] 实现自动登录（Token 刷新）

---

## Phase 3: 首页和基础页面

### 3.1 首页（Landing Page）
- [ ] 创建首页布局（/）
- [ ] 实现 Hero 区域（标题、Slogan、CTA）
- [ ] 实现核心功能介绍区域
- [ ] 实现使用场景展示
- [ ] 实现工作流程说明
- [ ] 实现特色功能展示
- [ ] 添加响应式设计
- [ ] 实现登录状态自动跳转

---

## Phase 4: 后端 API 开发 - Books 模块

### 4.1 数据库 Migration
- [ ] 创建 Books 表 Migration
- [ ] 创建 Chapters 表 Migration
- [ ] 创建 Media 表 Migration
- [ ] 添加索引和外键约束

### 4.2 Books Model
- [ ] 创建 Books Entity
- [ ] 创建 Books Model 业务逻辑
- [ ] 创建 Books View（响应格式）
- [ ] 实现树形结构查询

### 4.3 Books Controller
- [ ] 实现 POST /api/books（创建书籍）
- [ ] 实现 GET /api/books（获取列表）
- [ ] 实现 GET /api/books/search（搜索）
- [ ] 实现 GET /api/books/:id（获取详情）
- [ ] 实现 PUT /api/books/:id（更新）
- [ ] 实现 DELETE /api/books/:id（删除）
- [ ] 实现 GET /api/books/:id/tree（获取树形结构）
- [ ] 实现 POST /api/books/:id/reorder（排序）

---

## Phase 5: 前端 Books 功能

### 5.1 Books API Service
- [ ] 创建 Books API Service（所有接口封装）
- [ ] 创建 Books TanStack Query Hooks

### 5.2 Books 页面
- [ ] 创建 Dashboard 布局（带导航栏）
- [ ] 创建书籍列表页（/dashboard/books）
- [ ] 实现树形列表展示
- [ ] 实现新建书籍对话框
- [ ] 实现编辑书籍功能
- [ ] 实现删除书籍确认
- [ ] 实现拖拽排序

### 5.3 Book 详情页
- [ ] 创建 Book 详情页（/dashboard/books/:id）
- [ ] 实现 Tab 切换（媒体/章节/设置）
- [ ] 实现媒体列表展示
- [ ] 实现返回列表功能

---

## Phase 6: 后端 Chapters 模块

### 6.1 Chapters Model
- [ ] 创建 Chapters Entity
- [ ] 创建 Chapters Model 业务逻辑
- [ ] 创建 Chapters View

### 6.2 Chapters Controller
- [ ] 实现 POST /api/books/:book_id/chapters（创建章节）
- [ ] 实现 GET /api/books/:book_id/chapters（获取列表）
- [ ] 实现 GET /api/chapters/:id（获取详情）
- [ ] 实现 PUT /api/chapters/:id（更新）
- [ ] 实现 DELETE /api/chapters/:id（删除）
- [ ] 实现 POST /api/chapters/:id/reorder（排序）

---

## Phase 7: 后端 Media 模块

### 7.1 Media Model
- [ ] 创建 Media Entity
- [ ] 创建 Media Model 业务逻辑
- [ ] 创建 Media View
- [ ] 实现 access_token 生成逻辑

### 7.2 文件存储服务
- [ ] 创建 Storage Service（保存文件）
- [ ] 实现文件类型验证
- [ ] 实现文件大小验证
- [ ] 实现 UUID 文件名生成
- [ ] 创建目录结构（users/{user_id}/books/{book_id}/media）

### 7.3 Media Controller
- [ ] 实现 POST /api/media/upload（上传文件）
- [ ] 实现 GET /api/media（获取列表）
- [ ] 实现 GET /api/media/search（搜索）
- [ ] 实现 GET /api/media/:id（获取详情）
- [ ] 实现 PUT /api/media/:id（更新元数据）
- [ ] 实现 PUT /api/media/:id/replace-file（替换文件）
- [ ] 实现 DELETE /api/media/:id（删除）
- [ ] 实现 POST /api/media/:id/publish（发布/取消发布）

---

## Phase 8: 二维码和公开访问

### 8.1 二维码服务
- [ ] 添加 qrcode crate 依赖
- [ ] 创建 QRCode Service
- [ ] 实现二维码生成（基于 access_url）
- [ ] 实现二维码保存到静态目录

### 8.2 后台 Worker
- [ ] 创建 MediaProcessor Worker
- [ ] 实现上传后自动生成二维码
- [ ] 可选：实现媒体信息提取（时长等）

### 8.3 公开访问 API
- [ ] 实现 GET /public/media/:access_token（播放）
- [ ] 实现 Range 请求支持（流媒体传输）
- [ ] 实现权限控制逻辑
- [ ] 实现播放次数统计

### 8.4 公开访问页面 Controller
- [ ] 创建 Public Controller
- [ ] 实现 GET /api/media/:id/qrcode（获取二维码）
- [ ] 实现 POST /api/media/:id/regenerate-qr（重新生成）

---

## Phase 9: 前端 Media 功能

### 9.1 Media API Service
- [ ] 创建 Media API Service
- [ ] 创建 Media TanStack Query Hooks
- [ ] 实现文件上传进度监听

### 9.2 上传页面
- [ ] 创建上传页面（/dashboard/upload）
- [ ] 实现拖拽上传（react-dropzone）
- [ ] 实现文件类型和大小验证
- [ ] 实现上传队列展示
- [ ] 实现上传进度条
- [ ] 实现批量上传（最多 10 个）

### 9.3 Media 列表页
- [ ] 创建 Media 列表页（/dashboard/media）
- [ ] 实现搜索功能
- [ ] 实现筛选功能（类型、状态）
- [ ] 实现媒体卡片展示
- [ ] 实现操作菜单（编辑、删除、复制链接等）

### 9.4 Media 详情/编辑
- [ ] 创建编辑媒体对话框
- [ ] 实现元数据编辑（标题、描述）
- [ ] 实现文件替换功能
- [ ] 实现发布/取消发布
- [ ] 实现二维码显示和下载

---

## Phase 10: 公开访问前端

### 10.1 公开播放页面
- [ ] 创建公开访问页面（/public/:token）
- [ ] 实现音频播放器（Plyr）
- [ ] 实现视频播放器（video.js）
- [ ] 实现播放器控制栏
- [ ] 实现媒体信息展示
- [ ] 实现二维码展示
- [ ] 实现复制链接功能
- [ ] 实现分享功能
- [ ] 实现下载功能

---

## Phase 11: 优化和完善

### 11.1 错误处理
- [ ] 统一错误提示 Toast
- [ ] 实现 404 页面
- [ ] 实现 500 错误页面
- [ ] 实现网络错误提示

### 11.2 加载状态
- [ ] 添加 Skeleton Loading
- [ ] 添加 Empty State
- [ ] 添加加载指示器

### 11.3 响应式设计
- [ ] 优化移动端布局
- [ ] 优化平板端布局
- [ ] 测试各种屏幕尺寸

### 11.4 性能优化
- [ ] 实现图片懒加载
- [ ] 实现路由懒加载
- [ ] 优化打包体积
- [ ] 实现缓存策略

---

## Phase 12: 测试和部署

### 12.1 测试
- [ ] 后端单元测试
- [ ] 前端组件测试
- [ ] E2E 测试
- [ ] 手动测试完整流程

### 12.2 部署准备
- [ ] 配置生产环境变量
- [ ] 前端构建配置
- [ ] 后端生产配置
- [ ] 数据库迁移脚本

### 12.3 文档
- [ ] API 文档
- [ ] 用户使用文档
- [ ] 部署文档

---

## 可选功能（未来扩展）

- [ ] Chapters 管理功能
- [ ] 批量操作（批量删除、批量发布）
- [ ] 数据统计（播放次数、时长统计）
- [ ] 用户设置页面
- [ ] 暗色模式
- [ ] 国际化（i18n）
- [ ] 文件版本历史管理
- [ ] 封面图上传和管理
- [ ] RSS 订阅功能
