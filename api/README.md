# Blessing Skin Status API

这是一个用于获取 Blessing Skin 服务器状态和仪表盘数据的 API 服务。

## 功能特性

- 🔍 抓取 Blessing Skin 服务器状态信息
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
host = "127.0.0.1"
port = 3000

[database]
url = "mysql://username:password@localhost:3306/blessingskin"
```

**注意**: 请将 `username`、`password`、`localhost`、`3306` 替换为你的实际 MySQL 连接信息。

## API 端点

### GET /health
健康检查端点

### GET /api/status
获取 Blessing Skin 服务器状态

### GET /api/dashboard
获取仪表盘统计数据

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