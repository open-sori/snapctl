use serde_json::json;
use anyhow::{Result, anyhow};
use crate::rpc::client::SnapcastRpcClient;
use uuid::Uuid;

pub async fn delete_clients(server_url: &str, client_ids: &str) -> Result<()> {
    let client = SnapcastRpcClient::new(server_url);

    // Split the comma-separated client IDs
    let client_id_list: Vec<&str> = client_ids.split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    if client_id_list.is_empty() {
        return Err(anyhow!("No valid client IDs provided"));
    }

    // First, check if all clients exist
    for client_id in &client_id_list {
        let client_status_message = json!({
            "id": Uuid::new_v4().to_string(),
            "jsonrpc": "2.0",
            "method": "Client.GetStatus",
            "params": {
                "id": client_id
            }
        });

        let client_status_response = client.send_rpc_message(client_status_message).await?;

        // Check if client exists
        if let Some(error) = client_status_response.get("error") {
            if let Some(code) = error.get("code").and_then(|c| c.as_i64()) {
                if code == -32603 {
                    if let Some(message) = error.get("message").and_then(|m| m.as_str()) {
                        return Err(anyhow!("Client not found: {}: {}", client_id, message));
                    }
                    return Err(anyhow!("Client not found: {}", client_id));
                }
            }
        }
    }

    // Delete each client
    for client_id in &client_id_list {
        let delete_message = json!({
            "id": Uuid::new_v4().to_string(),
            "jsonrpc": "2.0",
            "method": "Server.DeleteClient",
            "params": {
                "id": client_id
            }
        });

        let _delete_response = client.send_rpc_message(delete_message).await?;
    }

    // After deletion, get the updated list of clients
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

                let current_client_id = client.get("id")
                    .and_then(|id| id.as_str())
                    .unwrap_or("unknown");

                let connected = client.get("connected")
                    .and_then(|c| c.as_bool())
                    .unwrap_or(false);

                let status = if connected { "connected" } else { "disconnected" };

                println!("{:<16} {:<16} {:<36} {:<36}",
                    current_client_id, status, group_id, stream_id);
            }
        }
    }

    if !clients_found {
        println!("No clients found.");
    }

    Ok(())
}