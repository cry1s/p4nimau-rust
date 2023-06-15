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

pub struct AddAdmin;

pub struct DelAdmin;

pub struct Help;

pub struct Get;

impl Command for Get {
    fn alias(&self) -> String {
        "get".to_string()
    }

    fn code(&self, msg: VkMessageData, cfg: Arc<Mutex<AppConfig>>, group_client: Arc<GroupClient>) {
        let arg = msg.text.replace(self.alias().as_str(), "");
        let mut_cfg = cfg.lock().unwrap();
        msg.reply(
            "[\n".to_owned()
                + match arg.trim() {
                    "anecdote" => mut_cfg.anecdote.get_possible_answers(),
                    "checkok" => mut_cfg.checkok.get_possible_answers(),
                    "error" => mut_cfg.error.get_possible_answers(),
                    "success" => mut_cfg.success.get_possible_answers(),
                    "unresolved" => mut_cfg.unresolved.get_possible_answers(),
                    "forbidden" => mut_cfg.forbidden.get_possible_answers(),
                    _ => {
                        drop(mut_cfg);
                        return unresolved(msg, cfg, group_client)
                    }
                }
                .iter()
                .map(|x| x.to_string() + "\n")
                .collect::<String>()
                .as_str() + "]",
            group_client,
        );
    }
}

impl Command for Help {
    fn alias(&self) -> String {
        "help".to_string()
    }

    fn role(&self) -> Role {
        Role::User
    }

    fn code(&self, msg: VkMessageData, cfg: Arc<Mutex<AppConfig>>, group_client: Arc<GroupClient>) {
        msg.reply(
            "help:
        get cfg
        get my id
        add admin [id]
        del admin [id]
        add [anecdote] answer [new answer]
        del [anecdote] answer [old answer]
        edit [anecdote] chance [new chance]
        get [anecdote]
        💅💅anecdote
        💅💅checkok
        💅💅error
        💅💅unresolved
        💅💅forbidden
        💅💅success
        "
            .to_string(),
            group_client,
        )
    }
}

impl Command for DelAdmin {
    fn alias(&self) -> String {
        "del admin".to_string()
    }

    fn code(&self, msg: VkMessageData, cfg: Arc<Mutex<AppConfig>>, group_client: Arc<GroupClient>) {
        let del_admin = msg.text.replace(self.alias().as_str(), "");
        let del_admin: i32 = match del_admin.trim().parse() {
            Ok(id) => id,
            Err(e) => return error(msg, cfg, group_client, e.to_string()),
        };
        if !cfg.lock().unwrap().admin_chat_ids.contains(&del_admin) {
            error(
                msg,
                cfg,
                group_client,
                format!("admin {} doesnt exists", del_admin),
            );
            return;
        }
        let mut mut_cfg = cfg.lock().unwrap();
        mut_cfg.admin_chat_ids.retain(|id| *id != del_admin);
        mut_cfg.write();
        drop(mut_cfg);
        success(msg, cfg, group_client)
    }
}

impl Command for AddAdmin {
    fn alias(&self) -> String {
        "add admin".to_string()
    }

    fn code(&self, msg: VkMessageData, cfg: Arc<Mutex<AppConfig>>, group_client: Arc<GroupClient>) {
        let new_admin = msg.text.replace(self.alias().as_str(), "");
        let new_admin: i32 = match new_admin.trim().parse() {
            Ok(id) => id,
            Err(e) => return error(msg, cfg, group_client, e.to_string()),
        };
        if cfg.lock().unwrap().admin_chat_ids.contains(&new_admin) {
            error(
                msg,
                cfg,
                group_client,
                format!("admin {} already exists", new_admin),
            );
            return;
        }
        let mut mut_cfg = cfg.lock().unwrap();
        mut_cfg.admin_chat_ids.push(new_admin);
        mut_cfg.write();
        drop(mut_cfg);
        success(msg, cfg, group_client)
    }
}

impl Command for GetCfg {
    fn alias(&self) -> String {
        "get cfg".to_string()
    }

    fn code(&self, msg: VkMessageData, cfg: Arc<Mutex<AppConfig>>, group_client: Arc<GroupClient>) {
        msg.reply(
            serde_json::to_string_pretty(&*cfg.lock().unwrap()).unwrap(),
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

    fn code(
        &self,
        msg: VkMessageData,
        _cfg: Arc<Mutex<AppConfig>>,
        group_client: Arc<GroupClient>,
    ) {
        msg.reply(msg.from_id.to_string(), group_client)
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
    ($x:ident, $alias:ident, $name:expr, edit chance) => {
        pub struct $x;
        impl Command for $x {
            fn alias(&self) -> String {
                format!("edit {} chance", $name)
            }

            fn code(
                &self,
                msg: VkMessageData,
                cfg: Arc<Mutex<AppConfig>>,
                group_client: Arc<GroupClient>,
            ) {
                let new_chance = msg.text.replace(self.alias().as_str(), "");
                let new_chance = new_chance.trim().parse::<f32>();
                match new_chance {
                    Ok(chance) => {
                        let mut mut_cfg = cfg.lock().unwrap();
                        *mut_cfg.$alias.get_mut_chance_of_answer() = chance;
                        mut_cfg.write();
                        drop(mut_cfg);
                        success(msg, cfg, group_client);
                    }
                    Err(_) => unresolved(msg, cfg, group_client),
                }
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

command!(EditAnecdote, anecdote, "anecdote", edit chance);
command!(EditCheckOk, checkok, "checkok", edit chance);
command!(EditErrorMsg, error, "error", edit chance);
command!(EditUnresolved, unresolved, "unresolved", edit chance);
command!(EditForbidden, forbidden, "forbidden", edit chance);
command!(EditSuccess, success, "success", edit chance);

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
