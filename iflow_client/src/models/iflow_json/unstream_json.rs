use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IflowApiKey {
    pub success: bool,
    pub code: String,
    pub message: String,
    pub data: ApiKeyData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyData {
    pub has_expired: bool,
    pub name: String,
    pub api_key_mask: String,
}

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
    pub tool_calls: Option<Vec<FunctionTool>>,
}

impl AiMessages {
    pub fn new(content: &str) -> AiMessages {
        AiMessages {
            content: content.to_string(),
            tool_calls: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ImageSendJson {
    pub model: String,
    pub stream: bool,
    pub messages: Vec<ImageSendMessage>,
}

impl ImageSendJson {
    pub fn new(image_url: &str) -> Self {
        Self {
            model: "qwen3-vl-plus".to_string(),
            stream: false,
            messages: vec![ImageSendMessage {
                role: "user".to_string(),
                content: vec![
                    ImageContent {
                        r#type: "text".to_string(),
                        text: Some("请你详细描述这张图片,然后请使用(用户发送了一张图片,内容:)为开头来描述".to_string()),
                        image_url: None,
                    },
                    ImageContent {
                        r#type: "image_url".to_string(),
                        text: None,
                        image_url: Some(image_url.to_string()),
                    },
                ],
            }],
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ImageSendMessage {
    pub role: String,
    pub content: Vec<ImageContent>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ImageContent {
    pub r#type: String,
    pub image_url: Option<String>,
    pub text: Option<String>,
}
