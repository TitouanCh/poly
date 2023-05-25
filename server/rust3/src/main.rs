use tokio::{
    net::{TcpListener}
};

use log::info;
use env_logger::Env;

use rust::connexion::Connexion;
use rust::chat::Chat;

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
    let (mut global_chat, _global_chat_connector) = Chat::new(0, Some("global".to_string()));
    let global_chat_link = global_chat.as_link(false);

    tokio::spawn(async move {
        loop {
            let _ = global_chat.handle().await;
        }
    });

    /*
    // Start game handler
    let (mut game_handler, game_handler_sender, game_handler_connector) = GameHandler::new("main".to_string());
    tokio::spawn(async move {
        loop {
            let _ = game_handler.handle().await;
        }
    });
     */

    // Start listening to port 3000
    let listener: TcpListener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    info!("Listening");

    loop {
        // Accept a connexion (Tcp stream), returns socket and address
        let (socket, addr) = listener.accept().await.unwrap();
        let connexion = Connexion::new(socket, addr);

        // Get another link for the global chat
        let global_chat_link = global_chat_link.clone();

        // Get another sender for the game handler
        /*
        let game_handler_sender = game_handler_sender.clone();
        let game_handler_connector = game_handler_connector.clone();
         */

        tokio::spawn(async move {
            let (mut peer, peer_connector) = connexion.get_peer().await;
            peer_connector.send(global_chat_link).await.unwrap();
            //peer.connect_to_game_handler(game_handler_sender, game_handler_connector).await;

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