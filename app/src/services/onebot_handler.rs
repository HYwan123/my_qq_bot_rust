use anyhow::{Context, Result};
use futures::StreamExt;
use serde_json::Value;
use tokio_tungstenite::{connect_async, tungstenite::client::IntoClientRequest};

use crate::{models::app_state::OneBotState, services::handle_message::message_handler};

pub async fn start_handler(app_state: OneBotState) -> Result<()> {
    let url: String = "ws://192.168.2.118:3002".into();
    let mut request = url.into_client_request()?;
    request
        .headers_mut()
        .insert("Authorization", "Bearer Qqwe123123".parse()?);
    let (mut ws_stream, _resp) = 
        connect_async(request)
            .await
            .context("连接websocket时出错")?;
    println!("连接成功!");

    loop {
        let resp = ws_stream.next().await.unwrap()?;
        
        let _resp_json: Value = serde_json::from_str(&resp.to_string())?;
        //println!("{resp_json:#?}");


        message_handler(
            app_state.clone(),
            &mut ws_stream,
            resp.into_text()?.as_str(),
        )
        .await?;

        
    }
}
