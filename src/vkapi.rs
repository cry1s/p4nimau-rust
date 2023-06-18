use std::{
    env,
    io::Cursor,
    sync::{Arc, Mutex},
};

use crate::{config::AppConfig, vkapi::types::VkPhoto};
use dotenvy_macro::dotenv;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::copy, task::JoinHandle};
use vkclient::{
    longpoll::VkLongPoll,
    upload::{Form, VkUploader},
    List, VkApi, VkApiError,
};

use self::types::{Forward, SendMessageRequest, SendMessageResponse, VkMessageData};

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
        user: UserClient(get_user_client(), VkUploader::default()),
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
pub struct UserClient(pub VkApi, pub VkUploader);

impl UserClient {
    pub async fn get_owner_id(&self) -> Result<i32, VkApiError> {
        #[derive(Deserialize)]
        struct UserID {
            id: i32,
        }
        let request: Vec<UserID> = self.0.send_request("users.get", ()).await?;
        Ok(request[0].id)
    }

    pub async fn main_wall_post(
        self: Arc<UserClient>,
        http_client: Client,
        cfg: Arc<Mutex<AppConfig>>,
        last_date: Arc<Mutex<u64>>,
        msg: VkMessageData,
    ) {
        #[derive(Serialize)]
        struct WallPostRequest {
            owner_id: i32,
            from_group: i8,
            message: String,
            attachments: List<Vec<String>>,
            publish_date: Option<u64>,
        }
        #[derive(Deserialize, Debug)]
        struct Response {
            post_id: i32,
        }
        let publish_date = if *last_date.lock().unwrap() == 0 {
            *last_date.lock().unwrap() = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            None
        } else {
            *last_date.lock().unwrap() += 3600;
            Some(*last_date.lock().unwrap())
        };
        let group_id = cfg.lock().unwrap().group_id.unwrap();
        let attachments = self
            .clone()
            .reupload_attachments(http_client, msg.attachments, group_id)
            .await;
        if attachments.is_empty() {
            return;
        }
        let request = WallPostRequest {
            owner_id: -group_id,
            from_group: 1,
            message: msg.text,
            attachments: List(attachments),
            publish_date,
        };
        tokio::spawn(async move {
            let response: Result<Response, VkApiError> =
                self.0.send_request("wall.post", request).await;
            match response {
                Ok(s) => eprintln!("{}", s.post_id),
                Err(e) => eprintln!("{}", e),
            }
        });
    }

    async fn reupload_photo(
        self: Arc<UserClient>,
        http_client: Client,
        url: String,
        group_id: i32,
    ) -> Option<String> {
        #[derive(Serialize)]
        struct Request {
            group_id: i32,
            album_id: i32,
        }
        #[derive(Deserialize, Debug)]
        struct ServerResponse {
            upload_url: String,
        }
        let user_client = self.clone();
        let get_server_task: JoinHandle<ServerResponse> = tokio::spawn(async move {
            user_client
                .0
                .send_request(
                    "photos.getWallUploadServer",
                    Request {
                        group_id,
                        album_id: 279150453,
                    },
                )
                .await
                .unwrap()
        });

        let copy_client = http_client.clone();
        let tmp_dir = tempdir::TempDir::new("p4nimau").unwrap();
        let tmp_dir_path = tmp_dir.path().to_owned();
        let download_image_task = tokio::spawn(async move {
            let response = copy_client.get(url).send().await.unwrap();
            let fname = response
                .url()
                .path_segments()
                .and_then(|segments| segments.last())
                .and_then(|name| if name.is_empty() { None } else { Some(name) })
                .unwrap_or("tmp.jpg");

            let fname = tmp_dir_path.join(fname);
            let mut dest = File::create(&fname).await.unwrap();
            let mut content = Cursor::new(response.bytes().await.unwrap());
            copy(&mut content, &mut dest).await.unwrap();
            dbg!(fname)
        });
        let server = dbg!(get_server_task.await.ok())?;
        let image = download_image_task.await.ok()?;
        let mut form = Form::default();
        dbg!(form.add_file("photo", image)).ok()?;
        let uploaded = dbg!(self.1.upload(server.upload_url, form).await).ok()?;

        #[derive(Serialize, Deserialize, Debug)]
        struct UploadResponse {
            server: i32,
            photo: String,
            hash: String,
            group_id: Option<i32>,
        }
        let mut request = dbg!(serde_json::from_str::<UploadResponse>(&uploaded)).ok()?;
        request.group_id = Some(group_id);
        let photos: Vec<VkPhoto> = dbg!(
            self.0
                .send_request("photos.saveWallPhoto", dbg!(request))
                .await
        )
        .ok()?;
        Some(format!("photo{}_{}", photos[0].owner_id, photos[0].id))
    }

    async fn reupload_attachments(
        self: Arc<UserClient>,
        http_client: Client,
        attachments: Vec<types::VkMessagesAttachment>,
        group_id: i32,
    ) -> Vec<String> {
        let len = attachments.len();
        let works = attachments
            .into_iter()
            .filter_map(|attachment| match attachment {
                types::VkMessagesAttachment::Photo { photo } => {
                    photo.get_largest_size().map(|pic| {
                        tokio::spawn(self.clone().reupload_photo(
                            http_client.clone(),
                            pic.url.to_string(),
                            group_id,
                        ))
                    })
                }
                types::VkMessagesAttachment::Audio { audio: _ } => todo!(),
                types::VkMessagesAttachment::Video { video: _ } => todo!(),
                types::VkMessagesAttachment::Wall { wall: _ } => todo!(),
                types::VkMessagesAttachment::Story { story: _ } => todo!(),
                _ => None,
            });
        let mut res = Vec::with_capacity(len);
        for jh in works {
            if let Ok(Some(s)) = dbg!(jh.await) {
                res.push(s)
            }
        }
        res
    }
}

impl GroupClient {
    pub fn longpoll(&self) -> VkLongPoll {
        self.0.longpoll()
    }
    pub async fn get_long_poll_server(
        &self,
        cfg: Arc<Mutex<AppConfig>>,
    ) -> Result<LongPollServer, VkApiError> {
        #[derive(Serialize)]
        struct Request {
            group_id: i32,
        }
        let group_id = cfg.lock().unwrap().group_id;
        self.0
            .send_request(
                "groups.getLongPollServer",
                Request {
                    group_id: group_id.expect("Group id is not loaded"),
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
    pub fn _send_msg(self: Arc<GroupClient>, peer_id: i32, message: String) {
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
