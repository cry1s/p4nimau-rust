use std::env;

use crate::config::AppConfig;
use dotenvy_macro::dotenv;
use serde::{Deserialize, Serialize};
use vkclient::{VkApi, VkApiError};

pub mod types;

pub struct Clients {
    pub user: VkApi,
    pub group: VkApi,
}

pub fn get_group_client() -> VkApi {
    vkclient::VkApiBuilder::new(
        env::var("VK_GROUP_TOKEN").unwrap_or(dotenv!("VK_GROUP_TOKEN").to_string()),
    )
    .into()
}

pub fn get_user_client() -> VkApi {
    vkclient::VkApiBuilder::new(
        env::var("VK_USER_TOKEN").unwrap_or(dotenv!("VK_USER_TOKEN").to_string()),
    )
    .into()
}

pub fn get_clients() -> Clients {
    Clients {
        user: get_user_client(),
        group: get_group_client(),
    }
}

#[derive(Deserialize)]
pub struct LongPollServer {
    pub key: String,
    pub server: String,
    pub ts: String,
}

pub async fn get_long_poll_server(cfg: &AppConfig) -> Result<LongPollServer, VkApiError> {
    #[derive(Serialize)]
    struct Request {
        group_id: i32,
    }
    get_group_client()
        .send_request(
            "groups.getLongPollServer",
            Request {
                group_id: cfg.group_id.expect("Group id is not loaded"),
            },
        )
        .await
}

pub async fn get_my_group_id() -> Result<i32, VkApiError> {
    #[derive(Deserialize)]
    struct GroupID {
        id: i32,
    }
    let request: Vec<GroupID> = get_group_client()
        .send_request("groups.getById", ())
        .await?;
    Ok(request[0].id)
}

pub async fn get_owner_id() -> Result<i32, VkApiError> {
    #[derive(Deserialize)]
    struct UserID {
        id: i32,
    }
    let request: Vec<UserID> = get_user_client().send_request("users.get", ()).await?;
    Ok(request[0].id)
}

pub fn send_reply(
    peer_id: i32,
    conversation_message_ids: i32,
    message: String,
) {
    #[derive(Serialize)]
    struct SendMessageRequest {
        random_id: i32,
        peer_id: i32,
        message: String,
        forward: String,
    }
    #[derive(Serialize)]
    struct Forward {
        peer_id: i32,
        conversation_message_ids: i32,
        is_reply: i32,
    }
    #[derive(Deserialize, Debug)]
    struct SendMessageResponse {
        peer_id: i32,
        message_id: i32,
    }
    tokio::spawn(async move {
        let request = get_group_client()
            .send_request::<SendMessageResponse, SendMessageRequest, &str>(
                "messages.send",
                SendMessageRequest {
                    random_id: 0,
                    peer_id,
                    message,
                    forward: serde_json::to_string(&Forward {
                        peer_id,
                        conversation_message_ids,
                        is_reply: 1,
                    }).unwrap(),
                },
            )
            .await;
        println!("Message sent: {:?}", request)
    });
}
