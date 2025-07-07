use crate::rpc::client::SnapcastRpcClient;
use crate::utils::display::print_table;
use anyhow::{Result, Context};
use serde_json::Value;

pub async fn get_group(server_url: &str, identifier: &str) -> Result<()> {
    let client = SnapcastRpcClient::new(server_url);
    let server_info = client.get_status().await?;

    // Extract server version
    let version = server_info["server"]["snapserver"]["version"]
        .as_str()
        .unwrap_or("unknown");

    // Find the specified group by ID or name
    let group = find_group(&server_info, identifier)
        .with_context(|| {
            let available_groups: Vec<String> = get_available_groups(&server_info);
            format!("Group with identifier '{}' not found. Available groups: {:?}", identifier, available_groups)
        })?;

    // Extract group information
    let group_id = group.get("id")
        .and_then(|id| id.as_str())
        .unwrap_or("unknown");

    let name = group.get("name")
        .and_then(|n| n.as_str())
        .unwrap_or("undefined");

    let muted = group.get("muted")
        .and_then(|m| m.as_bool())
        .unwrap_or(false);

    let status = if muted { "muted" } else { "unmuted" };

    let stream_id = group.get("stream_id")
        .and_then(|id| id.as_str())
        .unwrap_or("none");

    let clients = group.get("clients")
        .and_then(|c| c.as_array())
        .map(|clients| {
            clients.iter()
                .filter_map(|client| client.get("id").and_then(|id| id.as_str()))
                .collect::<Vec<_>>()
                .join(", ")
        })
        .unwrap_or_else(|| "None".to_string());

    let headers = vec!["GROUP ID", "NAME", "VERSION", "STATUS", "STREAM ID", "CLIENTS"];
    let data = vec![vec![
        group_id.to_string(),
        name.to_string(),
        version.to_string(),
        status.to_string(),
        stream_id.to_string(),
        clients,
    ]];

    print_table(headers, data);

    Ok(())
}

/// Helper function to get all available group IDs and names for debugging
fn get_available_groups(server_info: &Value) -> Vec<String> {
    server_info.get("groups")
        .and_then(|g| g.as_array())
        .map(|groups| {
            groups.iter()
                .filter_map(|g| {
                    let id = g.get("id").and_then(|id| id.as_str()).unwrap_or("unknown");
                    let name = g.get("name").and_then(|n| n.as_str()).unwrap_or("unnamed");
                    Some(format!("{} ({})", id, name))
                })
                .collect()
        })
        .unwrap_or_default()
}

/// Find a group by ID or name in the JSON structure
fn find_group(server_info: &Value, identifier: &str) -> Option<Value> {
    server_info.get("groups")
        .and_then(|groups| groups.as_array())
        .and_then(|groups| {
            groups.iter().find(|group| {
                // Check if ID matches
                let id_matches = group.get("id")
                    .and_then(|id| id.as_str())
                    .map(|id| id == identifier)
                    .unwrap_or(false);
                // Check if name matches
                let name_matches = group.get("name")
                    .and_then(|name| name.as_str())
                    .map(|name| name == identifier)
                    .unwrap_or(false);
                id_matches || name_matches
            }).cloned()
        })
}