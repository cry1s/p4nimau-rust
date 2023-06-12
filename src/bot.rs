use std::sync::Arc;

use serde::Deserialize;
use tokio::sync::Mutex;

use crate::config::AppConfig;

#[derive(Deserialize, Debug)]
pub struct Event {}

impl Event {
    pub async fn handle(self, _cfg: Arc<Mutex<AppConfig>>) {

    }
}