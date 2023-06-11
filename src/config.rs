use std::{
    fs::{read_to_string, File},
    io::BufWriter,
};

use serde::{Deserialize, Serialize};
use vkclient::VkApi;

use crate::vkapi;

pub struct TokenConfig {
    pub group_token: String,
    pub user_token: String,
}

impl TokenConfig {
    pub fn new(group_token: String, user_token: String) -> TokenConfig {
        TokenConfig {
            group_token,
            user_token,
        }
    }
}

#[derive(Deserialize, Serialize, Default)]
pub struct AppConfig {
    pub group_id: Option<i32>,
    pub admin_chat_ids: Vec<i32>,
    pub main_chat_ids: Vec<i32>,
    pub anecdote: Anecdote,
    pub checkok: CheckOk,
}

impl AppConfig {
    pub fn new() -> Self {
        let json = &read_to_string("config.json");
        match json {
            Ok(json) => serde_json::from_str(json).unwrap_or(Self::default()),
            Err(_) => Self::default(),
        }
    }

    pub fn write(&self) {
        let file = File::create("config.json").unwrap();
        serde_json::to_writer_pretty(BufWriter::new(file), self).unwrap();
    }

    pub async fn load_group_id(&mut self, group_client: &VkApi) {
        self.group_id = Some(vkapi::get_my_group_id(group_client).await);
        self.write();
    }
}

pub trait CommandAnswers {
    fn get_possible_answers(&self) -> &Vec<String>;
    fn get_mut_possible_answers(&mut self) -> &mut Vec<String>;
    fn get_chance_of_answer(&self) -> f32;
    fn get_mut_chance_of_answer(&mut self) -> &mut f32;
    fn get_answer(&self) -> Option<&str> {
        if self.get_possible_answers().len() == 0
            || rand::random::<f32>() > self.get_chance_of_answer()
        {
            return None;
        }
        let answers = self.get_possible_answers();
        let index = rand::random::<usize>() % answers.len();
        Some(answers[index].as_str())
    }
    fn add_possible_answer(&mut self, answer: String) {
        self.get_mut_possible_answers().push(answer);
    }
    fn delete_possible_answer(&mut self, answer: String) {
        self.get_mut_possible_answers().retain(|x| x != &answer);
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Anecdote {
    pub answers: Vec<String>,
    pub min_length: usize,
    pub chance_of_answer: f32,
}

impl CommandAnswers for Anecdote {
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

#[derive(Serialize, Deserialize, Default)]
pub struct CheckOk {
    pub trigger_phrase: String,
    pub answers: Vec<String>,
    pub chance_of_answer: f32,
}

impl CommandAnswers for CheckOk {
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
