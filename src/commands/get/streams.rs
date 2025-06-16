use crate::rpc::client::SnapcastRpcClient;
use crate::utils::display::print_table;
use anyhow::Result;

pub async fn get_streams(server_url: &str) -> Result<()> {
    let client = SnapcastRpcClient::new(server_url);
    let server_info = client.get_status().await?;

    let headers = vec!["STREAM ID", "STATUS"];
    let mut data = Vec::new();

    if let Some(streams) = server_info["streams"].as_array() {
        for stream in streams {
            let id = stream["id"].as_str().unwrap_or("unknown").to_string();
            let status = stream["status"].as_str().unwrap_or("unknown").to_string();
            data.push(vec![id, status]);
        }
    }

    print_table(headers, data);

    Ok(())
}