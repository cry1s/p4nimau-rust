use serde::{Serialize, Deserialize};
use vkclient::VkApi;
use crate::config::{TokenConfig, AppConfig};

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

#[derive(Deserialize)]
pub struct LongPollServer {
    
}

pub async fn get_long_poll_server(group_client: &VkApi, cfg: &AppConfig) -> LongPollServer {
    #[derive(Serialize)]
    struct Request {
        
    }
    group_client.send_request("groups.getLongPollServer", Request {

    }).await.unwrap()
}

pub async fn get_my_group_id(group_client: &VkApi) -> i32 {
    #[derive(Deserialize)]
    struct GroupID {
        id: i32,
    }
    let request: Vec<GroupID> = group_client.send_request("groups.getById", ()).await.unwrap();
    request[0].id
}