use serde_json::{json, Value};
use anyhow::{Result, Context};
use crate::utils::websocket::send_websocket_message;

pub struct SnapcastRpcClient {
    server_url: String,
}

impl SnapcastRpcClient {
    pub fn new(server_url: &str) -> Self {
        SnapcastRpcClient {
            server_url: server_url.to_string(),
        }
    }

    pub async fn get_status(&self) -> Result<Value> {
        let message = json!({
            "id": uuid::Uuid::new_v4().to_string(),
            "jsonrpc": "2.0",
            "method": "Server.GetStatus"
        });

        let response = send_websocket_message(&self.server_url, message)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to send websocket message: {}", e))?;

        response.get("result")
            .and_then(|r| r.get("server"))
            .cloned()
            .context("Failed to get server information from response")
            .map(|server| server.clone())
    }

    pub async fn send_rpc_message(&self, message: Value) -> Result<Value> {
        let response = send_websocket_message(&self.server_url, message)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to send websocket message: {}", e))?;

        Ok(response)
    }
}