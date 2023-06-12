use std::{sync::Arc};

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::{
    config::AppConfig,
    vkapi::types::VkMessage,
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "object")]
#[serde(rename_all = "snake_case")]
pub enum Event {
    MessageNew(VkMessage)
}

impl Event {
    pub async fn handle(self, _cfg: Arc<Mutex<AppConfig>>) {
        println!("{:#?}", self);
    }
}