use crate::command::Command;
use log::{debug, error};
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufStream},
    net::TcpStream,
};

#[derive(Debug)]
pub struct Client {
    pub username: Option<String>,
    socket: BufStream<TcpStream>,
    pub sender: Sender<String>,
    receiver: Receiver<String>,
}

impl Client {
    pub fn new(socket: TcpStream) -> Self {
        let (sender, receiver) = channel(100);
        Client {
            username: None,
            socket: BufStream::new(socket),
            sender,
            receiver,
        }
    }

    pub fn login(&mut self, name: String) {
        self.username = Some(name);
    }

    pub fn is_login(&self) -> bool {
        self.username.is_some()
    }

    pub async fn reply(&mut self, words: &str) -> anyhow::Result<()> {
        self.socket.write_all(words.as_bytes()).await?;
        self.socket.flush().await?;

        Ok(())
    }

    pub async fn bye(&mut self, bye: &str) -> anyhow::Result<()> {
        self.reply(bye).await?;
        self.socket.shutdown().await?;

        Ok(())
    }

    pub async fn receive(&mut self) -> anyhow::Result<Command> {
        let mut buffer = String::new();
        tokio::select! {
            // FIXME read_line is not cancel safe
            v = self.socket.read_line(&mut buffer) => {
                match v {
                    Ok(0) => {
                        debug!("client disconnect from chatty server");
                        Ok(Command::Logout)
                    }
                    Ok(_n) => {
                        debug!("chatty recieved {} from client", &buffer);
                        Ok(Command::parse(buffer))
                    }
                    Err(e) => {
                        error!("chatty encountered error {}", e);
                        Err(e.into())
                    }
                }
            },
            v = self.receiver.recv() => {
                if let Some(words) = v {
                    Ok(Command::Chat(words))
                } else {
                    // FIXME this is not correct, just a placeholder
                    Ok(Command::Logout)
                }
            },
        }
    }
}
