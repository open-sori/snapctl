use crate::rpc::client::SnapcastRpcClient;
use crate::utils::display::print_table;
use anyhow::Result;

pub async fn get_groups(server_url: &str) -> Result<()> {
    let client = SnapcastRpcClient::new(server_url);
    let server_info = client.get_status().await?;

    let headers = vec!["GROUP ID", "NAME", "STATUS", "STREAM ID", "CLIENTS"];
    let mut data = Vec::new();

    // Get groups array
    let groups = if let Some(groups) = server_info.get("groups").and_then(|g| g.as_array()) {
        groups
    } else {
        println!("No groups found.");
        return Ok(());
    };

    if groups.is_empty() {
        println!("No groups found.");
        return Ok(());
    }

    // Process each group
    for group in groups {
        let group_id = group.get("id")
            .and_then(|id| id.as_str())
            .unwrap_or("unknown").to_string();

        // Show empty string for undefined names instead of "undefined"
        let name = group.get("name")
            .and_then(|n| n.as_str())
            .unwrap_or("").to_string();

        let muted = group.get("muted")
            .and_then(|m| m.as_bool())
            .unwrap_or(false);

        let status = if muted { "muted" } else { "unmuted" };

        let stream_id = group.get("stream_id")
            .and_then(|id| id.as_str())
            .unwrap_or("none").to_string();

        let clients = group.get("clients")
            .and_then(|c| c.as_array())
            .map(|clients| {
                clients.iter()
                    .filter_map(|client| client.get("id").and_then(|id| id.as_str()))
                    .collect::<Vec<_>>()
                    .join(", ")
            })
            .unwrap_or_else(|| "None".to_string());

        data.push(vec![group_id, name, status.to_string(), stream_id, clients]);
    }

    print_table(headers, data);

    Ok(())
}