use std::error::Error;

mod answers;
mod vkapi;
mod config;

pub use crate::config::TokenConfig;
use vkapi::init_clients;

pub async fn run(config: TokenConfig) -> Result<(), Box<dyn Error>> {
    let clients = init_clients(config);
    
    Ok(())
}


#[cfg(test)]
mod tests;
