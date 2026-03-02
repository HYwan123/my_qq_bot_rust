use crate::models::iflow_json::unstream_json::{
    AiMessageItem, FunctionInfo, FunctionTool, IflowSendJson, Tool,
    ToolMessages,
};

pub fn get_datetime() -> String {
    println!("我被调用了");
    chrono::prelude::Local::now().to_string()
}



impl IflowSendJson {
    pub fn new(model: &str, messages: Vec<AiMessageItem>) -> Self {
        Self {
            model: model.to_string(),
            messages,
            stream: false,
            tools: Some(Self::get_all_tools()),
        }
    }

    pub fn new_without_function_tool(model: &str, messages: Vec<AiMessageItem>) -> Self {
        Self {
            model: model.to_string(),
            messages,
            stream: false,
            tools: None,
        }
    }

    pub fn get_all_tools() -> Vec<Tool> {
        vec![
            Tool {
                r#type: "function".to_string(),
                function: FunctionInfo {
                    description: "返回当前时间".to_string(),
                    name: "get_datetime".to_string(),
                    parameters: None,
                    strict: false,
                },
            },

        ]
    }

    pub async fn exec_func(func: FunctionTool) -> AiMessageItem {
        let result = match func.index {
            0 => get_datetime(),

            _ => "".to_string(),
        };
        let message = AiMessageItem::Tool(ToolMessages {
            content: result,
            tool_call_id: func.id,
        });
        message
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_time() {
        let result = chrono::prelude::Local::now();
        let result = result.to_string();
        println!("{}", result)
    }
}
