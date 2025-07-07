use crate::rpc::client::SnapcastRpcClient;
use crate::utils::display::print_table;
use anyhow::{Result, Context};
use serde_json::Value;

pub async fn get_stream(server_url: &str, stream_id: &str) -> Result<()> {
    let client = SnapcastRpcClient::new(server_url);
    let server_info = client.get_status().await?;

    // Extract server version
    let version = server_info["server"]["snapserver"]["version"]
        .as_str()
        .unwrap_or("unknown");

    // Find the specified stream
    let stream = find_stream(&server_info, stream_id)
        .with_context(|| {
            let available_streams: Vec<String> = get_available_streams(&server_info);
            format!("Stream with ID '{}' not found. Available streams: {:?}", stream_id, available_streams)
        })?;

    // Extract stream information
    let stream_id_str = stream.get("id")
        .and_then(|id| id.as_str())
        .unwrap_or("unknown");

    let status = stream.get("status")
        .and_then(|s| s.as_str())
        .unwrap_or("unknown");

    let uri = stream.get("uri")
        .and_then(|u| u.get("raw"))
        .and_then(|r| r.as_str())
        .unwrap_or("unknown");

    // Find groups associated with this stream
    let groups = find_groups_for_stream(&server_info, stream_id_str);

    let headers = vec!["STREAM ID", "STATUS", "VERSION", "GROUP ID", "CLIENTS", "URI"];
    let mut data = Vec::new();

    if groups.is_empty() {
        // If no groups found, print a single line with the stream info
        data.push(vec![
            stream_id_str.to_string(),
            status.to_string(),
            version.to_string(),
            "None".to_string(),
            "None".to_string(),
            uri.to_string(),
        ]);
    } else {
        // Print first row with all information including version
        let first_group = &groups[0];
        let first_group_id = first_group.get("id")
            .and_then(|id| id.as_str())
            .unwrap_or("unknown").to_string();

        let first_clients = get_client_ids(first_group);

        data.push(vec![
            stream_id_str.to_string(),
            status.to_string(),
            version.to_string(),
            first_group_id,
            first_clients,
            uri.to_string(),
        ]);

        // Print subsequent rows with only group ID, clients, and URI
        for group in groups.iter().skip(1) {
            let group_id = group.get("id")
                .and_then(|id| id.as_str())
                .unwrap_or("unknown").to_string();

            let clients = get_client_ids(group);

            data.push(vec![
                "".to_string(),
                "".to_string(),
                "".to_string(),
                group_id,
                clients,
                uri.to_string(),
            ]);
        }
    }

    print_table(headers, data);

    Ok(())
}

/// Helper function to get all available stream IDs for debugging
fn get_available_streams(server_info: &Value) -> Vec<String> {
    server_info.get("streams")
        .and_then(|s| s.as_array())
        .map(|streams| {
            streams.iter()
                .filter_map(|s| s.get("id").and_then(|id| id.as_str()))
                .map(|id| id.to_string())
                .collect()
        })
        .unwrap_or_default()
}

/// Find a stream by ID in the JSON structure
fn find_stream(server_info: &Value, stream_id: &str) -> Option<Value> {
    server_info.get("streams")
        .and_then(|streams| streams.as_array())
        .and_then(|streams| {
            streams.iter().find(|stream| {
                stream.get("id")
                    .and_then(|id| id.as_str())
                    .map(|id| id == stream_id)
                    .unwrap_or(false)
            }).cloned()
        })
}

/// Find groups associated with a stream ID
fn find_groups_for_stream(server_info: &Value, stream_id: &str) -> Vec<Value> {
    server_info.get("groups")
        .and_then(|g| g.as_array())
        .map(|groups| {
            groups.iter()
                .filter(|group| {
                    group.get("stream_id")
                        .and_then(|id| id.as_str())
                        .map(|id| id == stream_id)
                        .unwrap_or(false)
                })
                .cloned()
                .collect()
        })
        .unwrap_or_default()
}

/// Helper function to get client IDs for a group
fn get_client_ids(group: &Value) -> String {
    group.get("clients")
        .and_then(|c| c.as_array())
        .map(|clients| {
            clients.iter()
                .filter_map(|client| client.get("id").and_then(|id| id.as_str()))
                .collect::<Vec<_>>()
                .join(", ")
        })
        .unwrap_or_else(|| "None".to_string())
}