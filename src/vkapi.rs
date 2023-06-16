use std::{env, sync::Arc};

use crate::{config::AppConfig};
use dotenvy_macro::dotenv;
use futures_util::lock::Mutex;
use serde::{Deserialize, Serialize};
use vkclient::{longpoll::VkLongPoll, VkApi, VkApiError, List};

use self::types::{Forward, SendMessageRequest, SendMessageResponse};

pub mod types;

pub struct Clients {
    pub user: UserClient,
    pub group: GroupClient,
}

fn get_group_client() -> VkApi {
    vkclient::VkApiBuilder::new(
        env::var("VK_GROUP_TOKEN").unwrap_or(dotenv!("VK_GROUP_TOKEN").to_string()),
    )
    .into()
}

fn get_user_client() -> VkApi {
    vkclient::VkApiBuilder::new(
        env::var("VK_USER_TOKEN").unwrap_or(dotenv!("VK_USER_TOKEN").to_string()),
    )
    .into()
}

pub fn get_clients() -> Clients {
    Clients {
        user: UserClient(get_user_client()),
        group: GroupClient(get_group_client()),
    }
}

#[derive(Deserialize)]
pub struct LongPollServer {
    pub key: String,
    pub server: String,
    pub ts: String,
}

pub struct GroupClient(pub VkApi);
pub struct UserClient(pub VkApi);

impl UserClient {
    pub async fn get_owner_id(&self) -> Result<i32, VkApiError> {
        #[derive(Deserialize)]
        struct UserID {
            id: i32,
        }
        let request: Vec<UserID> = self.0.send_request("users.get", ()).await?;
        Ok(request[0].id)
    }

    pub fn wall_post(&self, cfg: Arc<Mutex<AppConfig>>) {
        #[derive(Serialize)]
        struct WallPostRequest {
            owner_id: i32,
            from_group: i8,
            message: String,
            attachments: List<Vec<String>>,
            publish_date: Option<u32>,
        }
    }
}

impl GroupClient {
    pub fn longpoll(&self) -> VkLongPoll {
        self.0.longpoll()
    }
    pub async fn get_long_poll_server(
        &self,
        cfg: &AppConfig,
    ) -> Result<LongPollServer, VkApiError> {
        #[derive(Serialize)]
        struct Request {
            group_id: i32,
        }
        self.0
            .send_request(
                "groups.getLongPollServer",
                Request {
                    group_id: cfg.group_id.expect("Group id is not loaded"),
                },
            )
            .await
    }

    pub async fn get_my_group_id(&self) -> Result<i32, VkApiError> {
        #[derive(Deserialize)]
        struct GroupID {
            id: i32,
        }
        let request: Vec<GroupID> = self.0.send_request("groups.getById", ()).await?;
        Ok(request[0].id)
    }
    pub fn send_msg(self: Arc<GroupClient>, peer_id: i32, message: String) {
        tokio::spawn(async move {
            let _request = self
                .0
                .send_request::<SendMessageResponse, SendMessageRequest, &str>(
                    "messages.send",
                    SendMessageRequest {
                        random_id: 0,
                        peer_id,
                        message,
                        forward: None,
                    },
                )
                .await;
        });
    }
    pub fn send_reply(
        self: Arc<GroupClient>,
        peer_id: i32,
        conversation_message_ids: i32,
        message: String,
    ) {
        tokio::spawn(async move {
            let _request = self
                .0
                .send_request::<SendMessageResponse, SendMessageRequest, &str>(
                    "messages.send",
                    SendMessageRequest {
                        random_id: 0,
                        peer_id,
                        message,
                        forward: Some(
                            serde_json::to_string_pretty(&Forward {
                                peer_id,
                                conversation_message_ids,
                                is_reply: 1,
                            })
                            .unwrap(),
                        ),
                    },
                )
                .await;
        });
    }
}
