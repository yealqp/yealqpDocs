use axum::{
    extract::State,
    response::Json,
};
use tracing::{info, error};
use chrono::Utc;

use crate::types::{ApiResponse, SkinStatusData, DashboardData, HealthResponse};
use crate::AppState;
use crate::utils::{create_success_response, create_error_response};

/// 健康检查端点
pub async fn health_check() -> Json<ApiResponse<HealthResponse>> {
    let response = HealthResponse {
        status: "healthy".to_string(),
        service: "blessing-skin-status-api".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: Utc::now(),
    };

    Json(create_success_response(response, "服务运行正常"))
}

/// 获取 Blessing Skin 状态数据
pub async fn get_status(State(state): State<AppState>) -> Json<ApiResponse<SkinStatusData>> {
    match state.database.fetch_status().await {
        Ok(data) => {
            info!("成功获取状态数据");
            Json(create_success_response(data, "成功获取 Blessing Skin 状态"))
        }
        Err(e) => {
            error!("获取状态数据失败: {}", e);
            Json(create_error_response::<SkinStatusData>(&e.to_string(), "获取状态数据失败"))
        }
    }
}

/// 获取 Blessing Skin 仪表盘数据
pub async fn get_dashboard(State(state): State<AppState>) -> Json<ApiResponse<DashboardData>> {
    match state.database.fetch_dashboard().await {
        Ok(data) => {
            info!("成功获取仪表盘数据");
            Json(create_success_response(data, "成功获取 Blessing Skin 仪表盘数据"))
        }
        Err(e) => {
            error!("获取仪表盘数据失败: {}", e);
            Json(create_error_response::<DashboardData>(&e.to_string(), "获取仪表盘数据失败"))
        }
    }
} 