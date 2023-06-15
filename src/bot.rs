use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

mod commands;

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
    // may be new events, but i need only this one
}

impl Event {
    pub fn handle(
        self,
        cfg: Arc<Mutex<AppConfig>>,
        user_client: Arc<UserClient>,
        group_client: Arc<GroupClient>,
    ) {
        let msg = match self {
            Event::MessageNew(msg) => msg.message,
        };
        println!("got msg {:#?}", msg);
        handle_admin_commands(msg, cfg, user_client, group_client)
    }
}

fn handle_admin_commands(
    _msg: VkMessageData,
    _cfg: Arc<Mutex<AppConfig>>,
    _user_client: Arc<UserClient>,
    _group_client: Arc<GroupClient>,
) {
    todo!()
}
