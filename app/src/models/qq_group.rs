
#[derive(Debug)]
pub struct QQGroup {
    pub memory: Vec<AiMessageItem>,
    pub group_info: Option<GroupInfo>,
    pub group_id: u64,
}

impl QQGroup {
    pub fn new(group_id: u64, group_info: Option<GroupInfo>) -> Self {
        QQGroup {
            memory: Vec::with_capacity(10),
            group_info: group_info,
            group_id: group_id,
        }
    }


    pub fn push_memory_user(&mut self, text: &str) {
        self.memory.push(AiMessageItem::User(
            AiMessages::new(text),
        ));
    }
    pub fn push_memory_system(&mut self, text: &str) {
        self.memory.push(AiMessageItem::System(
            AiMessages::new(text),
        ));
    }
    pub fn push_memory_assistant(&mut self, text: &str) {
        let final_text = if text.is_empty() {
            "null"
        } else {
            text
        };
        self.memory.push(AiMessageItem::Assistant(
            AiMessages::new(final_text),
        ));
    }
}

use iflow_client::models::iflow_json::unstream_json::{AiMessageItem, AiMessages};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GroupInfo {
    data: GroupData,
    message: String,
    retcode: i64,
    status: String,
    wording: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GroupData {
    /// 活跃成员数
    active_member_count: i64,
    /// 群头像
    avatar_url: String,
    /// 群创建时间
    group_create_time: i64,
    /// 群号
    group_id: i64,
    /// 群介绍
    group_memo: String,
    /// 群名称
    group_name: String,
    /// 群是否被冻结
    is_freeze: bool,
    /// 群是否顶置
    is_top: bool,
    /// 最大成员数（群容量）
    max_member_count: i64,
    /// 成员数
    member_count: i64,
    /// 群主 QQ 号
    owner_id: i64,
    /// 群备注
    remark_name: String,
    /// 全员禁言结束时间
    shut_up_all_timestamp: i64,
    /// 自身禁言结束时间
    shut_up_me_timestamp: i64,
}
