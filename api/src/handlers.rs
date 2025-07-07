use axum::{
    extract::State,
    response::Json,
};
use tracing::{info, error};
use chrono::Utc;

use crate::types::{ApiResponse, DashboardData, HealthResponse, SshInfo, SshHost, Config};
use crate::AppState;
use crate::utils::{create_success_response, create_error_response};
use crate::ssh_query::get_cpu_mem_usage;
use axum::extract::Query;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SshQuery {
    pub host: String,
    pub port: Option<u16>,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct SshStatusQuery {
    pub api: Option<String>,
}

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

pub async fn ssh_status(Query(q): Query<SshStatusQuery>, State(_state): State<AppState>) -> Json<serde_json::Value> {
    // 读取配置
    let config: Config = config::Config::builder()
        .add_source(config::File::with_name("config"))
        .build()
        .and_then(|c| c.try_deserialize())
        .unwrap();
    let mut results = vec![];
    if let Some(ssh_info) = config.ssh_info {
        for host in ssh_info.hosts {
            // 只根据api字段精确匹配
            if let Some(ref api_key) = q.api {
                if host.api.as_deref() != Some(api_key) {
                    continue;
                }
            }
            let port = host.port.unwrap_or(22);
            let r = get_cpu_mem_usage(&host.host, port, &host.username, &host.password).await;
            match r {
                Ok((cpu, mem)) => results.push(serde_json::json!({
                    "api": host.api,
                    "host": host.host,
                    "cpu": cpu,
                    "mem": mem,
                    "success": true
                })),
                Err(e) => results.push(serde_json::json!({
                    "api": host.api,
                    "host": host.host,
                    "error": format!("{}", e),
                    "success": false
                })),
            }
            // 只查一个
            if q.api.is_some() { break; }
        }
    }
    Json(serde_json::json!({ "success": true, "data": results }))
} 