use serde::{Deserialize, Serialize};

pub mod user_params;
pub mod group_params;

use crate::models::onebot_send_json::group_params::*;
use crate::models::onebot_send_json::user_params::*;

#[derive(Deserialize, Serialize)]
pub struct SendMessageJson {
    pub action: Actions,
    pub echo: String,
    pub params: Params,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Actions {
    SendPrivateMsg,
    SendGroupMsg,
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum Params {
    SendPrivateMsg(SendPrivateMsg),
    SendGroupMsg(SendGroupMsg),
}

impl SendMessageJson {
    pub fn json_send_private_msg(user_id: u64, text: &str) -> Self {
        Self {
            action: Actions::SendPrivateMsg,
            echo: "1".to_string(),
            params: Params::SendPrivateMsg(SendPrivateMsg::new(user_id, text)),
        }
    }

    pub fn json_send_group_msg(group_id: u64, text: &str) -> Self {
        Self {
            action: Actions::SendGroupMsg,
            echo: "1".to_string(),
            params: Params::SendGroupMsg(SendGroupMsg::new(group_id, text)),
        }
    }
}
