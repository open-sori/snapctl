use crate::rpc::client::SnapcastRpcClient;
use anyhow::Result;

pub async fn get_streams(server_url: &str) -> Result<()> {
    let client = SnapcastRpcClient::new(server_url);
    let server_info = client.get_status().await?;

    println!("{:<20} {:<10}", "STREAM ID", "STATUS");

    if let Some(streams) = server_info["streams"].as_array() {
        for stream in streams {
            let id = stream["id"].as_str().unwrap_or("unknown");
            let status = stream["status"].as_str().unwrap_or("unknown");
            println!("{:<20} {:<10}", id, status);
        }
    }

    Ok(())
}