use dotenvy_macro::dotenv;
use std::{env, error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let vk_group_token =
        env::var("VK_GROUP_TOKEN").unwrap_or(dotenv!("VK_GROUP_TOKEN").to_string());
    let vk_user_token = env::var("VK_USER_TOKEN").unwrap_or(dotenv!("VK_USER_TOKEN").to_string());
    let config = p4nimau_rust::TokenConfig::new(vk_group_token, vk_user_token);
    p4nimau_rust::run(config).await
}
