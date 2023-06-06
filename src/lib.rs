use std::error::Error;

mod answers;


pub struct Config {
    pub(crate) group_token: String,
    pub(crate) user_token: String,
}

impl Config {
    pub fn new(group_token: String, user_token: String) -> Config {
        Config {
            group_token,
            user_token,
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("group_token: {}", config.group_token);
    println!("my_token: {}", config.user_token);
    Ok(())
}


#[cfg(test)]
mod tests;
