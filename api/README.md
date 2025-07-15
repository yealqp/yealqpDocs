# Blessing Skin Dashboard API

这是一个用于获取 Blessing Skin 仪表盘数据的 API 服务。

## 功能特性

- 📊 获取仪表盘统计数据
- 🏥 健康检查端点
- 🔒 自定义错误处理
- 📝 结构化日志记录
- 🌐 CORS 支持

## 项目结构

```
src/
├── main.rs          # 应用程序入口点
├── types.rs         # 数据结构定义
├── database.rs      # 数据库查询逻辑
├── handlers.rs      # HTTP 路由处理器
├── error.rs         # 自定义错误类型
├── database.rs      # 数据库查询逻辑
└── utils.rs         # 工具函数
```

## 模块说明

### main.rs
- 应用程序配置和启动
- 路由设置
- CORS 配置
- 状态管理

### types.rs
- API 响应结构
- 配置数据结构
- 业务数据模型

### database.rs
- MySQL 数据库连接
- 数据查询逻辑
- 统计信息计算

### handlers.rs
- HTTP 端点处理器
- 响应格式化
- 错误处理

### error.rs
- 自定义错误类型
- 错误转换实现
- HTTP 响应映射

### utils.rs
- 通用工具函数
- 响应创建助手
- 文本处理函数

## 配置

编辑 `config.toml` 文件：

```toml
[server]
# backend_host
host = "0.0.0.0"
port = 5000

[database]
# blessingskin_database_host
url = "mysql://root:password@example.com:3389/database_name" 

[cos_s3]
bucket = ""
region = ""
endpoint = ""
access_key = ""
secret_key = ""

```

## API 端点

### GET /health
健康检查端点

### GET /api/dashboard
获取仪表盘统计数据

### GET /api/cos_size
获取COS存储空间使用情况

## 运行

```bash
cargo run
```

## 构建

```bash
cargo build --release
```

## 重构改进

1. **模块化设计**: 将代码分离到不同的模块中，提高可维护性
2. **错误处理**: 使用自定义错误类型和统一的错误处理
3. **代码复用**: 提取通用工具函数，减少重复代码
4. **类型安全**: 改进类型定义和错误处理
5. **文档化**: 添加详细的代码注释和文档 