use std::env;

fn main() {
    let vk_group_token = env::var("VK_GROUP_TOKEN").expect("VK_GROUP_TOKEN must be set");
    let vk_user_token = env::var("VK_MY_TOKEN").expect("VK_MY_TOKEN must be set");
    let config = p4nimau_rust::Config::new(vk_group_token, vk_user_token);
    p4nimau_rust::run(config).unwrap_or_else(|err| {
        eprintln!("Application error: {}", err);
        std::process::exit(1);
    });
}
