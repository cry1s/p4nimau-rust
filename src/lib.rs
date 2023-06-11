use std::error::Error;

mod vkapi;
mod config;

pub use crate::config::TokenConfig;
use vkapi::init_clients;

pub async fn run(config: TokenConfig) -> Result<(), Box<dyn Error>> {
    let clients = init_clients(config);
    let mut cfg = config::AppConfig::new();
    cfg.load_group_id(&clients.group).await;

    let longpoll = clients.group.longpoll();
    // longpoll.subscribe(vkclient::longpoll::LongPollRequest { server: (), key: (), ts: (), wait: 25, additional_params: () });
    Ok(())
}


#[cfg(test)]
mod tests;
