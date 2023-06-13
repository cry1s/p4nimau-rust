use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::{
    config::AppConfig,
    vkapi::{
        types::{VkMessage, VkMessageData},
        GroupClient, UserClient,
    },
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "object")]
#[serde(rename_all = "snake_case")]
pub enum Event {
    MessageNew(VkMessage),
}

impl Event {
    pub async fn handle(
        self,
        cfg: Arc<Mutex<AppConfig>>,
        user_client: Arc<UserClient>,
        group_client: Arc<GroupClient>,
    ) {
        let msg = match self {
            Event::MessageNew(msg) => msg.message,
        };
        println!("{:#?}", msg);

        if !cfg.lock().await.main_chat_ids.contains(&msg.peer_id) {
            if !cfg.lock().await.admin_chat_ids.contains(&msg.from_id) {
                return;
            }
            return handle_admin_commands(msg, cfg, user_client, group_client).await;
        }
    }
}

async fn handle_admin_commands(
    msg: VkMessageData,
    cfg: Arc<Mutex<AppConfig>>,
    _user_client: Arc<UserClient>,
    group_client: Arc<GroupClient>,
) {
    let mut cmd = msg.text.split_whitespace();
    let Some(header) = cmd.next() else { return };
    match header.to_lowercase().as_str() {
        "cfg" => msg.reply(
            serde_json::to_string_pretty(&*cfg.lock().await)
                .unwrap()
                .as_str(),
            group_client,
        ),
        
        _ => (),
    }
}
