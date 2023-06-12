use crate::config::{AppConfig, TokenConfig};
use serde::{Deserialize, Serialize};
use vkclient::{VkApi, VkApiError};

pub mod types;

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

#[derive(Deserialize)]
pub struct LongPollServer {
    pub key: String,
    pub server: String,
    pub ts: String,
}

pub async fn get_long_poll_server(
    group_client: &VkApi,
    cfg: &AppConfig,
) -> Result<LongPollServer, VkApiError> {
    #[derive(Serialize)]
    struct Request {
        group_id: i32,
    }
    group_client
        .send_request(
            "groups.getLongPollServer",
            Request {
                group_id: cfg.group_id.expect("Group id is not loaded"),
            },
        )
        .await
}

pub async fn get_my_group_id(group_client: &VkApi) -> Result<i32, VkApiError> {
    #[derive(Deserialize)]
    struct GroupID {
        id: i32,
    }
    let request: Vec<GroupID> = group_client.send_request("groups.getById", ()).await?;
    Ok(request[0].id)
}

pub async fn get_owner_id(user_client: &VkApi) -> Result<i32, VkApiError> {
    #[derive(Deserialize)]
    struct UserID {
        id: i32,
    }
    let request: Vec<UserID> = user_client.send_request("users.get", ()).await?;
    Ok(request[0].id)
}
