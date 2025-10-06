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
    file_path VARCHAR(512) NOT NULL, -- 存储路径
    file_size BIGINT,
    duration INTEGER,                -- 时长（秒）
    mime_type VARCHAR(100),
    access_token VARCHAR(64) UNIQUE NOT NULL, -- 公开访问唯一标识
    access_url VARCHAR(512),         -- 完整访问链接
    qr_code_path VARCHAR(512),       -- 二维码图片路径
    play_count INTEGER DEFAULT 0,    -- 播放次数
    is_public BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP,
    updated_at TIMESTAMP,
    FOREIGN KEY (book_id) REFERENCES books(id) ON DELETE CASCADE,
    FOREIGN KEY (chapter_id) REFERENCES chapters(id) ON DELETE SET NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);
```

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
GET    /api/media/:id                 获取媒体详情
PUT    /api/media/:id                 更新媒体信息
DELETE /api/media/:id                 删除媒体
GET    /api/media/:id/qrcode          获取二维码图片
POST   /api/media/:id/regenerate-qr   重新生成二维码
POST   /api/media/:id/publish         发布/取消发布
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
  "is_public": false,
  "created_at": "2025-10-06T12:00:00Z"
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
├── UI 框架: Ant Design 或 shadcn/ui
├── 路由: React Router v6
├── 文件上传: react-dropzone
├── 播放器:
│   ├── 音频: Plyr
│   └── 视频: video.js
├── 二维码展示: qrcode.react
└── HTTP 客户端: axios
```

---

## 五、前端页面设计

### 5.1 页面结构

```
/                       首页（公开书籍展示）
/login                  登录页
/register               注册页

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

#### 书籍管理页面（/dashboard/books）

```
┌────────────────────────────────────────────────┐
│  我的书籍                          [+ 新建书籍] │
├────────────────────────────────────────────────┤
│  🔍 搜索...                                    │
├────────────────────────────────────────────────┤
│  📚 Podcast 系列 1              [编辑] [删除]  │
│    ├── 📖 Season 1                             │
│    │   ├── 🎵 Episode 1  [👁️ 公开] [QR]       │
│    │   └── 🎵 Episode 2  [🔒 私密] [QR]       │
│    └── 📖 Season 2                             │
│        └── 🎵 Episode 3  [👁️ 公开] [QR]       │
│                                                 │
│  📚 有声书                      [编辑] [删除]  │
│    ├── 🎵 第一章            [👁️ 公开] [QR]     │
│    └── 🎵 第二章            [🔒 私密] [QR]     │
└────────────────────────────────────────────────┘
```

#### 上传页面（/dashboard/upload）

```
┌────────────────────────────────────────────────┐
│  上传媒体文件                                  │
├────────────────────────────────────────────────┤
│  选择归属书籍: [下拉选择]                     │
│  选择归属章节: [下拉选择] (可选)              │
├────────────────────────────────────────────────┤
│  ┌──────────────────────────────────────────┐ │
│  │   拖拽文件到此处                         │ │
│  │   或点击选择文件                         │ │
│  │   支持 MP3, MP4, WAV, M4A 等格式        │ │
│  └──────────────────────────────────────────┘ │
├────────────────────────────────────────────────┤
│  上传队列:                                     │
│  ✓ episode1.mp3  (100%)                       │
│  ⏳ episode2.mp3  (45%)                        │
│  ⏸️ episode3.mp4  (等待中)                     │
└────────────────────────────────────────────────┘
```

#### 公开访问页面（/public/:token）

```
┌────────────────────────────────────┐
│   ┌──────────────────────────┐     │
│   │     [封面图/缩略图]      │     │
│   └──────────────────────────┘     │
│                                     │
│   标题: 第一集                      │
│   描述: 这是第一集的内容            │
│   时长: 20:00                       │
│   播放次数: 123                     │
│                                     │
│   ━━━━━━━━●─────────── 12:34/20:00 │
│   [⏮️] [⏯️] [⏭️]  [🔊] [⚙️] [全屏] │
│                                     │
│   ┌──────────────┐                 │
│   │              │                 │
│   │   QR Code    │                 │
│   │              │                 │
│   └──────────────┘                 │
│                                     │
│   [📋 复制链接] [📱 分享] [⬇️ 下载] │
└────────────────────────────────────┘
```

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
   ├── 生成访问 token
   ├── 生成访问 URL
   └── 保存到数据库
4. 后台任务（Worker）
   ├── 生成二维码
   ├── 生成缩略图（视频）
   └── 更新数据库
5. 返回响应给前端
```

### 7.2 二维码生成

```rust
use qrcode::{QrCode, render::svg};
use image::Luma;

// 生成二维码
let code = QrCode::new(access_url)?;
let image = code.render::<Luma<u8>>().build();
image.save(qr_code_path)?;
```

### 7.3 流媒体传输（支持 Range 请求）

```rust
// 使用 tower-http 提供文件服务
use tower_http::services::ServeFile;

// 支持 Range 请求，实现断点续传和拖动播放
// 响应头: Accept-Ranges: bytes
// 请求头: Range: bytes=0-1023
```

### 7.4 权限控制

```rust
// 访问控制逻辑
pub async fn can_access_media(
    db: &DatabaseConnection,
    media_id: i32,
    user_id: Option<i32>,
) -> Result<bool> {
    let media = Media::find_by_id(db, media_id).await?;

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
2. 数据库 Migration（books, media 表）
3. Book CRUD API
4. Media 上传功能
5. 二维码生成服务
6. 公开访问 API
7. 流媒体播放
8. 前端 UI 开发

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

---

## 十一、开发里程碑

### Phase 1: 后端基础（1-2 周）
- [ ] 数据库 Migration
- [ ] Book Model + API
- [ ] Media Model + API
- [ ] 文件上传服务
- [ ] 二维码生成服务

### Phase 2: 核心功能（1-2 周）
- [ ] 公开访问 API
- [ ] 流媒体播放
- [ ] 权限控制
- [ ] Worker 后台任务

### Phase 3: 前端开发（2-3 周）
- [ ] 项目初始化（Vite + React）
- [ ] 路由和布局
- [ ] 书籍管理页面
- [ ] 上传页面
- [ ] 播放页面

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

**文档版本**: v1.0
**创建日期**: 2025-10-06
**最后更新**: 2025-10-06
**作者**: Claude & pingfury
