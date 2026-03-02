use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IflowSendJson {
    pub model: String,
    pub messages: Vec<AiMessageItem>,
    pub stream: bool,
    pub tools: Option<Vec<Tool>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tool {
    pub r#type: String,
    pub function: FunctionInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunctionInfo {
    pub description: String,        //函数介绍,
    pub name: String,               //函数名
    pub parameters: Option<String>, //函数参数
    pub strict: bool,               //false
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IflowRecvJson {
    pub id: String,
    pub choices: Vec<Choice>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Choice {
    pub message: ChoicesMessage,
    pub finish_reason: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChoicesMessage {
    pub role: String,
    pub content: String,
    pub reasoning_content: Option<String>,
    pub tool_calls: Option<Vec<FunctionTool>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FunctionTool {
    pub index: u64,
    pub id: String,
    pub r#type: String,
    pub function: Function,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Function {
    pub arguments: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "role", rename_all = "snake_case")]
pub enum AiMessageItem {
    User(AiMessages),
    Assistant(AiMessages),
    System(AiMessages),
    Tool(ToolMessages),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ToolMessages {
    pub content: String,
    pub tool_call_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AiMessages {
    pub content: String,
    pub tool_calls: Option<Vec<FunctionTool>>
}

impl AiMessages {
    pub fn new(content: &str) -> AiMessages {
        AiMessages {
            content: content.to_string(),
            tool_calls: None,
        }
    }
}
