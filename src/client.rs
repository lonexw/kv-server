use anyhow::Result;
use kv_server::{start_client_with_config, CommandRequest, ClientConfig};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    // 生成一个 HSET 命令
    let cmd = CommandRequest::new_hset("table1", "hello", "world".to_string().into());
    
    let config: ClientConfig = toml::from_str(include_str!("../fixtures/client.conf"))?;
    let mut client = start_client_with_config(&config).await?;

    // 发送 HSET 命令
    let data = client.execute(cmd).await?;
    info!("Got response {:?}", data);

    Ok(())
}