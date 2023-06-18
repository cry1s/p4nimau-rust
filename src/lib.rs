use std::{
    error::Error,
    sync::{Arc, Mutex},
};

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

    let longpoll = clients.group.longpoll();

    let http_client = reqwest::Client::new();
    let shared_last_time_post = Arc::new(Mutex::new(0));
    let shared_group_client = Arc::new(clients.group);
    let shared_user_client = Arc::new(clients.user);
    let shared_cfg = Arc::new(Mutex::new(cfg));
    loop {
        let longpoll_input = shared_group_client
            .get_long_poll_server(Arc::clone(&shared_cfg))
            .await?;
        let stream = longpoll.subscribe::<(), Event>(LongPollRequest {
            server: longpoll_input.server,
            key: longpoll_input.key,
            ts: longpoll_input.ts,
            wait: 25,
            additional_params: (),
        });
        pin_mut!(stream);
        println!("Started polling!");
        let timestamp = std::time::SystemTime::now();
        while let Some(event) = stream.next().await {
            match event {
                Ok(event) => event.handle(
                    http_client.clone(),
                    Arc::clone(&shared_cfg),
                    Arc::clone(&shared_user_client),
                    Arc::clone(&shared_group_client),
                    Arc::clone(&shared_last_time_post),
                ),
                Err(err) => eprintln!("{}", err),
            }
        }
        println!(
            "Succesful polling for {} secs",
            std::time::SystemTime::now()
                .duration_since(timestamp)
                .unwrap()
                .as_secs()
        );
    }
}

#[cfg(test)]
mod tests;
