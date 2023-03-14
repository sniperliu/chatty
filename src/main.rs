mod client;
mod command;
mod hall;

use futures::stream::StreamExt;
use log::{error, info};
use std::env;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env::set_var("RUST_LOG", "DEBUG");
    env_logger::init();

    dotenvy::dotenv().ok();

    info!("chatty server init ...");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let address = format!("{}:{}", host, port);

    match TcpListener::bind(&address).await {
        Ok(listener) => {
            let hall = &Arc::new(hall::Hall::new());
            TcpListenerStream::new(listener)
                .for_each_concurrent(None, |socket| async move {
                    let hall = hall.clone();
                    let mut client = client::Client::new(socket.unwrap());
                    hall.clone().process(&mut client).await.unwrap();
                })
                .await;
        }
        Err(e) => {
            error!("chatty server failed to start because {}", e);
        }
    };

    Ok(())
}
