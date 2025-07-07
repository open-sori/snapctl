use crate::utils::display::print_table;
use serde_json::json;
use anyhow::{Result, anyhow};
use crate::rpc::client::SnapcastRpcClient;
use uuid::Uuid;

/// Deletes a client from the server.
///
/// # Arguments
///
/// * `server_url` - The URL of the server.
/// * `client_id` - The ID of the client to delete.
///
/// # Returns
///
/// A `Result` indicating success or failure.
pub async fn delete_client(server_url: &str, client_id: &str) -> Result<()> {
    let client = SnapcastRpcClient::new(server_url);

    let client_status_message = json!({
        "id": Uuid::new_v4().to_string(),
        "jsonrpc": "2.0",
        "method": "Client.GetStatus",
        "params": {
            "id": client_id
        }
    });

    let client_status_response = client.send_rpc_message(client_status_message).await?;

    if let Some(error) = client_status_response.get("error") {
        if let Some(code) = error.get("code").and_then(|c| c.as_i64()) {
            if code == -32603 {
                if let Some(message) = error.get("message").and_then(|m| m.as_str()) {
                    return Err(anyhow!("Client not found: {}", message));
                }
                return Err(anyhow!("Client not found"));
            }
        }
    }

    let delete_message = json!({
        "id": Uuid::new_v4().to_string(),
        "jsonrpc": "2.0",
        "method": "Server.DeleteClient",
        "params": {
            "id": client_id
        }
    });

    let _delete_response = client.send_rpc_message(delete_message).await?;

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
                let current_client_id = client.get("id")
                    .and_then(|id| id.as_str())
                    .unwrap_or("unknown").to_string();

                let connected = client.get("connected")
                    .and_then(|c| c.as_bool())
                    .unwrap_or(false);

                let status = if connected { "connected" } else { "disconnected" };

                data.push(vec![current_client_id, status.to_string(), group_id.clone(), stream_id.clone()]);
            }
        }
    }

    print_table(headers, data);

    Ok(())
}