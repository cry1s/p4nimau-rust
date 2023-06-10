use vkclient::VkApi;

pub struct Config {
    pub(crate) group_token: String,
    pub(crate) user_token: String,
}

impl Config {
    pub fn new(group_token: String, user_token: String) -> Config {
        Config {
            group_token,
            user_token,
        }
    }
}

pub struct Clients {
    pub user: VkApi,
    pub group: VkApi,
}

pub fn init_clients(config: Config) -> Clients {
    Clients {
        user: vkclient::VkApiBuilder::new(config.user_token).into(),
        group: vkclient::VkApiBuilder::new(config.group_token).into(),
    }
}