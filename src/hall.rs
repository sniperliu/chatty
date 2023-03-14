use crate::client::Client;
use crate::command::Command;
use log::{error, info};
use std::collections::HashMap;
use std::sync::{Mutex, RwLock};
use tokio::sync::mpsc::Sender;

#[derive(Debug)]
pub struct User {
    channel: Sender<String>,
}

#[derive(Debug)]
pub struct Hall {
    users: RwLock<HashMap<String, Mutex<User>>>,
}

impl Hall {
    pub fn new() -> Self {
        Hall {
            users: RwLock::new(HashMap::new()),
        }
    }

    pub fn login(&self, name: String, sender: Sender<String>) {
        info!("{} login chatty server", name);
        if let Ok(mut repository) = self.users.write() {
            repository.insert(name.clone(), Mutex::new(User { channel: sender }));
        } else {
            // FIXME handle exception
            error!("{} failed login chatty sever", &name);
        }
    }

    pub fn logout(&self, name: &str) {
        info!("{} logout chatty server", name);
        if let Ok(mut repository) = self.users.write() {
            repository.remove(name);
        }
    }

    pub async fn to(&self, username: String, words: String) {
        let sender = if let Ok(repository) = self.users.read() {
            // TODO check if user not login
            let user = repository.get(&username).unwrap();
            if let Ok(ref mut user) = user.lock() {
                Some(user.channel.clone())
            } else {
                None
            }
        } else {
            None
        };
        if sender.is_some() {
            let mut words = words.to_string();
            words.push_str("\n");
            sender.unwrap().send(words).await.unwrap();
        }
    }

    pub fn welcome(&self) -> &'static str {
        "Welcome to chatty!\n"
    }

    pub async fn process(&self, client: &mut Client) -> anyhow::Result<()> {
        client.reply(self.welcome()).await?;
        loop {
            match client.receive().await {
                Ok(Command::Login { username }) => {
                    let welcome = format!("Welcome {}\n", &username);
                    self.login(username.clone(), client.sender.clone());
                    client.login(username.clone());
                    client.reply(&welcome).await?;
                }
                Ok(Command::Logout) => {
                    let username: &str = &client.username.clone().unwrap();
                    if client.is_login() {
                        let bye = format!("{}, see you soon!\n", username);
                        client.bye(&bye).await?;
                        self.logout(username);
                    }
                    break;
                }
                Ok(Command::To(username, word)) => {
                    self.to(username, word).await;
                }
                Ok(Command::Chat(word)) => {
                    if client.is_login() {
                        client.reply(&word).await?;
                    } else {
                        client.reply("Please login to play.\n").await?;
                    }
                }
                Err(e) => {
                    error!("chatty encountered error {}", e);
                    break;
                }
            }
        }

        Ok(())
    }
}
