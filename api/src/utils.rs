use chrono::Utc;
use crate::types::ApiResponse;

/// 创建成功响应
pub fn create_success_response<T>(data: T, message: &str) -> ApiResponse<T> {
    ApiResponse {
        success: true,
        data: Some(data),
        error: None,
        message: Some(message.to_string()),
        timestamp: Utc::now(),
    }
}

/// 创建错误响应
pub fn create_error_response<T>(error: &str, message: &str) -> ApiResponse<T> {
    ApiResponse {
        success: false,
        data: None,
        error: Some(error.to_string()),
        message: Some(message.to_string()),
        timestamp: Utc::now(),
    }
}

/// 验证 URL 格式
pub fn is_valid_url(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://")
}

/// 清理 HTML 文本内容
pub fn clean_html_text(text: &str) -> String {
    text.trim()
        .replace('\n', " ")
        .replace('\r', " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
} 