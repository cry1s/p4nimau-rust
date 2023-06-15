use std::{error::Error, sync::{Arc, Mutex}};

mod bot;
mod config;
mod vkapi;

use crate::bot::Event;
pub use crate::config::TokenConfig;
use futures_util::{pin_mut, StreamExt};
use vkapi::get_clients;
use vkclient::longpoll::LongPollRequest;

pub async fn run() -> Result<(), Box<dyn Error>> {
    let clients = get_clients();
    let mut cfg = config::AppConfig::new();
    cfg.load_ids(&clients).await;

    let longpoll_input = clients.group.get_long_poll_server(&cfg).await?;
    let longpoll = clients.group.longpoll();

    let stream = longpoll.subscribe::<(), Event>(LongPollRequest {
        server: longpoll_input.server,
        key: longpoll_input.key,
        ts: longpoll_input.ts,
        wait: 25,
        additional_params: (),
    });
    pin_mut!(stream);
    let shared_group_client = Arc::new(clients.group);
    let shared_user_client = Arc::new(clients.user);
    let shared_cfg = Arc::new(Mutex::new(cfg));
    shared_group_client.clone().send_msg(
        shared_cfg.lock().unwrap().admin_chat_ids[0],
        "Starting polling".to_owned(),
    );
    println!("Started polling!");
    while let Some(event) = stream.next().await {
        match event {
            Ok(event) => event.handle(
                Arc::clone(&shared_cfg),
                Arc::clone(&shared_user_client),
                Arc::clone(&shared_group_client),
            ),
            Err(err) => eprintln!("{}", err),
        }
    }
    Ok(())  
}

#[cfg(test)]
mod tests;
