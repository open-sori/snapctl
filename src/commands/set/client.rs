use crate::rpc::client::SnapcastRpcClient;
use crate::utils::display::print_table;
use anyhow::{Result, anyhow};
use serde_json::json;
use uuid::Uuid;

pub async fn set_client(
    server_url: &str,
    client_id: &str,
    mute: Option<bool>,
    volume: Option<i64>,
    latency: Option<i64>,
    name: Option<String>,
    group: Option<String>
) -> Result<()> {
    let client = SnapcastRpcClient::new(server_url);

    // Initialize variables for group and stream information
    let mut group_id = String::from("N/A");
    let mut stream_id = String::from("N/A");
    let mut group_name = String::from("N/A");

    // Check if client exists before making any changes
    let client_status_message = json!({
        "id": Uuid::new_v4().to_string(),
        "jsonrpc": "2.0",
        "method": "Client.GetStatus",
        "params": {
            "id": client_id
        }
    });

    let client_status_response = client.send_rpc_message(client_status_message).await?;

    // Check for "Client not found" error
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

    // Handle name settings if provided
    if let Some(name_value) = name {
        let message = json!({
            "id": Uuid::new_v4().to_string(),
            "jsonrpc": "2.0",
            "method": "Client.SetName",
            "params": {
                "id": client_id,
                "name": name_value
            }
        });

        let response = client.send_rpc_message(message).await?;

        // Check for errors in the response
        if let Some(error) = response.get("error") {
            if let Some(code) = error.get("code").and_then(|c| c.as_i64()) {
                if code == -32603 {
                    if let Some(message) = error.get("message").and_then(|m| m.as_str()) {
                        return Err(anyhow!("Failed to set client name: {}", message));
                    }
                    return Err(anyhow!("Failed to set client name: Client not found"));
                }
            }
        }
    }

    // Handle volume settings if provided
    if mute.is_some() || volume.is_some() {
        let mut volume_params = json!({});

        if let Some(mute_value) = mute {
            volume_params["muted"] = json!(mute_value);
        }

        if let Some(volume_value) = volume {
            volume_params["percent"] = json!(volume_value);
        }

        let message = json!({
            "id": Uuid::new_v4().to_string(),
            "jsonrpc": "2.0",
            "method": "Client.SetVolume",
            "params": {
                "id": client_id,
                "volume": volume_params
            }
        });

        let response = client.send_rpc_message(message).await?;

        // Check for errors in the response
        if let Some(error) = response.get("error") {
            if let Some(code) = error.get("code").and_then(|c| c.as_i64()) {
                if code == -32603 {
                    if let Some(message) = error.get("message").and_then(|m| m.as_str()) {
                        return Err(anyhow!("Failed to set client volume: {}", message));
                    }
                    return Err(anyhow!("Failed to set client volume: Client not found"));
                }
            }
        }
    }

    // Handle latency settings if provided
    if let Some(latency_value) = latency {
        let message = json!({
            "id": Uuid::new_v4().to_string(),
            "jsonrpc": "2.0",
            "method": "Client.SetLatency",
            "params": {
                "id": client_id,
                "latency": latency_value
            }
        });

        let response = client.send_rpc_message(message).await?;

        // Check for errors in the response
        if let Some(error) = response.get("error") {
            if let Some(code) = error.get("code").and_then(|c| c.as_i64()) {
                if code == -32603 {
                    if let Some(message) = error.get("message").and_then(|m| m.as_str()) {
                        return Err(anyhow!("Failed to set client latency: {}", message));
                    }
                    return Err(anyhow!("Failed to set client latency: Client not found"));
                }
            }
        }
    }

    // Handle group assignment if provided
    if let Some(group_value) = &group {
        let should_remove = group_value.is_empty() ||
                            group_value.to_lowercase() == "none" ||
                            group_value.to_lowercase() == "null";

        if should_remove {
            // Remove client from its current group
            if let Some(current_group_id) = find_client_group(&client, client_id).await? {
                group_id = current_group_id.clone();

                let group_status_message = json!({
                    "id": Uuid::new_v4().to_string(),
                    "jsonrpc": "2.0",
                    "method": "Group.GetStatus",
                    "params": {
                        "id": current_group_id
                    }
                });

                let group_status_response = client.send_rpc_message(group_status_message).await?;

                // Check for errors in the response
                if let Some(error) = group_status_response.get("error") {
                    if let Some(code) = error.get("code").and_then(|c| c.as_i64()) {
                        if code == -32603 {
                            if let Some(message) = error.get("message").and_then(|m| m.as_str()) {
                                return Err(anyhow!("Failed to get group status: {}", message));
                            }
                            return Err(anyhow!("Failed to get group status: Group not found"));
                        }
                    }
                }

                // Extract group information
                if let Some(group_info) = group_status_response.get("result")
                    .and_then(|result| result.get("group")) {
                    if let Some(id) = group_info.get("id").and_then(|id| id.as_str()) {
                        group_id = id.to_string();
                    }
                    if let Some(name) = group_info.get("name").and_then(|n| n.as_str()) {
                        group_name = name.to_string();
                    }
                    if let Some(stream) = group_info.get("stream_id").and_then(|id| id.as_str()) {
                        stream_id = stream.to_string();
                    }
                }

                let existing_clients = group_status_response.get("result")
                    .and_then(|result| result.get("group"))
                    .and_then(|group| group.get("clients"))
                    .and_then(|clients| clients.as_array())
                    .map(|clients| {
                        clients.iter()
                            .filter_map(|client| client.get("id").and_then(|id| id.as_str()))
                            .filter(|&id| id != client_id)
                            .map(|id| id.to_string())
                            .collect::<Vec<String>>()
                    })
                    .unwrap_or_default();

                let set_clients_message = json!({
                    "id": Uuid::new_v4().to_string(),
                    "jsonrpc": "2.0",
                    "method": "Group.SetClients",
                    "params": {
                        "id": current_group_id,
                        "clients": existing_clients
                    }
                });

                let response = client.send_rpc_message(set_clients_message).await?;

                // Check for errors in the response
                if let Some(error) = response.get("error") {
                    if let Some(code) = error.get("code").and_then(|c| c.as_i64()) {
                        if code == -32603 {
                            if let Some(message) = error.get("message").and_then(|m| m.as_str()) {
                                return Err(anyhow!("Failed to update group clients: {}", message));
                            }
                            return Err(anyhow!("Failed to update group clients: Group not found"));
                        }
                    }
                }
            }
        } else {
            // Add client to the specified group
            let group_status_message = json!({
                "id": Uuid::new_v4().to_string(),
                "jsonrpc": "2.0",
                "method": "Group.GetStatus",
                "params": {
                    "id": group_value
                }
            });

            let group_status_response = client.send_rpc_message(group_status_message).await?;

            // Check for errors in the response
            if let Some(error) = group_status_response.get("error") {
                if let Some(code) = error.get("code").and_then(|c| c.as_i64()) {
                    if code == -32603 {
                        if let Some(message) = error.get("message").and_then(|m| m.as_str()) {
                            return Err(anyhow!("Failed to get group status: {}", message));
                        }
                        return Err(anyhow!("Failed to get group status: Group not found"));
                    }
                }
            }

            // Extract group information
            if let Some(group_info) = group_status_response.get("result")
                .and_then(|result| result.get("group")) {
                group_id = group_value.clone();
                if let Some(name) = group_info.get("name").and_then(|n| n.as_str()) {
                    group_name = name.to_string();
                }
                if let Some(stream) = group_info.get("stream_id").and_then(|id| id.as_str()) {
                    stream_id = stream.to_string();
                }
            }

            let existing_clients = group_status_response.get("result")
                .and_then(|result| result.get("group"))
                .and_then(|group| group.get("clients"))
                .and_then(|clients| clients.as_array())
                .map(|clients| {
                    clients.iter()
                        .filter_map(|client| client.get("id").and_then(|id| id.as_str()))
                        .map(|id| id.to_string())
                        .collect::<Vec<String>>()
                })
                .unwrap_or_default();

            let mut updated_clients = existing_clients;
            if !updated_clients.contains(&client_id.to_string()) {
                updated_clients.push(client_id.to_string());
            }

            let set_clients_message = json!({
                "id": Uuid::new_v4().to_string(),
                "jsonrpc": "2.0",
                "method": "Group.SetClients",
                "params": {
                    "id": group_value,
                    "clients": updated_clients
                }
            });

            let response = client.send_rpc_message(set_clients_message).await?;

            // Check for errors in the response
            if let Some(error) = response.get("error") {
                if let Some(code) = error.get("code").and_then(|c| c.as_i64()) {
                    if code == -32603 {
                        if let Some(message) = error.get("message").and_then(|m| m.as_str()) {
                            return Err(anyhow!("Failed to update group clients: {}", message));
                        }
                        return Err(anyhow!("Failed to update group clients: Group not found"));
                    }
                }
            }
        }
    }

    // Always get group info for final output
    if let Some(current_group_id) = find_client_group(&client, client_id).await? {
        let group_status_message = json!({
            "id": Uuid::new_v4().to_string(),
            "jsonrpc": "2.0",
            "method": "Group.GetStatus",
            "params": {
                "id": &current_group_id
            }
        });

        let group_status_response = client.send_rpc_message(group_status_message).await?;

        if let Some(group_info) = group_status_response.get("result").and_then(|r| r.get("group")) {
            group_id = group_info.get("id").and_then(|id| id.as_str()).unwrap_or(&current_group_id).to_string();
            group_name = group_info.get("name").and_then(|n| n.as_str()).unwrap_or("N/A").to_string();
            stream_id = group_info.get("stream_id").and_then(|s| s.as_str()).unwrap_or("N/A").to_string();
        }
    }

    // Get client status after making changes
    let client_status_message = json!({
        "id": Uuid::new_v4().to_string(),
        "jsonrpc": "2.0",
        "method": "Client.GetStatus",
        "params": {
            "id": client_id
        }
    });

    let client_status_response = client.send_rpc_message(client_status_message).await?;

    // Check for errors in the final status check
    if let Some(error) = client_status_response.get("error") {
        if let Some(code) = error.get("code").and_then(|c| c.as_i64()) {
            if code == -32603 {
                if let Some(message) = error.get("message").and_then(|m| m.as_str()) {
                    return Err(anyhow!("Failed to get final client status: {}", message));
                }
                return Err(anyhow!("Failed to get final client status: Client not found"));
            }
        }
    }

    // Extract client information for output
    let client_data = client_status_response.get("result")
        .and_then(|r| r.get("client"))
        .ok_or_else(|| anyhow::anyhow!("Failed to get client data"))?;

    let client_id_str = client_data.get("id")
        .and_then(|id| id.as_str())
        .unwrap_or("unknown");

    let status = if client_data.get("connected").and_then(|c| c.as_bool()).unwrap_or(false) {
        "connected"
    } else {
        "disconnected"
    };

    let instance = client_data.get("config")
        .and_then(|c| c.get("instance"))
        .map(|i| {
            if i.is_number() {
                i.as_i64().map(|n| n.to_string()).unwrap_or_else(|| "unknown".to_string())
            } else {
                i.as_str().unwrap_or("unknown").to_string()
            }
        })
        .unwrap_or_else(|| "unknown".to_string());

    let name = client_data.get("config")
        .and_then(|c| c.get("name"))
        .and_then(|n| n.as_str())
        .unwrap_or("unknown");

    let ip = client_data.get("host")
        .and_then(|h| h.get("ip"))
        .and_then(|ip| ip.as_str())
        .unwrap_or("unknown");

    let mac = client_data.get("host")
        .and_then(|h| h.get("mac"))
        .and_then(|mac| mac.as_str())
        .unwrap_or("unknown");

    let version = client_data.get("snapclient")
        .and_then(|s| s.get("version"))
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");

    let muted = client_data.get("config")
        .and_then(|c| c.get("volume"))
        .and_then(|v| v.get("muted"))
        .and_then(|m| m.as_bool())
        .unwrap_or(false);

    let volume = client_data.get("config")
        .and_then(|c| c.get("volume"))
        .and_then(|v| v.get("percent"))
        .and_then(|p| p.as_i64())
        .map(|p| p.to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let latency_value = client_data.get("config")
        .and_then(|c| c.get("latency"))
        .and_then(|l| l.as_i64())
        .map(|l| l.to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let headers = vec!["CLIENT ID", "STATUS", "INSTANCE", "NAME", "IP", "MAC", "VERSION", "MUTED", "VOLUME", "LATENCY", "GROUP ID", "GROUP NAME", "STREAM ID"];
    let data = vec![vec![
        client_id_str.to_string(),
        status.to_string(),
        instance,
        name.to_string(),
        ip.to_string(),
        mac.to_string(),
        version.to_string(),
        if muted { "true" } else { "false" }.to_string(),
        volume,
        latency_value,
        group_id,
        group_name,
        stream_id,
    ]];

    print_table(headers, data);

    Ok(())
}

/// Helper function to find the current group of a client
async fn find_client_group(client: &SnapcastRpcClient, client_id: &str) -> Result<Option<String>> {
    let server_status = client.get_status().await?;

    if let Some(groups) = server_status.get("groups").and_then(|g| g.as_array()) {
        for group in groups {
            if let Some(clients) = group.get("clients").and_then(|c| c.as_array()) {
                for client in clients {
                    if let Some(id) = client.get("id").and_then(|id| id.as_str()) {
                        if id == client_id {
                            return Ok(Some(group.get("id").and_then(|id| id.as_str()).unwrap_or("unknown").to_string()));
                        }
                    }
                }
            }
        }
    }

    Ok(None)
}