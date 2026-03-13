use std::sync::Arc;

use crate::{
    models::{app_state::OneBotState, onebot_recv_json::*, qq_group::QQGroup, qq_user::QQUser},
    services::{
        onebot_message_sender::{send_group_msg, send_private_msg},
        send_msg_by_http::get_group_info,
    },
};
use anyhow::Result;
use tokio::{net::TcpStream, sync::RwLock};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

pub async fn message_handler(
    app_state: OneBotState,
    socket: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
    json_text: &str,
) -> Result<()> {
    let message_json: RecvMessageJson = match serde_json::from_str(json_text) {
        Ok(v) => v,
        Err(e) => {
            println!("发生错误{e},在序列化\n{json_text}");
            send_private_msg(
                socket,
                app_state.config.manger_qq_id,
                &format!("发生错误{e},在序列化\n{json_text}"),
            )
            .await?;
            return Ok(());
        }
    };

    match message_json {
        RecvMessageJson::UnUseMessage(_) => {
            return Ok(());
        }
        RecvMessageJson::NormalMessage(normal_message) => match normal_message {
            NormalMessage::MetaEvent(_) => {
                return Ok(());
            }
            NormalMessage::Message(user_message) => match user_message.group_id {
                Some(_) => Ok(handle_group_message(socket, app_state.clone(), user_message).await?),
                None => Ok(handle_private_message(socket, app_state.clone(), user_message).await?),
            },
            NormalMessage::MessageSent(_) => {
                return Ok(());
            }
            NormalMessage::Notice(text) => {
                return Ok(send_private_msg(
                    socket,
                    app_state.config.manger_qq_id,
                    &text.to_string(),
                )
                .await?);
            }
        },
    }
}

pub async fn handle_group_message(
    socket: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
    app_state: OneBotState,
    user_message: UserMessage,
) -> Result<()> {
    let group_id = user_message.group_id.unwrap();

    let qq_group = match app_state.groups.entry(group_id) {
        dashmap::Entry::Occupied(entry) => {
            println!(
                "收到来自群{}用户{}{}的消息,群名片{}",
                group_id,
                user_message.user_id,
                user_message.sender.nickname,
                user_message.sender.card.clone().unwrap()
            );
            entry.get().clone()
        }

        dashmap::Entry::Vacant(entry) => {
            println!("收到群{group_id}的消息,即将初始化");
            let group_info = get_group_info(app_state.reqwest_client.clone(), group_id).await?;

            let mut qq_group = QQGroup::new(group_id, group_info);
            qq_group.push_memory_system(&app_state.config.bot_prompt);
            let rw_qq_group = Arc::new(RwLock::new(qq_group));
            entry.insert(rw_qq_group.clone());
            rw_qq_group
        }
    };
    let mut writer = qq_group.write().await;
    writer.push_memory_user(&format!(
        "用户{}:{}",
        user_message.sender.nickname,
        user_message.get_first_text()
    ));
    let resp = app_state.iflow_client.chat(writer.memory.clone()).await?;
    for str in resp {
        send_group_msg(socket, group_id, &str).await?;
        writer.push_memory_assistant(&str);
    }

    Ok(())
}

pub async fn handle_private_message(
    socket: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
    app_state: OneBotState,
    user_message: UserMessage,
) -> Result<()> {
    let user_id = user_message.user_id;

    let qq_user = match app_state.users.entry(user_id) {
        dashmap::Entry::Occupied(entry) => {
            println!("收到用户{user_id}的消息");
            entry.get().clone()
        }

        dashmap::Entry::Vacant(entry) => {
            println!("收到用户{user_id}的消息,即将初始化");
            let user_name = user_message.sender.nickname.clone();
            let mut qq_user = QQUser::new(user_id, &user_name);
            qq_user.push_memory_system(&app_state.config.bot_prompt);
            let rw_qq_user = Arc::new(RwLock::new(qq_user));
            entry.insert(rw_qq_user.clone());
            rw_qq_user
        }
    };
    let mut writer = qq_user.write().await;
    let messages = user_message.clone();
    for message in messages.message {
        match message {
            MessageItem::Image(image) => {
                writer.push_memory_system(
                    &app_state
                        .iflow_client
                        .get_image_info(&image.data.url)
                        .await
                        .unwrap(),
                );
            }
            MessageItem::Text(text) => writer.push_memory_user(&text.data.text),
            MessageItem::Mface(face) => writer.push_memory_system(
                &app_state
                    .iflow_client
                    .get_image_info(&face.data.url)
                    .await
                    .unwrap(),
            ),
        }
    }

    let resp = app_state.iflow_client.chat(writer.memory.clone()).await?;
    for str in resp {
        send_private_msg(socket, user_id, &str).await?;
        writer.push_memory_assistant(&str);
    }

    Ok(())
}
