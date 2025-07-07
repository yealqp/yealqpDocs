use async_ssh2_tokio::client::{Client, AuthMethod, ServerCheckMethod};

pub async fn get_cpu_mem_usage(
    host: &str,
    port: u16,
    username: &str,
    password: &str,
) -> Result<(String, String), async_ssh2_tokio::Error> {
    let mut client = Client::connect(
        (host, port),
        username,
        AuthMethod::with_password(password),
        ServerCheckMethod::NoCheck,
    ).await?;

    // 获取CPU占用
    let cpu_result = client.execute("top -bn2 | grep 'Cpu(s)'").await?;
    // 获取内存占用
    let mem_result = client.execute("free -m").await?;

    Ok((cpu_result.stdout, mem_result.stdout))
} 