use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::GroupClient;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct VkMessage {
    pub message: VkMessageData, // clinet_info field i dont care for now
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct VkMessageData {
    pub id: i32,
    pub conversation_message_id: i32,
    pub date: u64,
    pub peer_id: i32,
    pub from_id: i32,
    pub text: String,
    pub random_id: i32,
    pub attachments: Vec<VkMessagesAttachment>,
}

impl VkMessageData {
    pub fn reply(&self, message: String, client: Arc<GroupClient>) {
        client.send_reply(self.peer_id, self.conversation_message_id, message);
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum VkMessagesAttachment {
    Photo {
        photo: VkPhoto,
    },
    Audio {
        audio: VkAudio,
    },
    Video {
        video: VkVideo,
    },
    Wall {
        wall: VkWall,
    },
    Story {
        story: VkStory,
    },
    Doc,
    Link,
    Market,
    #[serde(rename = "market_album")]
    MarketAlbum,
    #[serde(rename = "wall_reply")]
    WallReply,
    Sticker,
    Gift,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct VkStory {
    pub id: i32,
    pub video: Option<VkStoryVideo>,
    pub photo: Option<VkPhoto>,
}

impl VkStory {
    pub fn get_url(&self) -> Option<String> {
        if let Some(video) = &self.video {
            return Some(video.get_url());
        } else if let Some(photo) = &self.photo {
            return photo.get_largest_size().map(|size| size.url.to_string());
        };
        None
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VkStoryVideo {
    files: Map<String, Value>, // Map<String, String>
}

impl VkStoryVideo {
    pub fn get_url(&self) -> String {
        // Sometimes its like:
        // {
        //     "failover_host": String("..."),
        //     "mp4_720": String("..."),
        // },
        // where i need to got only mp4_720
        // but sometimes
        // {
        //     "dash_sep": String("..."),
        //     "dash_webm": String("..."),
        //     "failover_host": String("..."),
        //     "hls": String("..."),
        //     "mp4_144": String("..."),
        //     "mp4_240": String("..."),
        //     "mp4_360": String("..."),
        //     "mp4_480": String("..."),
        // }
        // and i need to choose best. i think it will be max 720, so i will just peek max key
        let mp4_720 = self.files.get("mp4_720");
        match mp4_720 {
            Some(url) => url.as_str().unwrap().to_string(),
            None => {
                let key = self.files.keys().max().unwrap();
                self.files.get(key).unwrap().as_str().unwrap().to_string()
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum VkWallAttachment {
    Photo {
        photo: VkPhoto,
    },
    Audio {
        audio: VkAudio,
    },
    Video {
        video: VkVideo,
    },
    Wall {
        wall: VkWall,
    },
    Doc,
    Market,
    #[serde(rename = "posted_photo")]
    PostedPhoto,
    Graffiti,
    Link,
    Note,
    App,
    Poll,
    Page,
    Album,
    #[serde(rename = "photos_list")]
    PhotosList,
    #[serde(rename = "market_album")]
    MarketAlbum,
    Sticker,
    #[serde(rename = "pretty_cards")]
    PrettyCards,
    Event,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VkPhoto {
    pub id: i32,
    pub owner_id: i32,
    pub user_id: Option<i32>,
    pub sizes: Vec<VkPhotoSizes>,
    pub access_key: Option<String>,
}

impl VkPhoto {
    pub fn get_largest_size(&self) -> Option<&VkPhotoSizes> {
        self.sizes.iter().max_by_key(|x| {
            // o, p, q, r - Обрезанный размер фото - делаем минимальным
            if "opqr".contains(&x.r#type) {
                "a"
            } else {
                &x.r#type
            }
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VkPhotoSizes {
    pub r#type: String,
    pub url: String,
    pub width: i32,
    pub height: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VkAudio {
    pub id: i32,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VkVideo {
    pub id: i32,
    pub owner_id: i32,
    pub title: String,
    pub access_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VkWall {
    pub id: i32,
    pub text: String,
    pub attachments: Vec<VkWallAttachment>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Conversation {
    pub chat_settings: Option<ChatSettings>,
    pub peer: Peer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Peer {
    pub id: i32,
    pub local_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatSettings {
    pub title: Option<String>,
}

#[derive(Serialize)]
pub struct SendMessageRequest {
    pub random_id: i32,
    pub peer_id: i32,
    pub message: String,
    pub forward: Option<String>,
}
#[derive(Serialize)]
pub struct Forward {
    pub peer_id: i32,
    pub conversation_message_ids: i32,
    pub is_reply: i32,
}
#[derive(Deserialize, Debug)]
pub struct SendMessageResponse {
    #[serde(rename = "peer_id")]
    pub _peer_id: i32,
    #[serde(rename = "message_id")]
    pub _message_id: i32,
}

#[derive(Deserialize, Debug)]
pub struct Profile {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}
