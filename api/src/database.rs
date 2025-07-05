
use sqlx::mysql::MySqlPool;
use crate::types::{DashboardData, Plugin};
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