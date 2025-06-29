use axum::{
    extract::State,
    http::Method,
    response::Json,
    routing::get,
    Router,
};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing::{info, error};

mod scraper;
mod types;

use scraper::BlessingSkinScraper;
use types::{ApiResponse, SkinStatusData, DashboardData, HealthResponse, Config};
use chrono::Utc;

#[derive(Clone)]
struct AppState {
    scraper: Arc<BlessingSkinScraper>,
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

    // 创建 scraper 实例
    let scraper = Arc::new(BlessingSkinScraper::new(config.clone()));

    // 创建应用状态
    let state = AppState { scraper };

    // 配置 CORS
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_origin(Any)
        .allow_headers(Any);

    // 创建路由
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/status", get(get_status))
        .route("/api/dashboard", get(get_dashboard))
        .layer(cors)
        .with_state(state);

    // 启动服务器
    let addr = format!("{}:{}", config.server.host, config.server.port);
    info!("启动服务器在 {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> Json<ApiResponse<HealthResponse>> {
    let response = HealthResponse {
        status: "healthy".to_string(),
        service: "blessing-skin-status-api".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: Utc::now(),
    };

    Json(ApiResponse {
        success: true,
        data: Some(response),
        error: None,
        message: Some("服务运行正常".to_string()),
        timestamp: Utc::now(),
    })
}

async fn get_status(State(state): State<AppState>) -> Json<ApiResponse<SkinStatusData>> {
    match state.scraper.fetch_status().await {
        Ok(data) => {
            info!("成功获取状态数据");
            Json(ApiResponse {
                success: true,
                data: Some(data),
                error: None,
                message: Some("成功获取 Blessing Skin 状态".to_string()),
                timestamp: Utc::now(),
            })
        }
        Err(e) => {
            error!("获取状态数据失败: {}", e);
            Json(ApiResponse {
                success: false,
                data: None,
                error: Some(e.to_string()),
                message: Some("获取状态数据失败".to_string()),
                timestamp: Utc::now(),
            })
        }
    }
}

async fn get_dashboard(State(state): State<AppState>) -> Json<ApiResponse<DashboardData>> {
    match state.scraper.fetch_dashboard().await {
        Ok(data) => {
            info!("成功获取仪表盘数据");
            Json(ApiResponse {
                success: true,
                data: Some(data),
                error: None,
                message: Some("成功获取 Blessing Skin 仪表盘数据".to_string()),
                timestamp: Utc::now(),
            })
        }
        Err(e) => {
            error!("获取仪表盘数据失败: {}", e);
            Json(ApiResponse {
                success: false,
                data: None,
                error: Some(e.to_string()),
                message: Some("获取仪表盘数据失败".to_string()),
                timestamp: Utc::now(),
            })
        }
    }
} 