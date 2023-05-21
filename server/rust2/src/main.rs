use tokio::{
    net::{TcpListener}
};

use log::info;
use env_logger::Env;


use rust::connexion::Connexion;
use rust::chat::Chat;

#[tokio::main]
async fn main() {
    // Logging setup
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);

    info!("Starting Global Chat");

    // Start global chat
    let (mut global_chat, global_chat_sender, global_chat_connector) = Chat::new("global".to_string());
    tokio::spawn(async move {
        loop {
            let _ = global_chat.handle().await;
        }
    });

    // Start listening to port 3000
    let listener: TcpListener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    info!("Listening");

    loop {
        // Accept a connexion (Tcp stream), returns socket and address
        let (socket, addr) = listener.accept().await.unwrap();
        let connexion = Connexion::new(socket, addr);

        // Get another sender for the global chat
        let global_chat_sender = global_chat_sender.clone();
        let global_chat_connector = global_chat_connector.clone();

        tokio::spawn(async move {
            let mut peer = connexion.get_peer().await;
            peer.connect_to_global_chat(global_chat_sender, global_chat_connector).await;

            loop {
                let connected = peer.handle().await;
                if !connected {
                    break
                }
            }

            info!("{} has disconnected", peer.username);
        });
    }

    //print!("{:?}", buffer);
}