use std::{sync::Arc};

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::{
    config::AppConfig,
    vkapi::types::{VkMessage, VkMessageData},
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "object")]
#[serde(rename_all = "snake_case")]
pub enum Event {
    MessageNew(VkMessage)
}

impl Event {
    pub async fn handle(self, cfg: Arc<Mutex<AppConfig>>) {
        let msg = match self {
            Event::MessageNew(msg) => msg.message,
        };
        println!("{:#?}", msg);
        let (admins, mains) = async {
            let config = cfg.lock().await;
            (config.admin_chat_ids.clone(), config.main_chat_ids.clone())
        }.await;

        if !mains.contains(&msg.peer_id) {
            if !admins.contains(&msg.from_id) {
                return;
            }
            return handle_admin_commands(msg, cfg);
        }
        
    }
}

fn handle_admin_commands(msg: VkMessageData, _cfg: Arc<Mutex<AppConfig>>) {
    let mut cmd = msg.text.split_whitespace();
    let Some(header) = cmd.next() else { return };
    match header.to_lowercase().as_str() {
        "hello" =>{
            msg.reply("world!");
        },
        _ => ()
    }
}