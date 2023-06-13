use std::sync::Arc;

use serde::{Deserialize, Serialize};

use super::GroupClient;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct VkMessage {
    pub message: VkMessageData, // clinet_info field i dont care for now
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct VkMessageData {
    pub id: i32,
    pub conversation_message_id: i32,
    pub date: i32,
    pub peer_id: i32,
    pub from_id: i32,
    pub text: String,
    pub random_id: i32,
    pub attachments: Vec<VkMessagesAttachment>,
}

impl VkMessageData {
    pub fn reply(&self, msg: &str, client: Arc<GroupClient>) {
        client.send_reply(
            self.peer_id,
            self.conversation_message_id,
            msg.to_string(),
        );
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
    pub sizes: Vec<VkPhotoSizes>,
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
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VkWall {
    pub id: i32,
    pub text: String,
    pub attachments: Vec<VkWallAttachment>,
}
