use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

mod commands;

use crate::{
    config::AppConfig,
    vkapi::{
        types::{VkMessage},
        GroupClient, UserClient,
    }, bot::commands::*
};

use self::commands::Command;

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
        _user_client: Arc<UserClient>,
        group_client: Arc<GroupClient>,
    ) {
        let msg = match self {
            Event::MessageNew(msg) => msg.message,
        };
        println!("got msg {:#?}", msg);
        if msg.attachments.is_empty() {
            macro_rules! execute {
                ($x:expr) => {
                    if msg.text.starts_with($x.alias().as_str()) {return $x.execute(msg, cfg, group_client)} 
                };
            }
            execute!(Help);
            execute!(GetCfg);
            execute!(GetMyId);
            execute!(AddAdmin);
            execute!(DelAdmin);
            execute!(AddAnecdote);
            execute!(AddCheckOk);
            execute!(AddErrorMsg);
            execute!(AddForbidden);
            execute!(AddSuccess);
            execute!(AddUnresolved);
            execute!(DelAnecdote);
            execute!(DelCheckOk);
            execute!(DelErrorMsg);
            execute!(DelForbidden);
            execute!(DelSuccess);
            execute!(DelUnresolved);
            execute!(EditAnecdote);
            execute!(EditCheckOk);
            execute!(EditErrorMsg);
            execute!(EditForbidden);
            execute!(EditSuccess);
            execute!(EditUnresolved);
            execute!(Get);
            execute!(SwitchMain);
        }
    }
}
