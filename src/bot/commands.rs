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

macro_rules! command {
    ($x:ident, $alias:ident, $name:expr, add) => {
        pub struct $x;
        impl Command for $x {
            fn alias(&self) -> String {
                format!("add {} answer", $name)
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
                let answers = mut_cfg.$alias.get_mut_possible_answers();
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
    ($x:ident, $alias:ident, $name:expr, del) => {
        pub struct $x;
        impl Command for $x {
            fn alias(&self) -> String {
                format!("del {} answer", $name)
            }

            fn code(
                &self,
                msg: VkMessageData,
                cfg: Arc<Mutex<AppConfig>>,
                group_client: Arc<GroupClient>,
            ) {
                let del_answer = msg.text.replace(self.alias().as_str(), "");
                let del_answer = del_answer.trim();
                let mut mut_cfg = cfg.lock().unwrap();
                let answers = mut_cfg.$alias.get_mut_possible_answers();
                if !answers.contains(&del_answer.to_string()) {
                    drop(mut_cfg);
                    error(msg, cfg, group_client, "answer doesnt exists".to_string());
                    return;
                }
                answers.retain(|answer| answer != del_answer);
                mut_cfg.write();
                drop(mut_cfg);
                success(msg, cfg, group_client);
            }
        }
    };
}

command!(AddAnecdote, anecdote, "anecdote", add);
command!(AddCheckOk, checkok, "checkok", add);
command!(AddErrorMsg, error, "error", add);
command!(AddUnresolved, unresolved, "unresolved", add);
command!(AddForbidden, forbidden, "forbidden", add);
command!(AddSuccess, success, "success", add);

command!(DelAnecdote, anecdote, "anecdote", del);
command!(DelCheckOk, checkok, "checkok", del);
command!(DelErrorMsg, error, "error", del);
command!(DelUnresolved, unresolved, "unresolved", del);
command!(DelForbidden, forbidden, "forbidden", del);
command!(DelSuccess, success, "success", del);

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
