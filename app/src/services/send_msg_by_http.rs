use anyhow::Result;
use reqwest::Client;
use serde_json::json;

use crate::models::qq_group::GroupInfo;

pub async fn get_group_info(client: Client, group_id: u64) -> Result<Option<GroupInfo>> {
    let resp = client
        .post("http://192.168.2.118:3000/send_like")
        .header("Authorization", "Bearer Qqwe123123")
        .json(&json!({
            "group_id": group_id,
        }))
        .send()
        .await?
        .text()
        .await?;
    match serde_json::from_str(&resp) {
        Ok(result) => Ok(Some(result)),
        Err(e) => {
            println!("在序列化{resp}时发生错误{e}");
            Ok(None)
        }
    }
}

pub async fn send_private_message_http(client: Client, user_id: u64, text: &str) -> Result<()> {
    client
        .post("http://192.168.2.118:3000/send_private_msg")
        .header("Authorization", "Bearer Qqwe123123")
        .json(&json!({
            "user_id": user_id,
            "message": [
                {
                    "type": "text",
                    "data": {
                        "text": text
                    }
                }
            ]
        }))
        .send()
        .await?
        .text()
        .await?;
    Ok(())
}
