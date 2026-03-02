use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextMessage {
    pub data: TextData
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextData {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageMessage {
    pub data: ImageData
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageData {
    pub file: String,
    pub file_size: u64,
    pub url: String,
}
