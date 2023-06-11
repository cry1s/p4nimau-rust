use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Event {}

impl Event {
    pub fn handle(self) {}
}