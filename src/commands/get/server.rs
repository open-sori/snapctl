use crate::rpc::client::SnapcastRpcClient;
use anyhow::Result;

pub async fn get_server(server_url: &str) -> Result<()> {
    let client = SnapcastRpcClient::new(server_url);
    let server_info = client.get_status().await?;

    let name = server_info["server"]["snapserver"]["name"]
        .as_str()
        .unwrap_or("unknown");
    let version = server_info["server"]["snapserver"]["version"]
        .as_str()
        .unwrap_or("unknown");

    println!("{:<18} {:<10}", "NAME", "VERSION");
    println!("{:<18} {:<10}", name, version);

    Ok(())
}