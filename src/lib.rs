use std::{error::Error, sync::Arc};

mod bot;
mod config;
mod vkapi;

use crate::bot::Event;
pub use crate::config::TokenConfig;
use futures_util::{pin_mut, StreamExt};
use tokio::sync::Mutex;
use vkapi::init_clients;
use vkclient::longpoll::LongPollRequest;

pub async fn run(config: TokenConfig) -> Result<(), Box<dyn Error>> {
    let clients = init_clients(config);
    let mut cfg = config::AppConfig::new();
    cfg.load_ids(&clients).await;

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

    let shared_cfg = Arc::new(Mutex::new(cfg));
    println!("-----------------Started polling!-----------------");
    while let Some(event) = stream.next().await {
        match event {
            Ok(event) => {
                tokio::spawn(event.handle(Arc::clone(&shared_cfg)));
            }
            Err(err) => eprintln!("{}", err),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
