use crate::{
    models::iflow_json::unstream_json::{AiMessageItem, AiMessages, FunctionTool, IflowRecvJson, IflowSendJson},
};
use anyhow::Result;
use reqwest::{Client, header};
#[derive(Debug)]
pub struct IflowClient {
    client: Client,
    api_key: String,
}

impl IflowClient {
    const URL: &str = "https://apis.iflow.cn/v1/chat/completions";
    pub fn new(client: Client, api_key: &str) -> Self {
        if api_key == "" {
            panic!("未配置api_key")
        }
        Self {
            client,
            api_key: api_key.to_string(),
        }
    }

    pub fn chat_streamable() {
        todo!()
    }

    pub async fn iflow_message_handler(
        &self,
        recv_message: IflowRecvJson,
        send_message: IflowSendJson,
    ) -> Result<Vec<IflowRecvJson>> {
        if recv_message.choices.is_empty() {
            return Ok(vec![recv_message]);
        }
        match recv_message.choices[0].message.tool_calls.clone() {
            None => {
                return Ok(vec![recv_message]);
            }
        Some(tools) => {
            if tools.is_empty() {
                return Ok(vec![recv_message]);
            }
            let mut return_result = Vec::new();
            return_result.push(recv_message.clone());
            let authorization = format!("Bearer {}", self.api_key);
            let mut send = send_message.clone();
            let exec_result: AiMessageItem = IflowSendJson::exec_func(tools[0].clone()).await;
                let recv = recv_message.clone();
                if recv.choices.is_empty() {
                    return Ok(vec![recv_message]);
                }
                let tool_calls = match recv.choices[0].message.tool_calls.clone() {
                    Some(calls) => calls,
                    None => return Ok(vec![recv_message]),
                };
                if tool_calls.is_empty() {
                    return Ok(vec![recv_message]);
                }
                let recv_func_info = tool_calls[0].clone();
                let send_index = recv_func_info.index;
                let send_id = recv_func_info.id;
                let send_function = recv_func_info.function;


                let assistant_content = recv_message.choices[0].message.content.clone();
                let final_content = if assistant_content.is_empty() {
                    "null".to_string()
                } else {
                    assistant_content
                };
                
                send.messages.push(AiMessageItem::Assistant(AiMessages {
                    content: final_content,
                    tool_calls: Some(vec![
                            FunctionTool {
                                index: send_index,
                                id: send_id,
                                function: send_function,
                                r#type: "function".to_string()
                            }
                        
                        ])
                }));
                // 注意：exec_result 实际上是 Tool 类型，不是 Assistant 类型
                // 这段代码永远不会执行，可以删除
                send.messages.push(exec_result);


                let chat_result = self
                    .client
                    .post(IflowClient::URL)
                    .header(header::AUTHORIZATION, authorization)
                    .header(header::CONTENT_TYPE, "application/json")
                    .json(&send)
                    .send()
                    .await?
                    .text()
                    .await?;
                let recv: IflowRecvJson = serde_json::from_str(&chat_result).unwrap();

                return_result.push(recv);
                return Ok(return_result);
            }
        }
    }

    pub async fn chat(&self, messages: Vec<AiMessageItem>) -> Result<Vec<String>> {
        let send_json: IflowSendJson = IflowSendJson::new("qwen3-max", messages);
        let authorization = format!("Bearer {}", self.api_key);
        let resp = self
            .client
            .post(IflowClient::URL)
            .header(header::AUTHORIZATION, authorization)
            .header(header::CONTENT_TYPE, "application/json")
            .json(&send_json)
            .send()
            .await?
            .text()
            .await?;
        let recv_json: IflowRecvJson = match serde_json::from_str(&resp) {
            Ok(json) => json,
            Err(e) => {
                let msg = format!(
                    "在序列化{resp}时发送错误,错误:{e},当前apikey:{}",
                    self.api_key
                );
                println!("{msg}");
                return Err(e.into());
            }
        };

        let handled_json = self.iflow_message_handler(recv_json, send_json).await?;
        
        let result = handled_json.iter().map(|v: &IflowRecvJson| v.choices[0].message.content.clone()).collect();
        Ok(result)

    }

    pub async fn simple_chat(&self, text: &str) -> Result<String> {
        let messages = vec![AiMessageItem::User(AiMessages::new(text))];
        let mut result = "".to_string();
        self.chat(messages).await?.iter().for_each(|v| {
            result = result.clone() + v + "\n"
        });
        Ok(result)
    }

    pub fn reset_api_key(&mut self, key: &str) {
        self.api_key = key.to_string();
    }
}

#[cfg(test)]
mod tests {
    use std::env;


    #[tokio::test]
    async fn test_simple_chat() {
        dotenvy::dotenv().unwrap();
        let key = env::var("API_KEY").unwrap();
        let iflow_client = super::IflowClient {
            client: reqwest::Client::new(),
            api_key: key,
        };
        let result = match iflow_client.simple_chat("今天是几号").await {
            Ok(msg) => msg,
            Err(e) => panic!("{}", e),
        };
        println!("{:#?}", result)
    }
}
