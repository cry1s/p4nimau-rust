use crate::vkapi::GroupClient;

use crate::config::{AppConfig, CommandAnswers};

use std::sync::Mutex;

use std::sync::Arc;

use crate::vkapi::types::VkMessageData;

pub enum Role {
    User,
    Admin,
}

pub trait Command {
    fn alias(&self) -> String;
    fn role(&self) -> Role {
        Role::Admin
    }
    fn execute(
        &self,
        msg: VkMessageData,
        cfg: Arc<Mutex<AppConfig>>,
        group_client: Arc<GroupClient>,
    ) {
        match self.role() {
            Role::User => self.code(msg, cfg, group_client),
            Role::Admin => {
                if cfg.lock().unwrap().admin_chat_ids.contains(&msg.from_id) {
                    self.code(msg, cfg, group_client)
                } else {
                    forbidden(msg, cfg, group_client)
                }
            }
        }
    }
    fn matches(&self, text: &str) -> bool {
        self.alias().starts_with(text)
    }
    fn code(&self, msg: VkMessageData, cfg: Arc<Mutex<AppConfig>>, group_client: Arc<GroupClient>);
}

pub struct GetCfg;

pub struct GetMyId;

impl Command for GetCfg {
    fn alias(&self) -> String {
        "get cfg".to_string()
    }

    fn code(&self, msg: VkMessageData, cfg: Arc<Mutex<AppConfig>>, group_client: Arc<GroupClient>) {
        msg.reply(
            serde_json::to_string(&*cfg.lock().unwrap()).unwrap(),
            group_client,
        )
    }
}

impl Command for GetMyId {
    fn alias(&self) -> String {
        "get my id".to_string()
    }

    fn role(&self) -> Role {
        Role::User
    }

    fn code(&self, msg: VkMessageData, cfg: Arc<Mutex<AppConfig>>, group_client: Arc<GroupClient>) {
        msg.reply(
            serde_json::to_string(&*cfg.lock().unwrap()).unwrap(),
            group_client,
        )
    }
}

macro_rules! command_add {
    ($x:ident $alias:expr) => {
        pub struct $x;
        impl Command for $x {
            fn alias(&self) -> String {
                format!("add {} answer", $alias)
            }

            fn code(
                &self,
                msg: VkMessageData,
                cfg: Arc<Mutex<AppConfig>>,
                group_client: Arc<GroupClient>,
            ) {
                let new_answer = msg.text.replace(self.alias().as_str(), "");
                let new_answer = new_answer.trim();
                let mut mut_cfg = cfg.lock().unwrap();
                let answers = match $alias {
                    "anecdote" => mut_cfg.anecdote.get_mut_possible_answers(),
                    "error" => mut_cfg.error.get_mut_possible_answers(),
                    "unresolved" => mut_cfg.unresolved.get_mut_possible_answers(),
                    "checkok" => mut_cfg.checkok.get_mut_possible_answers(),
                    "forbidden" => mut_cfg.forbidden.get_mut_possible_answers(),
                    "success" => mut_cfg.success.get_mut_possible_answers(),
                    _ => {
                        panic!("type is not exists")
                    }
                };
                if answers.contains(&new_answer.to_string()) {
                    drop(mut_cfg);
                    error(msg, cfg, group_client, "answer already exists".to_string());
                    return;
                }
                answers.push(new_answer.to_string());
                mut_cfg.write();
                drop(mut_cfg);
                success(msg, cfg, group_client);
            }
        }
    };
}

command_add!(AddAnecdote "anecdote");

command_add!(AddCheckOk "checkok");

command_add!(AddErrorMsg "error");

command_add!(AddUnresolved "unresolved");

command_add!(AddForbidden "forbidden");

command_add!(AddSuccess "success");

pub(crate) fn unresolved(
    msg: VkMessageData,
    cfg: Arc<Mutex<AppConfig>>,
    group_client: Arc<GroupClient>,
) {
    if let Some(answer) = cfg.lock().unwrap().unresolved.get_answer() {
        msg.reply(answer.to_string(), group_client)
    }
}

pub(crate) fn error(
    msg: VkMessageData,
    cfg: Arc<Mutex<AppConfig>>,
    group_client: Arc<GroupClient>,
    e: String,
) {
    msg.reply(
        cfg.lock()
            .unwrap()
            .error
            .get_answer()
            .unwrap_or("")
            .to_string()
            + " "
            + &e,
        group_client,
    )
}

pub(crate) fn forbidden(
    msg: VkMessageData,
    cfg: Arc<Mutex<AppConfig>>,
    group_client: Arc<GroupClient>,
) {
    msg.reply(
        cfg.lock()
            .unwrap()
            .forbidden
            .get_answer()
            .unwrap_or("forbidden")
            .to_string(),
        group_client,
    )
}

pub(crate) fn success(
    msg: VkMessageData,
    cfg: Arc<Mutex<AppConfig>>,
    group_client: Arc<GroupClient>,
) {
    msg.reply(
        cfg.lock()
            .unwrap()
            .success
            .get_answer()
            .unwrap_or("success")
            .to_string(),
        group_client,
    )
}
