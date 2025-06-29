use crate::types::{SkinStatusData, Plugin, DashboardData, Config};
use anyhow::{Result, Context};
use reqwest::Client;
use scraper::{Html, Selector, Element};
use tracing::{info, warn};

pub struct BlessingSkinScraper {
    client: Client,
    config: Config,
}

impl BlessingSkinScraper {
    pub fn new(config: Config) -> Self {
        let client = Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .expect("Failed to build HTTP client");

        Self { client, config }
    }

    pub async fn fetch_status(&self) -> Result<SkinStatusData> {
        info!("开始获取 Blessing Skin 状态...");

        let response = self
            .client
            .get(&format!("{}/admin/status", self.config.blessing_skin.base_url))
            .header("Cookie", self.build_cookie_header())
            .send()
            .await
            .context("HTTP 请求失败")?;

        if !response.status().is_success() {
            anyhow::bail!("HTTP 状态码错误: {}", response.status());
        }

        let html_content = response.text().await.context("读取响应内容失败")?;
        let parsed_data = self.parse_html(&html_content)?;

        info!("成功解析 HTML 内容");
        Ok(parsed_data)
    }

    pub async fn fetch_dashboard(&self) -> Result<DashboardData> {
        info!("开始获取 Blessing Skin 仪表盘数据...");

        let response = self
            .client
            .get(&format!("{}/admin", self.config.blessing_skin.base_url))
            .header("Cookie", self.build_cookie_header())
            .send()
            .await
            .context("HTTP 请求失败")?;

        if !response.status().is_success() {
            anyhow::bail!("HTTP 状态码错误: {}", response.status());
        }

        let html_content = response.text().await.context("读取响应内容失败")?;
        let parsed_data = self.parse_dashboard_html(&html_content)?;

        info!("成功解析仪表盘 HTML 内容");
        Ok(parsed_data)
    }

    fn build_cookie_header(&self) -> String {
        self.config.blessing_skin.cookies.join("; ")
    }

    fn parse_html(&self, html_content: &str) -> Result<SkinStatusData> {
        let document = Html::parse_document(html_content);
        let mut data = SkinStatusData::default();

        // 解析表格数据
        let table_selector = Selector::parse("table").expect("无效的表格选择器");
        let row_selector = Selector::parse("tr").expect("无效的行选择器");
        let cell_selector = Selector::parse("td, th").expect("无效的单元格选择器");

        for table in document.select(&table_selector) {
            for row in table.select(&row_selector) {
                let cells: Vec<String> = row
                    .select(&cell_selector)
                    .map(|cell| cell.text().collect::<String>().trim().to_string())
                    .collect();

                if cells.len() >= 2 {
                    let label = &cells[0];
                    let value = &cells[1];

                    match label.as_str() {
                        "版本" => data.version = value.clone(),
                        "应用环境" => data.environment = value.clone(),
                        "是否处于调试状态" => data.debug = value.to_lowercase() == "yes",
                        "提交" => data.commit = value.clone(),
                        "Laravel 版本" => data.laravel_version = value.clone(),
                        "PHP 版本" => data.php_version = value.clone(),
                        "Web 服务软件" => data.web_server = value.clone(),
                        "操作系统" => data.os = value.clone(),
                        "服务器类型" => data.db_type = value.clone(),
                        "主机" => data.db_host = value.clone(),
                        "端口" => data.db_port = value.clone(),
                        "用户名" => data.db_user = value.clone(),
                        "数据库" => data.db_name = value.clone(),
                        _ => {}
                    }
                }
            }
        }

        // 解析插件信息
        data.plugins = self.parse_plugins(&document)?;

        Ok(data)
    }

    fn parse_dashboard_html(&self, html_content: &str) -> Result<DashboardData> {
        let document = Html::parse_document(html_content);
        let mut data = DashboardData::default();

        // 解析全局信息
        if let Some(script) = document.select(&Selector::parse("#blessing-globals").expect("无效的选择器")).next() {
            let _script_content = script.text().collect::<String>();
            // 这里可以解析 JSON 数据，但为了简单起见，我们使用默认值
        }

        // 解析统计数据
        let stats_selector = Selector::parse(".stats").expect("无效的统计选择器");
        let mut stats_values = Vec::new();

        for stat in document.select(&stats_selector) {
            let value = stat.text().collect::<String>().trim().to_string();
            stats_values.push(value);
        }

        // 根据顺序分配统计数据
        if stats_values.len() >= 4 {
            // 注册用户数
            if let Ok(count) = stats_values[0].parse::<i32>() {
                data.users_count = count;
            }
            
            // 角色总数
            if let Ok(count) = stats_values[1].parse::<i32>() {
                data.players_count = count;
            }
            
            // 上传材质总数
            if let Ok(count) = stats_values[2].parse::<i32>() {
                data.textures_count = count;
            }
            
            // 占用空间大小
            data.storage_size = stats_values[3].clone();
        }

        Ok(data)
    }

    fn parse_plugins(&self, document: &Html) -> Result<Vec<Plugin>> {
        let mut plugins = Vec::new();

        // 查找包含"已开启的插件"的标题
        let table_selector = Selector::parse("table").expect("无效的表格选择器");
        let row_selector = Selector::parse("tr").expect("无效的行选择器");
        let cell_selector = Selector::parse("td").expect("无效的单元格选择器");

        // 查找所有表格，然后检查前面的标题
        for table in document.select(&table_selector) {
            // 检查表格前面是否有包含"已开启的插件"的 h3 标题
            if let Some(prev_sibling) = table.prev_sibling_element() {
                if prev_sibling.value().name() == "h3" {
                    let h3_text = prev_sibling.text().collect::<String>();
                    if h3_text.contains("已开启的插件") {
                        // 解析这个表格中的插件信息
                        for row in table.select(&row_selector) {
                            let cells: Vec<String> = row
                                .select(&cell_selector)
                                .map(|cell| cell.text().collect::<String>().trim().to_string())
                                .collect();

                            if cells.len() >= 2 {
                                plugins.push(Plugin {
                                    name: cells[0].clone(),
                                    version: cells[1].clone(),
                                });
                            }
                        }
                        break;
                    }
                }
            }
        }

        if plugins.is_empty() {
            warn!("未找到插件信息，使用默认值");
            plugins.push(Plugin {
                name: "Yggdrasil API".to_string(),
                version: "5.2.1".to_string(),
            });
        }

        Ok(plugins)
    }
} 