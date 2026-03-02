pub mod message_item;
pub mod sender_data;


use iflow_client::models::iflow_json::unstream_json::AiMessageItem;
use iflow_client::models::iflow_json::unstream_json::AiMessages;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use self::message_item::*;
use self::sender_data::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum RecvMessageJson {
    UnUseMessage(UnUseMessage), //发一个消息就返回的json,暂时没用
    NormalMessage(NormalMessage),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnUseMessage {
    echo: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "post_type", rename_all = "snake_case")]
pub enum NormalMessage {
    MessageSent(UserMessage),
    MetaEvent(HeartBeat),
    Message(UserMessage),
    Notice(Value),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HeartBeat {
    pub time: u64,
    pub self_id: u64,
    pub meta_event_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserMessage {
    pub time: u64,
    pub user_id: u64,
    pub sub_type: String,
    pub self_id: u64,
    pub raw_message: String,
    pub message_type: String,
    pub message_seq: u64,
    pub message_id: i64,
    pub message_format: String,
    pub font: u64,

    pub sender: Sender,
    pub message: Vec<MessageItem>,

    pub group_id: Option<u64>,
}

impl UserMessage {
    pub fn get_first_text(&self) -> String {
        let first_message = &self.message[0];
        let text: String = match first_message {
            MessageItem::Text(text) => text.data.text.clone(),
            MessageItem::Image(_) => "这是一张图片".to_string(),
        };
        text
    }

    pub fn get_role_message(&self) -> AiMessageItem {
        let text = self.get_first_text();
        return AiMessageItem::User(AiMessages::new(&text));
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MessageItem {
    Text(TextMessage),
    Image(ImageMessage),
}
