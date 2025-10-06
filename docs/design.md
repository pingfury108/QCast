# QCast 音视频托管平台设计文档

## 一、项目概述

QCast 是一个个人音视频托管平台，用户可以上传音频、视频文件，为这些文件生成固定的访问链接和二维码，并通过 Book 进行分类管理。

### 核心功能
1. **用户管理**：注册、登录、认证
2. **Book 管理**：书籍/专辑/播客系列等（支持层级结构）
3. **媒体文件管理**：上传音频/视频，生成固定访问链接
4. **二维码生成**：为每个媒体文件生成二维码
5. **层级组织**：Book → Chapter → Media 结构

---

## 二、数据库模型设计

### 2.1 Books 表（书籍/专辑/系列）

```sql
CREATE TABLE books (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    cover_image VARCHAR(512),
    parent_id INTEGER,              -- 支持多级分类
    sort_order INTEGER DEFAULT 0,
    is_public BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP,
    updated_at TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (parent_id) REFERENCES books(id)
);
```

### 2.2 Chapters 表（章节/集）

```sql
CREATE TABLE chapters (
    id INTEGER PRIMARY KEY,
    book_id INTEGER NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    sort_order INTEGER DEFAULT 0,
    created_at TIMESTAMP,
    updated_at TIMESTAMP,
    FOREIGN KEY (book_id) REFERENCES books(id) ON DELETE CASCADE
);
```

### 2.3 Media 表（媒体文件）

```sql
CREATE TABLE media (
    id INTEGER PRIMARY KEY,
    chapter_id INTEGER,              -- 可选，属于某章节
    book_id INTEGER NOT NULL,        -- 必须属于某个 Book
    user_id INTEGER NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    file_type VARCHAR(10) NOT NULL,  -- 'audio' or 'video'
    file_path VARCHAR(512) NOT NULL, -- 存储路径（可更新）
    file_size BIGINT,
    duration INTEGER,                -- 时长（秒）
    mime_type VARCHAR(100),
    access_token VARCHAR(64) UNIQUE NOT NULL, -- 公开访问唯一标识（创建后永不改变）
    access_url VARCHAR(512),         -- 完整访问链接（基于 access_token 生成）
    qr_code_path VARCHAR(512),       -- 二维码图片路径
    file_version INTEGER DEFAULT 1,  -- 文件版本号（支持文件替换）
    original_filename VARCHAR(255),  -- 原始文件名
    play_count INTEGER DEFAULT 0,    -- 播放次数
    is_public BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP,
    updated_at TIMESTAMP,
    FOREIGN KEY (book_id) REFERENCES books(id) ON DELETE CASCADE,
    FOREIGN KEY (chapter_id) REFERENCES chapters(id) ON DELETE SET NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);
```

**重要说明：**
- `access_token`: 在媒体记录创建时生成，永不改变，保证访问链接永久有效
- `file_path`: 可以通过替换文件功能更新，但 `access_token` 保持不变
- `file_version`: 每次替换文件时自增，用于版本追踪
- `original_filename`: 保存用户上传的原始文件名，方便下载时使用

### 2.4 层级关系

```
User
  └── Book (可多级嵌套，通过 parent_id)
        ├── Chapter (可选)
        │     └── Media
        └── Media (直接属于 Book，不属于 Chapter)
```

---

## 三、后端 API 设计

### 3.1 项目结构

```
src/
├── controllers/
│   ├── auth.rs           # 用户认证（已有）
│   ├── books.rs          # 书籍管理
│   ├── chapters.rs       # 章节管理
│   ├── media.rs          # 媒体文件管理
│   └── public.rs         # 公开访问
├── models/
│   ├── _entities/
│   │   ├── users.rs      # (已有)
│   │   ├── books.rs
│   │   ├── chapters.rs
│   │   └── media.rs
│   ├── books.rs          # Book 业务逻辑
│   ├── chapters.rs       # Chapter 业务逻辑
│   └── media.rs          # Media 业务逻辑
├── services/
│   ├── storage.rs        # 文件存储服务
│   ├── qrcode.rs         # 二维码生成
│   └── media_processor.rs # 媒体处理（提取时长、缩略图等）
├── workers/
│   └── media_processor.rs # 后台处理媒体文件
└── views/
    ├── books.rs          # Book 响应视图
    ├── chapters.rs       # Chapter 响应视图
    └── media.rs          # Media 响应视图
```

### 3.2 API 路由设计

#### Books API（需要认证）

```
POST   /api/books                     创建书籍
GET    /api/books                     获取我的书籍列表（支持树形结构）
GET    /api/books/search?q=关键词     搜索我的书籍
GET    /api/books/:id                 获取书籍详情
PUT    /api/books/:id                 更新书籍
DELETE /api/books/:id                 删除书籍
GET    /api/books/:id/tree            获取书籍完整树形结构（含章节和媒体）
POST   /api/books/:id/reorder         调整书籍顺序
```

**请求/响应示例：**

```json
// POST /api/books
{
  "title": "我的播客系列",
  "description": "这是我的第一个播客",
  "parent_id": null,
  "is_public": true
}

// Response
{
  "id": 1,
  "user_id": 1,
  "title": "我的播客系列",
  "description": "这是我的第一个播客",
  "cover_image": null,
  "parent_id": null,
  "is_public": true,
  "created_at": "2025-10-06T12:00:00Z"
}
```

#### Chapters API（需要认证）

```
POST   /api/books/:book_id/chapters   创建章节
GET    /api/books/:book_id/chapters   获取章节列表
GET    /api/chapters/:id              获取章节详情
PUT    /api/chapters/:id              更新章节
DELETE /api/chapters/:id              删除章节
POST   /api/chapters/:id/reorder      调整章节顺序
```

#### Media API（需要认证）

```
POST   /api/media/upload              上传媒体文件（multipart/form-data）
GET    /api/media                     获取我的媒体列表
GET    /api/media/search?q=关键词     搜索我的媒体
GET    /api/media/:id                 获取媒体详情
PUT    /api/media/:id                 更新媒体信息（标题、描述等元数据）
PUT    /api/media/:id/replace-file    替换媒体文件（保持访问链接不变）
DELETE /api/media/:id                 删除媒体
GET    /api/media/:id/qrcode          获取二维码图片
POST   /api/media/:id/regenerate-qr   重新生成二维码
POST   /api/media/:id/publish         发布/取消发布
GET    /api/media/:id/history         获取文件版本历史
```

**上传请求示例：**

```
POST /api/media/upload
Content-Type: multipart/form-data

file: [binary data]
title: "第一集"
description: "这是第一集的内容"
book_id: 1
chapter_id: 1  (可选)
```

**响应示例：**

```json
{
  "id": 1,
  "title": "第一集",
  "description": "这是第一集的内容",
  "file_type": "audio",
  "file_size": 10485760,
  "duration": 1200,
  "access_token": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "access_url": "http://localhost:5150/public/media/a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "qr_code_path": "/static/qrcodes/1.png",
  "file_version": 1,
  "original_filename": "episode1.mp3",
  "is_public": false,
  "created_at": "2025-10-06T12:00:00Z",
  "updated_at": "2025-10-06T12:00:00Z"
}
```

**替换文件请求示例：**

```
PUT /api/media/:id/replace-file
Content-Type: multipart/form-data

file: [新的媒体文件]
```

**替换文件响应示例：**

```json
{
  "id": 1,
  "title": "第一集",
  "file_type": "audio",
  "file_size": 12345678,
  "duration": 1350,
  "access_token": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",  // 保持不变
  "access_url": "http://localhost:5150/public/media/a1b2c3d4-e5f6-7890-abcd-ef1234567890",  // 保持不变
  "file_version": 2,  // 版本号自增
  "original_filename": "episode1_v2.mp3",
  "updated_at": "2025-10-06T14:30:00Z"
}
```

#### Public API（无需认证）

```
GET    /public/media/:access_token    播放媒体（支持 Range 请求）
GET    /public/media/:access_token/info  获取媒体信息
GET    /public/books/:id              查看公开书籍
```

---

## 四、技术选型

### 4.1 后端技术栈

| 功能 | 技术方案 | Rust Crate | 说明 |
|------|---------|-----------|------|
| 框架 | Loco | `loco-rs = "0.16"` | 已有 |
| 文件上传 | Multipart form | `axum-extra` | 已有 |
| 文件存储 | 本地存储 | `tokio::fs` | 初期使用 |
| 二维码生成 | QR Code | `qrcode` | 待添加 |
| 图片处理 | Image | `image` | 待添加 |
| 媒体信息提取 | FFmpeg | `ffmpeg-next` | 可选 |
| 流媒体传输 | Range 请求 | `tower-http` | 待添加 |
| UUID 生成 | UUID v4 | `uuid` | 已有 |

**Cargo.toml 需要添加的依赖：**

```toml
qrcode = "0.14"
image = "0.25"
tower-http = { version = "0.6", features = ["fs", "trace"] }
bytes = "1.5"
```

### 4.2 前端技术栈

```
ui/
├── React 18 + TypeScript
├── 构建工具: Vite
├── 包管理: pnpm
├── 状态管理: TanStack Query (React Query)
├── UI 框架: shadcn/ui（基于 Radix UI + Tailwind CSS）
├── 样式: Tailwind CSS
├── 路由: React Router v6
├── 表单验证: React Hook Form + Zod
├── 文件上传: react-dropzone
├── 播放器:
│   ├── 音频: Plyr
│   └── 视频: video.js
├── 二维码: qrcode.react
└── HTTP 客户端: axios
```

**shadcn/ui 组件使用：**
- Button, Input, Form, Label
- Card, Dialog, DropdownMenu
- Table, Tabs, Toast
- Progress, Skeleton
- 暗色模式支持

---

## 五、前端页面设计

### 5.1 页面结构

```
/                       首页（Landing Page，宣传介绍页，无需登录）
/login                  登录页
/register               注册页
/forgot-password        忘记密码页
/reset-password/:token  重置密码页

/dashboard              用户控制台（需要认证）
  ├── /books            书籍管理（树形列表）
  ├── /books/new        创建新书籍
  ├── /books/:id        书籍详情（编辑、章节、媒体）
  ├── /books/:id/edit   编辑书籍
  ├── /upload           批量上传媒体
  └── /media            所有媒体文件管理

/public/:token          公开访问页面（无需认证）
  ├── 媒体播放器
  ├── 二维码展示
  └── 分享按钮
```

### 5.2 核心页面设计

#### 登录页（/login）

```
┌─────────────────────────────────────────────────────────┐
│  QCast                                      [返回首页]  │
├─────────────────────────────────────────────────────────┤
│                                                          │
│                    🎵 QCast                              │
│                  欢迎回来                                │
│                                                          │
│     ┌────────────────────────────────────────┐          │
│     │  登录                                  │          │
│     │                                        │          │
│     │  邮箱                                  │          │
│     │  ┌──────────────────────────────────┐ │          │
│     │  │ your@email.com                   │ │          │
│     │  └──────────────────────────────────┘ │          │
│     │                                        │          │
│     │  密码                                  │          │
│     │  ┌──────────────────────────────────┐ │          │
│     │  │ ••••••••••                       │ │          │
│     │  └──────────────────────────────────┘ │          │
│     │                                        │          │
│     │  [ ] 记住我     [忘记密码？]          │          │
│     │                                        │          │
│     │  ┌──────────────────────────────────┐ │          │
│     │  │         登  录                   │ │          │
│     │  └──────────────────────────────────┘ │          │
│     │                                        │          │
│     │  还没有账号？ [立即注册]              │          │
│     └────────────────────────────────────────┘          │
│                                                          │
└─────────────────────────────────────────────────────────┘
```

**shadcn/ui 组件：**
- `Card` - 登录表单容器
- `Form` + `Input` + `Label` - 表单字段
- `Button` - 登录按钮
- `Checkbox` - 记住我
- `Link` - 跳转链接

**表单验证（Zod）：**
```typescript
const loginSchema = z.object({
  email: z.string().email("请输入有效的邮箱地址"),
  password: z.string().min(6, "密码至少 6 位"),
  remember: z.boolean().optional(),
});
```

---

#### 注册页（/register）

```
┌─────────────────────────────────────────────────────────┐
│  QCast                                      [返回首页]  │
├─────────────────────────────────────────────────────────┤
│                                                          │
│                    🎵 QCast                              │
│                  创建你的账号                            │
│                                                          │
│     ┌────────────────────────────────────────┐          │
│     │  注册                                  │          │
│     │                                        │          │
│     │  用户名                                │          │
│     │  ┌──────────────────────────────────┐ │          │
│     │  │ 请输入用户名                     │ │          │
│     │  └──────────────────────────────────┘ │          │
│     │                                        │          │
│     │  邮箱                                  │          │
│     │  ┌──────────────────────────────────┐ │          │
│     │  │ your@email.com                   │ │          │
│     │  └──────────────────────────────────┘ │          │
│     │                                        │          │
│     │  密码                                  │          │
│     │  ┌──────────────────────────────────┐ │          │
│     │  │ ••••••••••                       │ │          │
│     │  └──────────────────────────────────┘ │          │
│     │  至少 6 位字符                         │          │
│     │                                        │          │
│     │  确认密码                              │          │
│     │  ┌──────────────────────────────────┐ │          │
│     │  │ ••••••••••                       │ │          │
│     │  └──────────────────────────────────┘ │          │
│     │                                        │          │
│     │  ┌──────────────────────────────────┐ │          │
│     │  │         注  册                   │ │          │
│     │  └──────────────────────────────────┘ │          │
│     │                                        │          │
│     │  已有账号？ [立即登录]                │          │
│     └────────────────────────────────────────┘          │
│                                                          │
└─────────────────────────────────────────────────────────┘
```

**shadcn/ui 组件：**
- `Card` - 注册表单容器
- `Form` + `Input` + `Label` - 表单字段
- `Button` - 注册按钮
- `Alert` - 错误提示

**表单验证（Zod）：**
```typescript
const registerSchema = z.object({
  name: z.string().min(2, "用户名至少 2 位"),
  email: z.string().email("请输入有效的邮箱地址"),
  password: z.string().min(6, "密码至少 6 位"),
  confirmPassword: z.string(),
}).refine((data) => data.password === data.confirmPassword, {
  message: "两次密码不一致",
  path: ["confirmPassword"],
});
```

---

#### 忘记密码页（/forgot-password）

```
┌─────────────────────────────────────────────────────────┐
│  QCast                                      [返回登录]  │
├─────────────────────────────────────────────────────────┤
│                                                          │
│                    🎵 QCast                              │
│                  重置密码                                │
│                                                          │
│     ┌────────────────────────────────────────┐          │
│     │  找回密码                              │          │
│     │                                        │          │
│     │  请输入你的注册邮箱，我们将发送重置    │          │
│     │  密码链接到你的邮箱。                  │          │
│     │                                        │          │
│     │  邮箱                                  │          │
│     │  ┌──────────────────────────────────┐ │          │
│     │  │ your@email.com                   │ │          │
│     │  └──────────────────────────────────┘ │          │
│     │                                        │          │
│     │  ┌──────────────────────────────────┐ │          │
│     │  │      发送重置链接                │ │          │
│     │  └──────────────────────────────────┘ │          │
│     │                                        │          │
│     │  [返回登录]                            │          │
│     └────────────────────────────────────────┘          │
│                                                          │
└─────────────────────────────────────────────────────────┘
```

---

#### 重置密码页（/reset-password/:token）

```
┌─────────────────────────────────────────────────────────┐
│  QCast                                                   │
├─────────────────────────────────────────────────────────┤
│                                                          │
│                    🎵 QCast                              │
│                  设置新密码                              │
│                                                          │
│     ┌────────────────────────────────────────┐          │
│     │  重置密码                              │          │
│     │                                        │          │
│     │  新密码                                │          │
│     │  ┌──────────────────────────────────┐ │          │
│     │  │ ••••••••••                       │ │          │
│     │  └──────────────────────────────────┘ │          │
│     │  至少 6 位字符                         │          │
│     │                                        │          │
│     │  确认新密码                            │          │
│     │  ┌──────────────────────────────────┐ │          │
│     │  │ ••••••••••                       │ │          │
│     │  └──────────────────────────────────┘ │          │
│     │                                        │          │
│     │  ┌──────────────────────────────────┐ │          │
│     │  │      重置密码                    │ │          │
│     │  └──────────────────────────────────┘ │          │
│     │                                        │          │
│     └────────────────────────────────────────┘          │
│                                                          │
└─────────────────────────────────────────────────────────┘
```

---

#### 首页（/ - Landing Page）

**设计目标：** 宣传站点功能，引导用户注册/登录

```
┌─────────────────────────────────────────────────────────┐
│  QCast                           [登录] [注册]          │
├─────────────────────────────────────────────────────────┤
│                                                          │
│              🎵 QCast 音视频托管平台                     │
│                                                          │
│         轻松托管你的音频和视频，随时随地分享             │
│                                                          │
│              [开始使用] [了解更多]                       │
│                                                          │
├─────────────────────────────────────────────────────────┤
│  核心功能                                                │
│                                                          │
│  📤 上传媒体              🔗 永久链接                    │
│  快速上传音频、视频      生成永久访问链接                │
│  支持批量操作            链接永不过期                    │
│                                                          │
│  📱 二维码分享            📚 层级管理                    │
│  自动生成二维码          Book/Chapter/Media             │
│  扫码即可播放            结构化组织内容                  │
│                                                          │
├─────────────────────────────────────────────────────────┤
│  使用场景                                                │
│                                                          │
│  🎙️ 播客制作者           📖 有声书作者                  │
│  托管你的播客系列        管理你的有声书章节              │
│                                                          │
│  🎓 在线课程             🎬 视频创作者                   │
│  组织课程视频            分享你的视频作品                │
│                                                          │
├─────────────────────────────────────────────────────────┤
│  工作流程                                                │
│                                                          │
│  1️⃣ 创建 Book          2️⃣ 上传媒体文件                 │
│  创建播客、专辑或课程    上传你的音视频文件              │
│                                                          │
│  3️⃣ 生成分享链接        4️⃣ 分享给观众                  │
│  自动生成链接和二维码    通过链接或二维码分享            │
│                                                          │
├─────────────────────────────────────────────────────────┤
│  特色功能                                                │
│                                                          │
│  ✅ 永久访问链接 - 链接生成后永不改变                    │
│  ✅ 文件可替换 - 可以更新文件但链接保持不变              │
│  ✅ 版本管理 - 追踪每次文件更新                          │
│  ✅ 二维码分享 - 自动生成二维码，扫码即播                │
│  ✅ 私密控制 - 只有你能管理，他人只能通过链接访问        │
│                                                          │
├─────────────────────────────────────────────────────────┤
│                  [立即开始]                              │
│                                                          │
│  Footer: © 2025 QCast | 关于我们 | 使用条款 | 隐私政策  │
└─────────────────────────────────────────────────────────┘
```

**交互逻辑：**
- **已登录用户访问首页**：自动跳转到 `/dashboard`
- **未登录用户**：显示完整宣传页
- **点击"开始使用"/"立即开始"**：跳转到 `/register`
- **点击"了解更多"**：页面滚动到功能介绍区域

**技术实现：**
- 纯静态页面，无需调用后端 API
- 响应式设计，支持移动端访问
- 可选：添加动画效果（渐入、滚动视差等）

---

#### 书籍管理页面（/dashboard/books）

```
┌─────────────────────────────────────────────────────────┐
│  QCast                    [我的书籍] [上传] [用户头像▼] │
├─────────────────────────────────────────────────────────┤
│  我的书籍                                  [+ 新建书籍] │
├─────────────────────────────────────────────────────────┤
│  🔍 搜索...                                              │
├─────────────────────────────────────────────────────────┤
│  📚 Podcast 系列 1                      [编辑] [删除]   │
│    ├── 📖 Season 1                                       │
│    │   ├── 🎵 Episode 1          [👁️ 公开] [QR]        │
│    │   └── 🎵 Episode 2          [🔒 私密] [QR]        │
│    └── 📖 Season 2                                       │
│        └── 🎵 Episode 3          [👁️ 公开] [QR]        │
│                                                          │
│  📚 有声书                              [编辑] [删除]   │
│    ├── 🎵 第一章                    [👁️ 公开] [QR]     │
│    └── 🎵 第二章                    [🔒 私密] [QR]     │
└─────────────────────────────────────────────────────────┘
```

**全局导航栏：**
- Logo: QCast（点击返回 Dashboard）
- 导航菜单: 我的书籍、上传
- 用户菜单: 头像下拉（设置、退出登录）

---

#### Dashboard 首页（/dashboard）

**说明：** 用户登录后的默认页面，自动跳转到 `/dashboard/books`

---

#### Book 详情页（/dashboard/books/:id）

```
┌─────────────────────────────────────────────────────────┐
│  QCast                    [我的书籍] [上传] [用户头像▼] │
├─────────────────────────────────────────────────────────┤
│  ← 返回书籍列表                                         │
├─────────────────────────────────────────────────────────┤
│  📚 Podcast 系列 1                      [编辑] [删除]   │
│  描述: 这是我的第一个播客系列                            │
│  创建时间: 2025-10-06  |  媒体数: 5                     │
│  状态: 👁️ 公开                                          │
├─────────────────────────────────────────────────────────┤
│  [📖 章节] [🎵 媒体] [⚙️ 设置]                          │
├─────────────────────────────────────────────────────────┤
│  媒体列表                                  [+ 上传媒体] │
│                                                          │
│  🎵 Episode 1                           [👁️] [QR] [⋮]  │
│  时长: 20:15  |  播放: 123次  |  v1                     │
│                                                          │
│  🎵 Episode 2                           [🔒] [QR] [⋮]  │
│  时长: 18:30  |  播放: 45次   |  v2                     │
│                                                          │
│  🎵 Episode 3                           [👁️] [QR] [⋮]  │
│  时长: 22:00  |  播放: 89次   |  v1                     │
└─────────────────────────────────────────────────────────┘
```

**交互说明：**
- 点击媒体行 → 展开详情/编辑
- [⋮] 菜单 → 编辑、替换文件、删除、复制链接
- Tab 切换：章节、媒体、设置

---

#### Media 列表页（/dashboard/media）

```
┌─────────────────────────────────────────────────────────┐
│  QCast                    [我的书籍] [上传] [用户头像▼] │
├─────────────────────────────────────────────────────────┤
│  所有媒体                                  [+ 上传媒体] │
├─────────────────────────────────────────────────────────┤
│  🔍 搜索媒体...                  [筛选: 全部 ▼]         │
├─────────────────────────────────────────────────────────┤
│  📚 Podcast 系列 1 / Episode 1           [👁️] [QR] [⋮] │
│  🎵 音频  |  20:15  |  123次播放  |  v1                 │
│  2025-10-06 创建                                        │
│                                                          │
│  📚 Podcast 系列 1 / Episode 2           [🔒] [QR] [⋮] │
│  🎵 音频  |  18:30  |  45次播放   |  v2                 │
│  2025-10-05 创建                                        │
│                                                          │
│  📚 有声书 / 第一章                      [👁️] [QR] [⋮] │
│  🎵 音频  |  35:20  |  67次播放   |  v1                 │
│  2025-10-04 创建                                        │
└─────────────────────────────────────────────────────────┘
```

**功能说明：**
- 显示所有媒体文件（跨 Book）
- 支持搜索和筛选（全部/音频/视频、公开/私密）
- 显示所属 Book 路径

---

#### 上传页面（/dashboard/upload）

```
┌─────────────────────────────────────────────────────────┐
│  QCast                    [我的书籍] [上传] [用户头像▼] │
├─────────────────────────────────────────────────────────┤
│  上传媒体文件                                            │
├─────────────────────────────────────────────────────────┤
│  选择归属书籍: [下拉选择]                               │
│  选择归属章节: [下拉选择] (可选)                        │
├─────────────────────────────────────────────────────────┤
│  ┌───────────────────────────────────────────────────┐  │
│  │   拖拽文件到此处                                  │  │
│  │   或点击选择文件                                  │  │
│  │   支持 MP3, MP4, WAV, M4A 等格式                 │  │
│  └───────────────────────────────────────────────────┘  │
├─────────────────────────────────────────────────────────┤
│  上传队列:                                               │
│  ✓ episode1.mp3  (100%)                                 │
│  ⏳ episode2.mp3  (45%)                                  │
│  ⏸️ episode3.mp4  (等待中)                               │
└─────────────────────────────────────────────────────────┘
```

#### 公开访问页面（/public/:token）

```
┌─────────────────────────────────────────────────────────┐
│  QCast                                                   │
├─────────────────────────────────────────────────────────┤
│   ┌───────────────────────────────────────────────┐     │
│   │          [封面图/缩略图]                      │     │
│   └───────────────────────────────────────────────┘     │
│                                                          │
│   标题: 第一集                                           │
│   描述: 这是第一集的内容                                 │
│   时长: 20:00                                            │
│   播放次数: 123                                          │
│                                                          │
│   ━━━━━━━━━━━━●──────────────── 12:34/20:00            │
│   [⏮️] [⏯️] [⏭️]  [🔊] [⚙️] [全屏]                      │
│                                                          │
│   ┌──────────────┐                                      │
│   │              │                                      │
│   │   QR Code    │                                      │
│   │              │                                      │
│   └──────────────┘                                      │
│                                                          │
│   [📋 复制链接] [📱 分享] [⬇️ 下载]                     │
│                                                          │
│   Powered by QCast                                      │
└─────────────────────────────────────────────────────────┘
```

**说明：**
- 公开页面保持简洁，只显示 Logo
- 无需登录，不显示用户菜单
- 底部添加品牌标识

---

### 5.3 UI 设计规范

#### 统一元素

**1. 页面宽度：**
- 所有页面使用统一宽度（约 57 个字符宽）
- 响应式设计：移动端自适应

**2. 顶部导航栏：**
- **首页（未登录）**：`QCast  [登录] [注册]`
- **Dashboard 页面**：`QCast  [我的书籍] [上传] [用户头像▼]`
- **公开访问页面**：`QCast`（仅 Logo）
- 高度固定，背景色统一

**3. 按钮样式：**
- 主要按钮：`[开始使用]` `[+ 新建书籍]`
- 次要按钮：`[编辑]` `[删除]` `[QR]`
- 图标按钮：`[📋 复制链接]` `[📱 分享]`

**4. 图标使用：**
- 📚 Book（书籍）
- 📖 Chapter（章节）
- 🎵 Audio（音频）
- 🎬 Video（视频）
- 👁️ 公开
- 🔒 私密
- ✓ 完成
- ⏳ 进行中

**5. 颜色体系（建议）：**
- 主色调：蓝色系（品牌色）
- 成功：绿色（✓）
- 警告：黄色（⏳）
- 错误：红色
- 中性：灰色（文本、边框）

**6. 间距规范：**
- 页面边距：统一
- 卡片间距：一致
- 按钮间距：标准化

**7. 字体层级：**
- H1: 页面标题（如"我的书籍"）
- H2: 区域标题（如"核心功能"）
- H3: 卡片标题（如"Podcast 系列 1"）
- Body: 正文内容

---

## 六、文件存储设计

### 6.1 目录结构

```
uploads/
  ├── users/
  │   └── {user_id}/
  │       └── books/
  │           └── {book_id}/
  │               ├── media/
  │               │   ├── {uuid}.mp3
  │               │   ├── {uuid}.mp4
  │               │   └── ...
  │               └── covers/
  │                   └── {uuid}.jpg
  └── qrcodes/
      ├── {media_id}.png
      └── ...

assets/
  └── static/
      ├── qrcodes/          # 二维码静态访问路径
      └── uploads/          # 软链接到 uploads/ (可选)
```

### 6.2 访问策略

- **媒体文件**: 通过 `/public/media/:access_token` 访问，后端校验权限
- **二维码**: 通过 `/static/qrcodes/:id.png` 直接访问（静态文件）
- **封面图**: 通过 `/static/covers/:filename` 访问

### 6.3 文件命名规则

```rust
// 媒体文件
let file_name = format!("{}.{}", Uuid::new_v4(), extension);

// 二维码
let qr_name = format!("{}.png", media_id);

// 访问 token
let access_token = Uuid::new_v4().to_string();
```

---

## 七、核心功能实现细节

### 7.1 文件上传流程

```
1. 前端选择文件 → 验证格式和大小
2. 上传到 /api/media/upload (multipart/form-data)
3. 后端接收文件
   ├── 保存到临时目录
   ├── 验证文件类型（MIME type）
   ├── 生成 UUID 文件名
   ├── 移动到目标目录 uploads/users/{user_id}/books/{book_id}/media/
   ├── 提取媒体信息（时长、码率等）- 可选
   ├── 生成访问 token（UUID v4，永不改变）
   ├── 生成访问 URL
   ├── 保存原始文件名
   └── 保存到数据库（file_version = 1）
4. 后台任务（Worker）
   ├── 生成二维码
   ├── 生成缩略图（视频）
   └── 更新数据库
5. 返回响应给前端
```

### 7.2 文件替换流程（新增）

```
1. 前端选择新文件上传到 PUT /api/media/:id/replace-file
2. 后端接收文件
   ├── 验证用户权限（只有所有者可以替换）
   ├── 获取现有 media 记录
   ├── 保留 access_token 不变
   ├── 验证新文件类型
   ├── 生成新的 UUID 文件名
   ├── 删除旧文件
   ├── 保存新文件到同一目录
   ├── 提取新文件的媒体信息
   └── 更新数据库
       ├── file_path → 新路径
       ├── file_size → 新大小
       ├── duration → 新时长
       ├── file_version → +1
       ├── original_filename → 新文件名
       ├── updated_at → 当前时间
       └── access_token 保持不变（重要！）
3. 二维码无需重新生成（因为 access_url 未变）
4. 返回响应给前端
```

**实现示例代码：**

```rust
// 替换媒体文件
pub async fn replace_media_file(
    db: &DatabaseConnection,
    media_id: i32,
    user_id: i32,
    new_file: UploadedFile,
) -> Result<media::Model> {
    // 1. 获取现有媒体记录并验证权限
    let media = media::Entity::find_by_id(media_id)
        .one(db)
        .await?
        .ok_or("Media not found")?;

    if media.user_id != user_id {
        return Err("Permission denied");
    }

    // 2. 保存新文件
    let new_file_path = storage::save_file(&new_file).await?;

    // 3. 删除旧文件
    if tokio::fs::metadata(&media.file_path).await.is_ok() {
        tokio::fs::remove_file(&media.file_path).await?;
    }

    // 4. 提取新文件信息
    let duration = extract_duration(&new_file_path).await?;

    // 5. 更新数据库（保持 access_token 不变）
    let mut active_model: media::ActiveModel = media.into();
    active_model.file_path = Set(new_file_path);
    active_model.file_size = Set(new_file.size);
    active_model.duration = Set(Some(duration));
    active_model.file_version = Set(media.file_version + 1);
    active_model.original_filename = Set(Some(new_file.filename));
    active_model.updated_at = Set(chrono::Utc::now().naive_utc());
    // access_token 不变

    let updated_media = active_model.update(db).await?;

    Ok(updated_media)
}
```

### 7.3 二维码生成

```rust
use qrcode::{QrCode, render::svg};
use image::Luma;

// 生成二维码
let code = QrCode::new(access_url)?;
let image = code.render::<Luma<u8>>().build();
image.save(qr_code_path)?;
```

**说明：**
- 二维码基于 `access_url` 生成，因为 `access_token` 不变，二维码也永久有效
- 替换文件时无需重新生成二维码

### 7.4 流媒体传输（支持 Range 请求）

```rust
// 使用 tower-http 提供文件服务
use tower_http::services::ServeFile;

// 支持 Range 请求，实现断点续传和拖动播放
// 响应头: Accept-Ranges: bytes
// 请求头: Range: bytes=0-1023
```

### 7.5 权限控制

```rust
// 访问控制逻辑
pub async fn can_access_media(
    db: &DatabaseConnection,
    access_token: &str,
    user_id: Option<i32>,
) -> Result<bool> {
    let media = Media::find_by_access_token(db, access_token).await?;

    // 公开文件任何人都可以访问
    if media.is_public {
        return Ok(true);
    }

    // 私有文件只有所有者可以访问
    if let Some(uid) = user_id {
        return Ok(media.user_id == uid);
    }

    Ok(false)
}
```

**说明：**
- 使用 `access_token` 而非 `media_id` 进行访问控制
- 即使文件被替换，访问权限逻辑保持不变

---

## 八、实施方案

### 方案 A：简化版 MVP（推荐先实现）

**数据模型：**
```
User → Book → Media
```

**特点：**
- 不实现 Chapter 层级
- Book 支持 `parent_id`（预留嵌套能力）
- 快速验证核心功能

**开发顺序：**
1. ✅ 用户认证（已完成）
2. 数据库 Migration（books, media 表，包含 file_version 和 original_filename 字段）
3. Book CRUD API
4. Media 上传功能（生成永久 access_token）
5. Media 文件替换功能（保持 access_token 不变）
6. 二维码生成服务
7. 公开访问 API
8. 流媒体播放
9. 前端 UI 开发

### 方案 B：完整版

**数据模型：**
```
User → Book (可嵌套) → Chapter → Media
```

**特点：**
- 支持多级 Book 分类
- 支持 Chapter 管理
- 更灵活的内容组织

**实施建议：**
- 在方案 A 基础上增加 Chapter 相关功能
- 适合内容较多、需要精细管理的场景

---

## 九、技术难点和解决方案

### 9.1 大文件上传

**问题：** 上传大型视频文件可能超时或占用大量内存

**解决方案：**
- 使用流式上传（streaming）
- 限制文件大小（如 500MB）
- 可选：实现分片上传（chunked upload）

### 9.2 媒体信息提取

**问题：** 需要提取音视频时长、码率等信息

**解决方案：**
- 方案 1：使用 `ffmpeg-next` crate（需要系统安装 FFmpeg）
- 方案 2：使用 `symphonia` crate（纯 Rust，仅支持音频）
- 方案 3：前端提取（使用 HTML5 API）

### 9.3 流媒体性能

**问题：** 大量并发播放可能影响性能

**解决方案：**
- 使用 CDN（生产环境）
- 本地开发使用 tower-http 的静态文件服务
- 实现缓存策略

### 9.4 文件替换的原子性（新增）

**问题：** 替换文件时如果中途失败，可能导致文件丢失

**解决方案：**
```rust
// 使用两阶段提交策略
async fn replace_file_safely(old_path: &str, new_path: &str) -> Result<()> {
    // 1. 先保存新文件到临时位置
    let temp_path = format!("{}.new", old_path);
    fs::copy(new_path, &temp_path).await?;

    // 2. 备份旧文件
    let backup_path = format!("{}.bak", old_path);
    if fs::metadata(old_path).await.is_ok() {
        fs::rename(old_path, &backup_path).await?;
    }

    // 3. 移动新文件到正式位置
    match fs::rename(&temp_path, old_path).await {
        Ok(_) => {
            // 成功后删除备份
            fs::remove_file(&backup_path).await.ok();
            Ok(())
        }
        Err(e) => {
            // 失败时恢复备份
            fs::rename(&backup_path, old_path).await.ok();
            Err(e)
        }
    }
}
```

---

## 十、未来扩展

1. **统计分析**
   - 播放次数、时长统计
   - 用户行为分析

2. **高级功能**
   - 评论系统
   - 点赞/收藏
   - 播放列表
   - RSS 订阅

3. **存储优化**
   - 对接云存储（S3, OSS）
   - 自动转码（统一格式）
   - 自动压缩

4. **社交功能**
   - 公开主页
   - 关注/订阅
   - 分享到社交媒体

5. **文件版本管理（新增）**
   - 保留历史版本
   - 版本回滚功能
   - 版本对比
   - 自动清理旧版本

---

## 十一、文件格式和限制规范

### 11.1 支持的文件格式

**音频格式：**
- MP3 (audio/mpeg)
- M4A (audio/mp4)
- WAV (audio/wav)
- AAC (audio/aac)
- OGG (audio/ogg)
- FLAC (audio/flac)

**视频格式：**
- MP4 (video/mp4)
- MOV (video/quicktime)
- AVI (video/x-msvideo)
- MKV (video/x-matroska)
- WebM (video/webm)

### 11.2 文件大小限制

- 单个文件最大: 500MB
- 总存储空间: 按用户配额管理（未来扩展）

### 11.3 上传限制

- 单次批量上传: 最多 10 个文件
- 并发上传: 最多 3 个文件同时上传

### 11.4 文件命名规则

- 服务器存储: UUID + 扩展名（如 `a1b2c3d4-e5f6.mp3`）
- 原始文件名: 保存在 `original_filename` 字段
- 下载时使用原始文件名

---

## 十二、开发里程碑

### Phase 1: 后端基础（1-2 周）
- [ ] 数据库 Migration（添加 file_version 和 original_filename 字段）
- [ ] Book Model + API
- [ ] Media Model + API（包含永久 access_token 生成）
- [ ] 文件上传服务
- [ ] 文件替换服务（保持访问链接不变）
- [ ] 二维码生成服务

### Phase 2: 核心功能（1-2 周）
- [ ] 公开访问 API（基于 access_token）
- [ ] 流媒体播放（Range 请求支持）
- [ ] 权限控制
- [ ] Worker 后台任务
- [ ] 文件替换原子性保证

### Phase 3: 前端开发（2-3 周）
- [ ] 项目初始化（Vite + React）
- [ ] 路由和布局
- [ ] 书籍管理页面
- [ ] 上传页面
- [ ] 文件替换功能 UI
- [ ] 播放页面
- [ ] 版本历史展示

### Phase 4: 优化和测试（1 周）
- [ ] 性能优化
- [ ] 错误处理
- [ ] 单元测试
- [ ] 集成测试

---

## 十二、配置文件调整

### config/development.yaml

```yaml
server:
  middlewares:
    static:
      enable: true
      must_exist: false
      folder:
        uri: "/"
        path: "assets/static"
      fallback: "assets/static/index.html"
    # 添加文件上传限制
    limit_payload:
      enable: true
      body_limit: 524288000  # 500MB

# 应用配置
settings:
  upload:
    max_file_size: 524288000  # 500MB
    allowed_types: ["audio/mpeg", "audio/mp4", "video/mp4", "video/quicktime"]
    storage_path: "uploads"
  media:
    base_url: "http://localhost:5150"
```

---

## 附录

### A. 数据库索引建议

```sql
-- Books
CREATE INDEX idx_books_user_id ON books(user_id);
CREATE INDEX idx_books_parent_id ON books(parent_id);

-- Chapters
CREATE INDEX idx_chapters_book_id ON chapters(book_id);

-- Media
CREATE INDEX idx_media_book_id ON media(book_id);
CREATE INDEX idx_media_chapter_id ON media(chapter_id);
CREATE INDEX idx_media_user_id ON media(user_id);
CREATE UNIQUE INDEX idx_media_access_token ON media(access_token);
CREATE INDEX idx_media_is_public ON media(is_public);
```

### B. API 响应格式规范

```json
// 成功响应
{
  "data": { ... },
  "message": "操作成功"
}

// 错误响应
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "参数验证失败",
    "details": [
      {
        "field": "title",
        "message": "标题不能为空"
      }
    ]
  }
}

// 分页响应
{
  "data": [...],
  "pagination": {
    "page": 1,
    "per_page": 20,
    "total": 100,
    "total_pages": 5
  }
}
```

---

**文档版本**: v1.1
**创建日期**: 2025-10-06
**最后更新**: 2025-10-06
**作者**: Claude & pingfury

---

## 变更历史

### v1.1 (2025-10-06)
- **新增**: Media 表添加 `file_version` 和 `original_filename` 字段
- **新增**: 文件替换功能（PUT /api/media/:id/replace-file）
- **新增**: 文件版本历史 API（GET /api/media/:id/history）
- **修改**: `access_token` 设计为永久不变，支持文件替换但链接不变
- **新增**: 文件替换的原子性解决方案
- **新增**: 二维码永久有效说明

### v1.0 (2025-10-06)
- 初始版本
- 基础数据模型设计
- 核心 API 设计
- 前后端技术选型
