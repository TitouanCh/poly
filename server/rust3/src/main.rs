use tokio::{
    net::{TcpListener}
};

use log::info;
use env_logger::Env;

use rust::connexion::Connexion;
use rust::chat::Chat;
use rust::gamehandler::GameHandler;

use rust::link::{
    linkable::Linkable,
    link::Link
};

#[tokio::main]
async fn main() {
    // Logging setup
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);

    info!("Starting global chat");

    // Start global chat
    let (mut global_chat, _global_chat_connector) = Chat::new(0, Some("Global".to_string()));
    let global_chat_link = global_chat.as_link(false);

    tokio::spawn(async move {
        loop {
            let _ = global_chat.handle().await;
        }
    });

    // Start game handler
    let (mut game_handler, _game_handler_connector) = GameHandler::new(0, Some("GameHandler".to_string()));
    let game_handler_link = game_handler.as_link(false);
    tokio::spawn(async move {
        loop {
            let _ = game_handler.handle().await;
        }
    });

    // Start listening to port 3000
    let listener: TcpListener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    info!("Listening");

    loop {
        // Accept a connexion (Tcp stream), returns socket and address
        let (socket, addr) = listener.accept().await.unwrap();
        let connexion = Connexion::new(socket, addr);

        // Get another link for the global chat
        let global_chat_link = global_chat_link.clone();

        // Get another link for the game handler
        let game_handler_link = game_handler_link.clone();

        tokio::spawn(async move {
            let (mut peer, peer_connector) = connexion.get_peer().await;
            peer_connector.send(global_chat_link).await.unwrap();
            peer_connector.send(game_handler_link).await.unwrap();

            loop {
                _ = peer.handle().await;
                /*
                if !connected {
                    break
                }
                */
            }

            //info!("{} has disconnected", peer.username);
        });
    }

    //print!("{:?}", buffer);
}