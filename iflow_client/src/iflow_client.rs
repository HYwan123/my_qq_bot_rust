use std::{os::windows::io::AsRawHandle, thread::sleep};

use crate::models::iflow_json::unstream_json::*;
use anyhow::{Context, Result};
use reqwest::{Client, header};
use serde::Serialize;
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

    async fn get_bearer(&self) -> String {
        format!("Bearer {}", self.api_key.read().await)
    }

    async fn send_iflow_json_text<T>(&self, json: &T) -> Result<String>
    where
        T: Serialize + ?Sized,
    {
        Ok(self
            .client
            .post(IflowClient::URL)
            .header(header::AUTHORIZATION, self.get_bearer().await)
            .header(header::CONTENT_TYPE, "application/json")
            .json(json)
            .send()
            .await?
            .text()
            .await?)
    }

    async fn send_iflow_json<T>(&self, json: &T) -> Result<IflowRecvJson>
    where
        T: Serialize + ?Sized,
    {
        for _ in 0..3 {
            let resp = self
                .client
                .post(IflowClient::URL)
                .header(header::AUTHORIZATION, self.get_bearer().await)
                .header(header::CONTENT_TYPE, "application/json")
                .json(json)
                .send()
                .await?;
            match resp.json::<IflowRecvJson>().await {
                Ok(json) => {
                    return Ok(json);
                }
                Err(e) => {
                    let msg = format!(
                        "在序列化{}时发送错误,错误:{e},当前apikey:{}",
                        self.send_iflow_json_text(json).await?,
                        self.api_key.read().await
                    );
                    println!("{msg}");
                    let new_api_key = Self::get_apikey(&self.client, &self.cookie).await?;
                    *self.api_key.write().await = new_api_key;
                    continue;
                }
            }
        }
        Err(anyhow::anyhow!(
            "发送iflow json发生错误,当前apikey:{}",
            self.api_key.read().await
        ))
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

    pub async fn iflow_message_handler(
        &self,
        recv_message: IflowRecvJson,
        send_message: IflowSendJson,
    ) -> Result<Vec<IflowRecvJson>> {
        //这是用来处理模型可能出现的function_call的
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

                let recv: IflowRecvJson = self.send_iflow_json(&send).await?;

                return_result.push(recv);
                return Ok(return_result);
            }
        }
    }

    pub async fn chat(&self, messages: Vec<AiMessageItem>) -> Result<Vec<String>> {
        for _ in 0..3 {
            let send_json: IflowSendJson = IflowSendJson::new("qwen3-max", messages.clone());
            let recv_json: IflowRecvJson = self.send_iflow_json(&send_json).await?;

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
        let send_json: ImageSendJson = ImageSendJson::new(image_url);
        let recv_json: IflowRecvJson = self.send_iflow_json(&send_json).await?;
        return Ok(recv_json.choices[0].message.content.clone());
    }
}

#[cfg(test)]
mod tests {
    use std::env;

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
        let result = iflow_client
            .get_image_info("https://httpbin.org/image/png")
            .await
            .unwrap();
        println!("{result}")
    }
}
