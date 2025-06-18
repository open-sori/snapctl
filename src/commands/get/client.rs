use crate::rpc::client::SnapcastRpcClient;
use anyhow::{Result, Context};
use serde_json::Value;

pub async fn get_client(server_url: &str, client_id: &str) -> Result<()> {
    let client = SnapcastRpcClient::new(server_url);
    let server_info = client.get_status().await?;

    // Find the specified client
    let client_data = find_client(&server_info, client_id)
        .with_context(|| {
            let available_clients: Vec<String> = get_available_clients(&server_info);
            format!("Client with ID '{}' not found. Available clients: {:?}", client_id, available_clients)
        })?;

    // Extract client information
    let client_id = client_data.get("id")
        .and_then(|id| id.as_str())
        .unwrap_or("unknown");

    // Handle instance which can be either a number or string
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
        .unwrap_or("");

    let mac = client_data.get("host")
        .and_then(|h| h.get("mac"))
        .and_then(|mac| mac.as_str())
        .unwrap_or("unknown");

    let version = client_data.get("snapclient")
        .and_then(|s| s.get("version"))
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");

    let connected = client_data.get("connected")
        .and_then(|c| c.as_bool())
        .unwrap_or(false);

    let status = if connected { "connected" } else { "disconnected" };

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

    // Find the group and stream information for this client
    let (group_id, stream_id) = find_group_and_stream_for_client(&server_info, client_id);

    // Print header with GROUP ID and STREAM ID added at the end
    println!("{:<12} {:<12} {:<12} {:<12} {:<12} {:<18} {:<8} {:<8} {:<8} {:<36} {:<12}",
        "CLIENT ID", "STATUS", "INSTANCE", "NAME", "IP", "MAC", "VERSION", "MUTED", "VOLUME", "GROUP ID", "STREAM ID");

    // Print client information with GROUP ID and STREAM ID added at the end
    println!("{:<12} {:<12} {:<12} {:<12} {:<12} {:<18} {:<8} {:<8} {:<8} {:<36} {:<12}",
        client_id,
        status,
        instance,
        name,
        client_data.get("host")
            .and_then(|h| h.get("ip"))
            .and_then(|ip| ip.as_str())
            .unwrap_or("unknown"),
        mac,
        version,
        if muted { "true" } else { "false" },
        volume,
        group_id,
        stream_id);

    Ok(())
}

/// Helper function to find the group and stream information for a specific client
fn find_group_and_stream_for_client(server_info: &Value, client_id: &str) -> (String, String) {
    let group_id = server_info.get("groups")
        .and_then(|groups| groups.as_array())
        .and_then(|groups| {
            groups.iter()
                .find(|group| {
                    group.get("clients")
                        .and_then(|c| c.as_array())
                        .map(|clients| {
                            clients.iter()
                                .any(|client| {
                                    client.get("id")
                                        .and_then(|id| id.as_str())
                                        .map(|id| id == client_id)
                                        .unwrap_or(false)
                                })
                        })
                        .unwrap_or(false)
                })
                .and_then(|group| group.get("id").and_then(|id| id.as_str()))
                .map(|id| id.to_string())
        })
        .unwrap_or_else(|| "unknown".to_string());

    let stream_id = server_info.get("groups")
        .and_then(|groups| groups.as_array())
        .and_then(|groups| {
            groups.iter()
                .find(|group| {
                    group.get("clients")
                        .and_then(|c| c.as_array())
                        .map(|clients| {
                            clients.iter()
                                .any(|client| {
                                    client.get("id")
                                        .and_then(|id| id.as_str())
                                        .map(|id| id == client_id)
                                        .unwrap_or(false)
                                })
                        })
                        .unwrap_or(false)
                })
                .and_then(|group| group.get("stream_id").and_then(|id| id.as_str()))
                .map(|id| id.to_string())
        })
        .unwrap_or_else(|| "unknown".to_string());

    (group_id, stream_id)
}

/// Helper function to get all available client IDs for debugging
fn get_available_clients(server_info: &Value) -> Vec<String> {
    server_info.get("groups")
        .and_then(|g| g.as_array())
        .map(|groups| {
            groups.iter()
                .flat_map(|group| {
                    group.get("clients").and_then(|c| c.as_array()).into_iter().flatten()
                })
                .filter_map(|client| {
                    client.get("id").and_then(|id| id.as_str())
                })
                .map(|id| id.to_string())
                .collect()
        })
        .unwrap_or_default()
}

/// Find a client by ID in the JSON structure
fn find_client(server_info: &Value, client_id: &str) -> Option<Value> {
    server_info.get("groups")
        .and_then(|groups| groups.as_array())
        .and_then(|groups| {
            groups.iter()
                .flat_map(|group| {
                    group.get("clients").and_then(|c| c.as_array()).into_iter().flatten()
                })
                .find(|client| {
                    client.get("id")
                        .and_then(|id| id.as_str())
                        .map(|id| id == client_id)
                        .unwrap_or(false)
                })
                .cloned()
        })
}