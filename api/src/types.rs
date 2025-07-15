use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// 配置结构
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub cos_s3: Option<CosS3Config>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CosS3Config {
    pub bucket: String,
    pub region: String,
    pub endpoint: String,
    pub access_key: String,
    pub secret_key: String,
}

/// API 响应结构
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub message: Option<String>,
    pub timestamp: DateTime<Utc>,
}

/// 仪表盘数据结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DashboardData {
    pub users_count: i32,
    pub players_count: i32,
    pub textures_count: i32,
    pub storage_size: String,
    pub site_name: String,
    pub version: String,
    pub locale: String,
    pub base_url: String,
}

/// 插件信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Plugin {
    pub name: String,
    pub version: String,
}

/// 健康检查响应
#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub version: String,
    pub timestamp: DateTime<Utc>,
}

impl Default for DashboardData {
    fn default() -> Self {
        Self {
            users_count: 0,
            players_count: 0,
            textures_count: 0,
            storage_size: String::new(),
            site_name: String::new(),
            version: String::new(),
            locale: String::new(),
            base_url: String::new(),
        }
    }
} 