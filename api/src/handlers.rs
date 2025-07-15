use axum::{
    extract::State,
    response::Json,
};
use tracing::{info, error};
use chrono::Utc;
use hmac::{Hmac, Mac};
use sha1::{Sha1, Digest};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use hex;

use crate::types::{ApiResponse, DashboardData, HealthResponse, Config, CosS3Config};
use crate::AppState;
use crate::utils::{create_success_response, create_error_response};
use axum::extract::Query;
use serde::Deserialize;

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

fn hmac_sha1(key: &str, msg: &str) -> String {
    let mut mac = Hmac::<Sha1>::new_from_slice(key.as_bytes()).unwrap();
    mac.update(msg.as_bytes());
    hex::encode(mac.finalize().into_bytes())
}

fn gen_cos_signature(
    secret_id: &str,
    secret_key: &str,
    method: &str,
    path: &str,
    headers: &[(&str, &str)],
    params: &[(&str, &str)],
    sign_time: &str,
) -> String {
    let http_string = format!(
        "{}\n{}\n{}\n{}\n",
        method.to_lowercase(),
        path,
        params.iter().map(|(k, v)| format!("{}={}", k, v)).collect::<Vec<_>>().join("&"),
        headers.iter().map(|(k, v)| format!("{}={}", k.to_lowercase(), v)).collect::<Vec<_>>().join("&")
    );
    let http_string_sha1 = format!("{:x}", Sha1::digest(http_string.as_bytes()));
    let sign_key = hmac_sha1(secret_key, sign_time);
    let string_to_sign = format!("sha1\n{}\n{}\n", sign_time, http_string_sha1);
    let signature = hmac_sha1(&sign_key, &string_to_sign);
    format!(
        "q-sign-algorithm=sha1&q-ak={}&q-sign-time={}&q-key-time={}&q-header-list=host&q-url-param-list=list-type&q-signature={}",
        secret_id, sign_time, sign_time, signature
    )
}

pub async fn cos_size() -> Json<serde_json::Value> {
    use reqwest::Client;
    // 读取配置
    let config: crate::types::Config = config::Config::builder()
        .add_source(config::File::with_name("config"))
        .build()
        .and_then(|c| c.try_deserialize())
        .unwrap();
    let cos = match config.cos_s3 {
        Some(c) => c,
        None => return Json(serde_json::json!({"success": false, "error": "No cos_s3 config"})),
    };
    let now = Utc::now().timestamp();
    let sign_time = format!("{};{}", now, now + 600); // 10分钟有效
    let host = format!("{}.cos.{}.myqcloud.com", cos.bucket, cos.region);
    let url = format!("https://{}/?list-type=2", host);
    let authorization = gen_cos_signature(
        &cos.access_key,
        &cos.secret_key,
        "GET",
        "/",
        &[("host", &host)],
        &[("list-type", "2")],
        &sign_time,
    );
    let client = Client::new();
    let resp = match client
        .get(&url)
        .header("Host", &host)
        .header("Authorization", authorization)
        .send()
        .await {
        Ok(r) => r,
        Err(e) => return Json(serde_json::json!({"success": false, "error": format!("{}", e)})),
    };
    let text = match resp.text().await {
        Ok(t) => t,
        Err(e) => return Json(serde_json::json!({"success": false, "error": format!("{}", e)})),
    };
    // 解析XML，累加<Size>字段
    let mut total_bytes = 0u64;
    let mut start = 0;
    while let Some(s) = text[start..].find("<Size>") {
        let s = s + start + 6;
        if let Some(e) = text[s..].find("</Size>") {
            let e = s + e;
            if let Ok(sz) = text[s..e].trim().parse::<u64>() {
                total_bytes += sz;
            }
            start = e + 7;
        } else {
            break;
        }
    }
    let gib = total_bytes as f64 / 1073749999.9999964;
    Json(serde_json::json!({"success": true, "size_gib": format!("{:.2}", gib)}))
} 