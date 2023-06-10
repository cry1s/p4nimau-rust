use std::error::Error;

mod answers;
mod vkapi;

pub use vkapi::Config;

pub async fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("group_token: {}", config.group_token);
    println!("my_token: {}", config.user_token);
    Ok(())
}


#[cfg(test)]
mod tests;
