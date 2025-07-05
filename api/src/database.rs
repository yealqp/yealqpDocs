
use sqlx::mysql::MySqlPool;
use crate::types::{DashboardData, SkinStatusData, Plugin};
use crate::error::AppError;
use tracing::info;

/// 数据库连接管理器
pub struct Database {
    pub pool: MySqlPool,
}

impl Database {
    /// 创建新的数据库连接
    pub async fn new(database_url: &str) -> Result<Self, AppError> {
        let pool = MySqlPool::connect(database_url)
            .await
            .map_err(|e| AppError::DatabaseError(format!("数据库连接失败: {}", e)))?;
        
        info!("数据库连接成功");
        Ok(Self { pool })
    }

    /// 获取仪表盘数据
    pub async fn fetch_dashboard(&self) -> Result<DashboardData, AppError> {
        info!("开始查询仪表盘数据...");

        // 查询用户数量
        let users_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(format!("查询用户数量失败: {}", e)))?;

        // 查询角色数量
        let players_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM players")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(format!("查询角色数量失败: {}", e)))?;

        // 查询皮肤数量
        let textures_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM textures")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(format!("查询皮肤数量失败: {}", e)))?;

        // 查询存储空间大小
        let storage_size: (Option<String>,) = sqlx::query_as("SELECT CAST(SUM(size) AS CHAR) FROM textures")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(format!("查询存储空间失败: {}", e)))?;

        let storage_size_kb = storage_size.0
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0) as i64;
        let storage_size_str = if storage_size_kb > 1024 {
            format!("{:.1}MB", storage_size_kb as f64 / 1024.0)
        } else {
            format!("{}KB", storage_size_kb)
        };

        let data = DashboardData {
            users_count: users_count.0 as i32,
            players_count: players_count.0 as i32,
            textures_count: textures_count.0 as i32,
            storage_size: storage_size_str,
            site_name: "Blessing Skin".to_string(),
            version: "6.0.2".to_string(),
            locale: "zh_CN".to_string(),
            base_url: "https://skin.example.com".to_string(),
        };

        info!("成功查询仪表盘数据: 用户={}, 角色={}, 皮肤={}, 存储={}", 
              data.users_count, data.players_count, data.textures_count, data.storage_size);
        
        Ok(data)
    }

    /// 获取服务器状态数据
    pub async fn fetch_status(&self) -> Result<SkinStatusData, AppError> {
        info!("开始查询服务器状态数据...");

        // 这里可以根据需要查询数据库中的配置信息
        // 或者返回默认的系统信息
        let data = SkinStatusData {
            version: "6.0.2".to_string(),
            environment: "production".to_string(),
            debug: false,
            commit: "".to_string(),
            laravel_version: "8.75.0".to_string(),
            php_version: "8.0.26".to_string(),
            web_server: "nginx/1.26.3".to_string(),
            os: "Linux 6.1.0-10-amd64 x86_64".to_string(),
            db_type: "MySQL/MariaDB".to_string(),
            db_host: "localhost".to_string(),
            db_port: "3306".to_string(),
            db_user: "blessingskin".to_string(),
            db_name: "blessingskin".to_string(),
            plugins: vec![Plugin {
                name: "Yggdrasil API".to_string(),
                version: "5.2.1".to_string(),
            }],
        };

        info!("成功查询服务器状态数据");
        Ok(data)
    }

    /// 测试数据库连接
    pub async fn test_connection(&self) -> Result<(), AppError> {
        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(format!("数据库连接测试失败: {}", e)))?;
        
        info!("数据库连接测试成功");
        Ok(())
    }
} 