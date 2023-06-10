use std::env;
use serde::{Serialize, Deserialize};
use crate::{Config, vkapi::Clients};

fn get_clients() -> Clients {
    use dotenvy_macro::dotenv;
    let group_token = env::var("VK_GROUP_TOKEN").unwrap_or(dotenv!("VK_GROUP_TOKEN").to_string());
    let user_token = env::var("VK_USER_TOKEN").unwrap_or(dotenv!("VK_USER_TOKEN").to_string());
    let config = Config::new(group_token, user_token);
    crate::vkapi::init_clients(config)
}

#[test]
fn command_trait() {
    use crate::answers::CommandAnswers;
    struct TestCommand {
        answers: Vec<String>,
    }
    impl CommandAnswers for TestCommand {
        fn get_possible_answers(&self) -> &Vec<String> {
            &self.answers
        }
        fn get_mut_possible_answers(&mut self) -> &mut Vec<String> {
            &mut self.answers
        }
    }
    let command = TestCommand {
        answers: vec!["a", "b", "c"].iter().map(|x| x.to_string()).collect(),
    };
    for _ in 0..6 {
        let answer = command.get_answer();
        assert!(command.get_possible_answers().contains(&answer.to_string()));
    }
}

#[tokio::test]
async fn clients_works() {
    let clients = get_clients();

    #[derive(Deserialize)]
    struct Empty {}

    let user_request: Result<Empty, vkclient::VkApiError> = clients.user.send_request("account.getInfo", ()).await;
    assert!(user_request.is_ok(), "User request failed!");
    
    let group_request: Result<Empty, vkclient::VkApiError> = clients.group.send_request("groups.getTokenPermissions", ()).await;
    assert!(group_request.is_ok(), "Group request failed!")
}
