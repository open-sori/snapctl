use crate::rpc::client::SnapcastRpcClient;
use anyhow::Result;

pub async fn get_clients(server_url: &str) -> Result<()> {
    let client = SnapcastRpcClient::new(server_url);
    let server_info = client.get_status().await?;

    // Print header with adjusted column widths
    println!("{:<16} {:<16} {:<36} {:<36}", "CLIENT ID", "STATUS", "GROUP ID", "STREAM ID");

    // Get groups array
    let groups = if let Some(groups) = server_info.get("groups").and_then(|g| g.as_array()) {
        groups
    } else {
        println!("No clients found (no groups available).");
        return Ok(());
    };

    let mut clients_found = false;

    // Process each group to find clients
    for group in groups {
        let group_id = group.get("id")
            .and_then(|id| id.as_str())
            .unwrap_or("unknown");

        let stream_id = group.get("stream_id")
            .and_then(|id| id.as_str())
            .unwrap_or("unknown");

        if let Some(clients) = group.get("clients").and_then(|c| c.as_array()) {
            for client in clients {
                clients_found = true;

                let client_id = client.get("id")
                    .and_then(|id| id.as_str())
                    .unwrap_or("unknown");

                let connected = client.get("connected")
                    .and_then(|c| c.as_bool())
                    .unwrap_or(false);

                let status = if connected { "connected" } else { "disconnected" };

                println!("{:<16} {:<16} {:<36} {:<36}",
                    client_id, status, group_id, stream_id);
            }
        }
    }

    if !clients_found {
        println!("No clients found.");
    }

    Ok(())
}