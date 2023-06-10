use std::{env, error::Error};
use dotenvy_macro::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let vk_group_token = dotenv!("VK_GROUP_TOKEN").to_string();
    let vk_user_token = dotenv!("VK_USER_TOKEN").to_string();
    let config = p4nimau_rust::Config::new(vk_group_token, vk_user_token);
    p4nimau_rust::run(config).await
}
