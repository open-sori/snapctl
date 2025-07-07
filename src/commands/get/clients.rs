use crate::rpc::client::SnapcastRpcClient;
use crate::utils::display::print_table;
use anyhow::Result;

pub async fn get_clients(server_url: &str) -> Result<()> {
    let client = SnapcastRpcClient::new(server_url);
    let server_info = client.get_status().await?;

    let headers = vec!["CLIENT ID", "STATUS", "GROUP ID", "STREAM ID"];
    let mut data = Vec::new();

    // Get groups array
    let groups = if let Some(groups) = server_info.get("groups").and_then(|g| g.as_array()) {
        groups
    } else {
        println!("No clients found (no groups available).");
        return Ok(());
    };

    // Process each group to find clients
    for group in groups {
        let group_id = group.get("id")
            .and_then(|id| id.as_str())
            .unwrap_or("unknown").to_string();

        let stream_id = group.get("stream_id")
            .and_then(|id| id.as_str())
            .unwrap_or("unknown").to_string();

        if let Some(clients) = group.get("clients").and_then(|c| c.as_array()) {
            for client in clients {
                let client_id = client.get("id")
                    .and_then(|id| id.as_str())
                    .unwrap_or("unknown").to_string();

                let connected = client.get("connected")
                    .and_then(|c| c.as_bool())
                    .unwrap_or(false);

                let status = if connected { "connected" } else { "disconnected" };

                data.push(vec![client_id, status.to_string(), group_id.clone(), stream_id.clone()]);
            }
        }
    }

    print_table(headers, data);

    Ok(())
}