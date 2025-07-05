use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

/// 自定义错误类型
#[derive(Error, Debug)]
pub enum AppError {
    #[error("配置错误: {0}")]
    ConfigError(String),
    
    #[error("数据库错误: {0}")]
    DatabaseError(String),
    
    #[error("解析错误: {0}")]
    ParseError(String),
    
    #[error("内部服务器错误: {0}")]
    InternalError(String),
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::InternalError(err.to_string())
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::ConfigError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::ParseError(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg),
            AppError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(json!({
            "success": false,
            "error": error_message,
            "message": "操作失败",
            "timestamp": chrono::Utc::now()
        }));

        (status, body).into_response()
    }
} 