use serde_json::Value;
use tokio_tungstenite::connect_async;
use futures::{SinkExt, StreamExt};
use tokio_tungstenite::tungstenite::protocol::Message;

pub async fn send_websocket_message(
    url: &str,
    message: Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    let (ws_stream, _) = connect_async(url).await?;
    let (mut write, mut read) = ws_stream.split();

    write.send(Message::Text(message.to_string().into())).await?;

    if let Some(msg) = read.next().await {
        match msg? {
            Message::Text(text) => {
                let response: Value = serde_json::from_str(&text)?;
                return Ok(response);
            }
            _ => return Err("Unexpected message type".into()),
        }
    }

    Err("No response received".into())
}