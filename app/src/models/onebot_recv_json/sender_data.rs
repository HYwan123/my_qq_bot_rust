use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Sender {
    pub nickname: String,
    pub user_id: u64,

    pub card: Option<String>,
    pub role: Option<String>,
    

}