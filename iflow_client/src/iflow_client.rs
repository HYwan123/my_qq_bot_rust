use crate::models::iflow_json::unstream_json::*;
use anyhow::{Context, Result};
use reqwest::{Client, header};
use serde_json::json;
use tokio::sync::RwLock;
#[derive(Debug)]
pub struct IflowClient {
    client: Client,
    cookie: String,
    api_key: RwLock<String>,
}

impl IflowClient {
    const URL: &str = "https://apis.iflow.cn/v1/chat/completions";
    pub async fn new(client: Client, cookie: &str) -> Self {
        if cookie == "" {
            panic!("未配置iflow cookie")
        }
        let api_key = Self::get_apikey(&client, cookie)
            .await
            .context("iflow_cookie无效")
            .unwrap();
        Self {
            client,
            cookie: cookie.to_string(),
            api_key: RwLock::new(api_key.to_string()),
        }
    }

    pub async fn get_apikey(client: &Client, cookie: &str) -> Result<String> {
        let resp = client
            .get("https://platform.iflow.cn/api/openapi/apikey")
            .header("Cookie", cookie)
            .send()
            .await?;

        let status = resp.status();
        let json: IflowApiKey = resp.json().await?;

        dbg!(status);
        if json.data.has_expired == true {
            let new_json: IflowApiKey = client
                .post("https://platform.iflow.cn/api/openapi/apikey")
                .header("Cookie", cookie)
                .json(&json!({"name": json.data.name}))
                .send()
                .await?
                .json()
                .await?;
            return Ok(new_json.data.api_key_mask);
        }

        Ok(json.data.api_key_mask)
    }

    pub fn chat_streamable() {
        todo!()
    }

    pub async fn iflow_message_handler( //这是用来处理模型可能出现的function_call的
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
                let authorization = format!("Bearer {}", self.api_key.read().await);
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
                    tool_calls: Some(vec![FunctionTool {
                        index: send_index,
                        id: send_id,
                        function: send_function,
                        r#type: "function".to_string(),
                    }]),
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
        for _ in 0..3 {
            let send_json: IflowSendJson = IflowSendJson::new("qwen3-max", messages.clone());
            let authorization = format!("Bearer {}", self.api_key.read().await);
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
                        self.api_key.read().await
                    );
                    println!("{msg}");
                    let new_api_key = Self::get_apikey(&self.client, &self.cookie).await?;
                    *self.api_key.write().await = new_api_key;
                    continue;
                }
            };

            let handled_json = self.iflow_message_handler(recv_json, send_json).await?;

            let result = handled_json
                .iter()
                .map(|v: &IflowRecvJson| v.choices[0].message.content.clone())
                .collect();
            return Ok(result);
        }
        return Err(anyhow::anyhow!("重试3次仍然失败"));
    }

    pub async fn simple_chat(self, text: &str) -> Result<String> {
        let messages = vec![AiMessageItem::User(AiMessages::new(text))];
        let mut result = "".to_string();
        self.chat(messages)
            .await?
            .iter()
            .for_each(|v| result = result.clone() + v + "\n");
        Ok(result)
    }

    pub async fn get_image_info(&self, image_url: &str) -> Result<String> {
        let send_json = ImageSendJson::new(image_url);
        for _ in 0..3 {
            let authorization = format!("Bearer {}", self.api_key.read().await);
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
                        self.api_key.read().await
                    );
                    println!("{msg}");
                    let new_api_key = Self::get_apikey(&self.client, &self.cookie).await?;
                    *self.api_key.write().await = new_api_key;
                    continue;
                }
            };
            return Ok(recv_json.choices[0].message.content.clone());
        }
        return Err(anyhow::anyhow!("重试3次仍然失败"));
    
    }

}

#[cfg(test)]
mod tests {
    use std::{env, result};

    use reqwest::Client;

    #[tokio::test]
    async fn test_simple_chat() {
        dotenvy::dotenv().unwrap();
        let cookie = env::var("COOKIE").unwrap();
        let iflow_client = super::IflowClient::new(Client::new(), &cookie).await;

        let result = match iflow_client.simple_chat("今天是几号").await {
            Ok(msg) => msg,
            Err(e) => panic!("{}", e),
        };
        println!("{:#?}", result)
    }
    
    #[tokio::test]
    async fn test_image() {
        dotenvy::dotenv().unwrap();
        let cookie = env::var("COOKIE").unwrap();
        let iflow_client = super::IflowClient::new(Client::new(), &cookie).await;
        let result = iflow_client.get_image_info("https://multimedia.nt.qq.com.cn/download?appid=1406&fileid=EhSjEvg9j59kwucUJu6aVGzF2TRExRje2Agg_goo-6zXlsSckwMyBHByb2RQgLsvWhDxF8h1BP-M2rHj16H58BTzegKniIIBAm5q&spec=0&rkey=CAESMBnpu_h6Y45jAPydtqoghZmiyYeZiYRypQT2X1kdKkdEaVQIy_7Lcp9wSYDkM9ilAg")
                                            .await.unwrap();
        println!("{result}")
    }
}
