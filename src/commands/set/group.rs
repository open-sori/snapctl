use crate::rpc::client::SnapcastRpcClient;
use crate::utils::display::print_table;
use anyhow::{Result, anyhow};
use serde_json::json;
use uuid::Uuid;

pub async fn set_group(
    server_url: &str,
    group_id: &str,
    name: Option<String>,
    mute: Option<bool>,
    stream_id: Option<String>,
    clients: Option<String>,
) -> Result<()> {
    let client = SnapcastRpcClient::new(server_url);

    // First, get the current group status to display all information
    let group_status_message = json!({
        "id": Uuid::new_v4().to_string(),
        "jsonrpc": "2.0",
        "method": "Group.GetStatus",
        "params": {
            "id": group_id
        }
    });

    let group_status_response = client.send_rpc_message(group_status_message).await?;

    // Check if group exists
    if let Some(error) = group_status_response.get("error") {
        if let Some(code) = error.get("code").and_then(|c| c.as_i64()) {
            if code == -32603 {
                if let Some(message) = error.get("message").and_then(|m| m.as_str()) {
                    return Err(anyhow!("Group not found: {}", message));
                }
                return Err(anyhow!("Group not found"));
            }
        }
    }

    let mut final_name = None;
    let mut final_muted = None;
    let mut final_stream_id = None;
    let mut final_clients = Vec::new();
    let mut name_was_set = false;
    let mut mute_was_set = false;
    let mut stream_was_set = false;
    let mut clients_was_set = false;

    // Extract information from the group status response
    if let Some(result) = group_status_response.get("result") {
        if let Some(group) = result.get("group") {
            final_name = group.get("name").and_then(|n| n.as_str()).map(|s| s.to_string());
            final_muted = group.get("muted").and_then(|m| m.as_bool());
            final_stream_id = group.get("stream_id").and_then(|s| s.as_str()).map(|s| s.to_string());

            // Extract client information
            if let Some(clients) = group.get("clients").and_then(|c| c.as_array()) {
                for client in clients {
                    if let Some(client_id) = client.get("id").and_then(|id| id.as_str()) {
                        final_clients.push(client_id.to_string());
                    }
                }
            }
        }
    }

    // Handle name settings if provided
    if let Some(name_value) = &name {
        name_was_set = true;

        // Handle special "none" and "null" values for name
        let name_to_set = if name_value.to_lowercase() == "none" || name_value.to_lowercase() == "null" {
            String::new() // Set to empty string for "none" or "null"
        } else {
            name_value.clone()
        };

        let message = json!({
            "id": Uuid::new_v4().to_string(),
            "jsonrpc": "2.0",
            "method": "Group.SetName",
            "params": {
                "id": group_id,
                "name": name_to_set
            }
        });

        let response = client.send_rpc_message(message).await?;

        // Check for errors in the response
        if let Some(error) = response.get("error") {
            if let Some(code) = error.get("code").and_then(|c| c.as_i64()) {
                if code == -32603 {
                    if let Some(message) = error.get("message").and_then(|m| m.as_str()) {
                        return Err(anyhow!("Failed to set group name: {}", message));
                    }
                    return Err(anyhow!("Failed to set group name: Group not found"));
                }
            }
        }

        final_name = Some(name_to_set);
    }

    // Handle mute settings if provided
    if let Some(mute_value) = mute {
        mute_was_set = true;
        let message = json!({
            "id": Uuid::new_v4().to_string(),
            "jsonrpc": "2.0",
            "method": "Group.SetMute",
            "params": {
                "id": group_id,
                "mute": mute_value
            }
        });

        let response = client.send_rpc_message(message).await?;

        // Check for errors in the response
        if let Some(error) = response.get("error") {
            if let Some(code) = error.get("code").and_then(|c| c.as_i64()) {
                if code == -32603 {
                    if let Some(message) = error.get("message").and_then(|m| m.as_str()) {
                        return Err(anyhow!("Failed to set group mute status: {}", message));
                    }
                    return Err(anyhow!("Failed to set group mute status: Group not found"));
                }
            }
        }

        if let Some(result) = response.get("result") {
            final_muted = result.get("mute").and_then(|m| m.as_bool());
        }
    }

    // Handle stream ID settings if provided
    if let Some(stream_id_value) = &stream_id {
        stream_was_set = true;

        // Handle special "none" and "null" values for stream_id
        let stream_id_to_set = if stream_id_value.to_lowercase() == "none" || stream_id_value.to_lowercase() == "null" {
            String::new() // Set to empty string for "none" or "null"
        } else {
            stream_id_value.clone()
        };

        let message = json!({
            "id": Uuid::new_v4().to_string(),
            "jsonrpc": "2.0",
            "method": "Group.SetStream",
            "params": {
                "id": group_id,
                "stream_id": stream_id_to_set
            }
        });

        let response = client.send_rpc_message(message).await?;

        // Check for errors in the response
        if let Some(error) = response.get("error") {
            if let Some(code) = error.get("code").and_then(|c| c.as_i64()) {
                if code == -32603 {
                    if let Some(message) = error.get("message").and_then(|m| m.as_str()) {
                        return Err(anyhow!("Failed to set group stream: {}", message));
                    }
                    return Err(anyhow!("Failed to set group stream: Group not found"));
                }
            }
        }

        if let Some(result) = response.get("result") {
            final_stream_id = result.get("stream_id").and_then(|s| s.as_str()).map(|s| s.to_string());
        }
    }

    // Handle clients settings if provided
    if let Some(clients_value) = &clients {
        clients_was_set = true;

        // Handle special "none" and "null" values for clients
        let client_ids: Vec<String> = if clients_value.to_lowercase() == "none" || clients_value.to_lowercase() == "null" {
            Vec::new() // Empty vector for "none" or "null"
        } else {
            clients_value.split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        };

        let message = json!({
            "id": Uuid::new_v4().to_string(),
            "jsonrpc": "2.0",
            "method": "Group.SetClients",
            "params": {
                "id": group_id,
                "clients": client_ids
            }
        });

        let response = client.send_rpc_message(message).await?;

        // Check for errors in the response
        if let Some(error) = response.get("error") {
            if let Some(code) = error.get("code").and_then(|c| c.as_i64()) {
                if code == -32603 {
                    if let Some(message) = error.get("message").and_then(|m| m.as_str()) {
                        return Err(anyhow!("Failed to set group clients: {}", message));
                    }
                    return Err(anyhow!("Failed to set group clients: Group not found"));
                }
            }
        }

        final_clients = client_ids;
    }

    // If no parameters were set
    if !name_was_set && !mute_was_set && !stream_was_set && !clients_was_set {
        println!("No parameters specified to set. Use --name, --mute, --stream-id, or --clients.");
        return Ok(());
    }

    let headers = vec!["GROUP ID", "NAME", "MUTED", "STREAM ID", "CLIENTS"];
    let data = vec![vec![
        group_id.to_string(),
        final_name.as_deref().unwrap_or("unknown").to_string(),
        match final_muted {
            Some(true) => "true",
            Some(false) => "false",
            None => "unknown"
        }.to_string(),
        final_stream_id.as_deref().unwrap_or("none").to_string(),
        final_clients.join(", "),
    ]];

    print_table(headers, data);

    Ok(())
}