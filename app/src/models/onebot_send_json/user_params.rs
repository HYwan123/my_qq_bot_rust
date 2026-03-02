use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SendPrivateMsg {
    pub user_id: u64,
    pub message: Vec<Message>,
}

impl SendPrivateMsg {
    pub fn new(user_id: u64, text: &str) -> Self {
        Self {
            user_id,
            message: vec![Message {
                data: Data::Text {
                    text: text.to_string(),
                },
                r#type: Type::Text,
            }],
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Message {
    pub r#type: Type,
    pub data: Data,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    Text,
    Reply,
    Image,
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum Data {
    Text { text: String },
    Id { id: String },
    File { file: String },
}
