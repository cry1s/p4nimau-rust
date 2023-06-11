use std::{error::Error};

mod config;
mod vkapi;
mod bot;

use crate::bot::Event;
pub use crate::config::TokenConfig;
use futures_util::{pin_mut, StreamExt};
use vkapi::init_clients;
use vkclient::longpoll::LongPollRequest;

pub async fn run(config: TokenConfig) -> Result<(), Box<dyn Error>> {
    let clients = init_clients(config);
    let mut cfg = config::AppConfig::new();
    cfg.load_group_id(&clients.group).await;

    let longpoll_input = vkapi::get_long_poll_server(&clients.group, &cfg).await?;
    let longpoll = clients.group.longpoll();
    
    
    let stream = longpoll.subscribe::<(), Event>(LongPollRequest {
        server: longpoll_input.server,
        key: longpoll_input.key,
        ts: longpoll_input.ts,
        wait: 25,
        additional_params: (),
    });
    pin_mut!(stream);
    while let Some(event) = stream.next().await {
        match event {
            Ok(event) => event.handle(),
            Err(err) => eprintln!("{}", err)
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
