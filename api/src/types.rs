use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// 配置结构
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub blessing_skin: BlessingSkinConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BlessingSkinConfig {
    pub base_url: String,
    pub cookies: Vec<String>,
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
            version: "6.0.2".to_string(),
            environment: "production".to_string(),
            debug: false,
            commit: "".to_string(),
            laravel_version: "8.75.0".to_string(),
            php_version: "8.0.26".to_string(),
            web_server: "nginx/1.26.3".to_string(),
            os: "Linux 6.1.0-10-amd64 x86_64".to_string(),
            db_type: "MySQL/MariaDB".to_string(),
            db_host: "121.62.29.115".to_string(),
            db_port: "3306".to_string(),
            db_user: "ser991526191534".to_string(),
            db_name: "ser991526191534".to_string(),
            plugins: vec![Plugin {
                name: "Yggdrasil API".to_string(),
                version: "5.2.1".to_string(),
            }],
        }
    }
}

impl Default for DashboardData {
    fn default() -> Self {
        Self {
            users_count: 45,
            players_count: 48,
            textures_count: 25,
            storage_size: "296KB".to_string(),
            site_name: "byd Yealqp Skin".to_string(),
            version: "6.0.2".to_string(),
            locale: "zh_CN".to_string(),
            base_url: "https://skin.yealqp.fun".to_string(),
        }
    }
} 