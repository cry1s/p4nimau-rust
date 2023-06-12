use crate::{
    config::AppConfig,
    vkapi::{self, get_clients},
};
use serde::Deserialize;

#[test]
fn command_trait() {
    use crate::config::CommandAnswers;
    struct TestCommand {
        answers: Vec<String>,
        chance_of_answer: f32,
    }
    impl CommandAnswers for TestCommand {
        fn get_possible_answers(&self) -> &Vec<String> {
            &self.answers
        }
        fn get_mut_possible_answers(&mut self) -> &mut Vec<String> {
            &mut self.answers
        }

        fn get_chance_of_answer(&self) -> f32 {
            self.chance_of_answer
        }

        fn get_mut_chance_of_answer(&mut self) -> &mut f32 {
            &mut self.chance_of_answer
        }
    }
    let command = TestCommand {
        answers: vec!["a", "b", "c"].iter().map(|x| x.to_string()).collect(),
        chance_of_answer: 1.0,
    };
    for _ in 0..6 {
        let answer = command.get_answer().unwrap();
        assert!(command.get_possible_answers().contains(&answer.to_string()));
    }
}

#[tokio::test]
async fn clients_works() {
    let clients = get_clients();

    #[derive(Deserialize)]
    struct Empty {}

    let user_request: Result<Empty, vkclient::VkApiError> =
        clients.user.send_request("account.getInfo", ()).await;
    assert!(user_request.is_ok(), "User request failed!");

    let group_request: Result<Empty, vkclient::VkApiError> = clients
        .group
        .send_request("groups.getTokenPermissions", ())
        .await;
    assert!(group_request.is_ok(), "Group request failed!")
}

#[test]
fn config_read_write() {
    let mut cfg = AppConfig::new();
    let len = cfg.main_chat_ids.len();
    cfg.main_chat_ids.push(0);
    cfg.write();
    cfg = AppConfig::new();
    assert_eq!(len + 1, cfg.main_chat_ids.len(), "Add element failed!");

    cfg.main_chat_ids.swap_remove(len);
    cfg.write();
    cfg = AppConfig::new();
    assert_eq!(len, cfg.main_chat_ids.len(), "Delete element failed!");
}

#[tokio::test]
async fn get_ids() {
    assert!(
        vkapi::get_my_group_id().await.is_ok(),
        "Unable to get group ID"
    );
    assert!(
        vkapi::get_owner_id().await.is_ok(),
        "Unable to get user ID"
    );
}
