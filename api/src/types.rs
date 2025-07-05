use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// 配置结构
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
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

/// API 响应结构
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub message: Option<String>,
    pub timestamp: DateTime<Utc>,
}

/// Blessing Skin 状态数据结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SkinStatusData {
    pub version: String,
    pub environment: String,
    pub debug: bool,
    pub commit: String,
    pub laravel_version: String,
    pub php_version: String,
    pub web_server: String,
    pub os: String,
    pub db_type: String,
    pub db_host: String,
    pub db_port: String,
    pub db_user: String,
    pub db_name: String,
    pub plugins: Vec<Plugin>,
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

impl Default for SkinStatusData {
    fn default() -> Self {
        Self {
            version: String::new(),
            environment: String::new(),
            debug: false,
            commit: String::new(),
            laravel_version: String::new(),
            php_version: String::new(),
            web_server: String::new(),
            os: String::new(),
            db_type: String::new(),
            db_host: String::new(),
            db_port: String::new(),
            db_user: String::new(),
            db_name: String::new(),
            plugins: Vec::new(),
        }
    }
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