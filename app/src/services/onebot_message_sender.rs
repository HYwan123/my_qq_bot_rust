use std::time::Duration;

use crate::models::onebot_send_json::SendMessageJson;
use anyhow::Result;
use futures::SinkExt;
use tokio::{net::TcpStream, time::sleep};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

pub async fn send_private_msg(
    socket: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
    user_id: u64,
    text: &str,
) -> Result<()> {
    //TODO :remove it
    sleep(Duration::from_secs_f64(0.1)).await;

    let sender_json =
        serde_json::to_string(&SendMessageJson::json_send_private_msg(user_id, text))?;
    socket.send(sender_json.into()).await?;
    Ok(())
}
pub async fn send_group_msg(
    socket: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
    group_id: u64,
    text: &str,
) -> Result<()> {
    //TODO :remove it
    sleep(Duration::from_secs_f64(0.1)).await;

    let sender_json = serde_json::to_string(&SendMessageJson::json_send_group_msg(group_id, text))?;
    socket.send(sender_json.into()).await?;
    Ok(())
}


