use vkclient::VkApi;
use crate::config::TokenConfig;

pub struct Clients {
    pub user: VkApi,
    pub group: VkApi,
}

pub fn init_clients(config: TokenConfig) -> Clients {
    Clients {
        user: vkclient::VkApiBuilder::new(config.user_token).into(),
        group: vkclient::VkApiBuilder::new(config.group_token).into(),
    }
}

pub fn get_chats(group_client: &VkApi) {

}