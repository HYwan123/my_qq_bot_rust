use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sender {
    pub nickname: String,
    pub user_id: u64,

    pub card: Option<String>,
    pub role: Option<String>,
    

}