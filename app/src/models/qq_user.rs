use iflow_client::models::iflow_json::unstream_json::{AiMessageItem, AiMessages};
#[derive(Debug)]
pub struct QQUser {
    pub memory: Vec<AiMessageItem>,
    pub user_id: u64,
    pub user_name: String,
}

impl QQUser {
    pub fn new(user_id: u64, user_name: &str) -> Self {
        QQUser {
            memory: Vec::with_capacity(10),
            user_id,
            user_name: user_name.to_string()
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
