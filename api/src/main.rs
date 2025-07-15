use axum::{
    http::Method,
    routing::get,
    Router,
};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

mod database;
mod types;
mod handlers;
mod error;
mod utils;

use database::Database;
use types::Config;
use handlers::{health_check, get_dashboard, cos_size};

#[derive(Clone)]
pub struct AppState {
    database: Arc<Database>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    tracing_subscriber::fmt::init();

    // 加载配置
    let config: Config = config::Config::builder()
        .add_source(config::File::with_name("config"))
        .build()?
        .try_deserialize()?;

    info!("配置加载成功: 服务器地址 {}:{}", config.server.host, config.server.port);

    // 创建数据库连接
    let database = Arc::new(Database::new(&config.database.url).await?);

    // 创建应用状态
    let state = AppState { database };

    // 配置 CORS
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_origin(Any)
        .allow_headers(Any);

    // 创建路由
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/dashboard", get(get_dashboard))
        .route("/api/cos_size", get(cos_size))
        .layer(cors)
        .with_state(state);

    // 启动服务器
    let addr = format!("{}:{}", config.server.host, config.server.port);
    info!("启动服务器在 {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
} 