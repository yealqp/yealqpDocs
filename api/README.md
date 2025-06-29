# Blessing Skin Status API

这是一个用 Rust 编写的 Blessing Skin 状态监控 API，用于获取 Blessing Skin 站点的运行状态和仪表盘数据。

## 功能特性

- 获取 Blessing Skin 站点运行状态
- 获取仪表盘统计数据
- 支持配置文件管理
- 自动刷新数据
- 健康检查端点

## 配置说明

### 配置文件

项目使用 `config.toml` 文件进行配置：

```toml
[server]
host = "0.0.0.0"
port = 5000

[blessing_skin]
base_url = "https://skin.yealqp.fun"
cookies = [
    "cookie1=value1",
    "cookie2=value2"
]
```

### 配置项说明

- `server.host`: 服务器监听地址
- `server.port`: 服务器监听端口
- `blessing_skin.base_url`: Blessing Skin 站点地址
- `blessing_skin.cookies`: 用于访问管理页面的 Cookie 列表

### 获取 Cookie

1. 使用浏览器登录 Blessing Skin 管理面板
2. 打开开发者工具 (F12)
3. 在 Network 标签页中找到对管理页面的请求
4. 复制请求头中的 Cookie 值
5. 将 Cookie 字符串按分号分割，添加到配置文件中

## API 端点

### 健康检查
```
GET /health
```

### 获取状态
```
GET /api/status
```

### 获取仪表盘数据
```
GET /api/dashboard
```

## 构建和运行

### 开发环境
```bash
cargo run
```

### 生产环境
```bash
cargo build --release
./target/release/blessing-skin-status-api
```

### 交叉编译 (Linux x86_64)
```bash
# 添加目标
rustup target add x86_64-unknown-linux-gnu

# 编译
cargo build --release --target x86_64-unknown-linux-gnu
```

## 部署

1. 将编译好的可执行文件和 `config.toml` 配置文件复制到服务器
2. 确保配置文件中的 Cookie 是最新的
3. 运行可执行文件

## 注意事项

- Cookie 会定期过期，需要及时更新配置文件
- 建议使用 systemd 或 supervisor 等工具管理服务
- 生产环境建议使用反向代理 (如 nginx) 提供 HTTPS 支持 