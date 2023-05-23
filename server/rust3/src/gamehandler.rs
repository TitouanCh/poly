use tokio::{
    sync::mpsc
};

use std::sync::mpsc::Sender;
use std::{
    collections::HashMap,
};

use log::info;

use crate::peer::UserSender;
use crate::chat::Message;
use crate::game::Game;
use crate::utilities::as_u32;

pub struct GameHandler {
    id: String,
    //list_of_games: HashMap<u32, &Game>,
    number_of_games: u32,

    command_receiver: mpsc::Receiver<Message>,
    connexion_receiver: mpsc::Receiver<UserSender>,

    connected: HashMap<String, mpsc::Sender<Vec<u8>>>,

    game_command_senders: HashMap<u32, mpsc::Sender<Message>>,
    game_connection_receivers: HashMap<u32, mpsc::Sender<UserSender>>
}

impl GameHandler {
    pub fn new(id: String) -> (Self, mpsc::Sender<Message>, mpsc::Sender<UserSender>) {
        //let list_of_games = HashMap::new();
        let connected = HashMap::new();
        let (tx, rx) = mpsc::channel(32);
        let (ctx, crx) = mpsc::channel(32);
        let game_command_senders = HashMap::new();
        let game_connection_receivers = HashMap::new();
        (GameHandler {id, number_of_games: 0, command_receiver: rx, connexion_receiver: crx, connected, game_command_senders, game_connection_receivers}, tx, ctx)
    }

    fn info_as_bytes(&self, n: u32) -> Vec<u8> {
        let mut counter = 0;
        let mut bytes = Vec::new();
        /*
        for (id, game) in &self.list_of_games {
            bytes.append(&mut game.game_state.to_bytes().clone());
            counter += 1;
            if counter >= n {
                break;
            }
        }
         */
        bytes
    }

    fn add_user(&mut self, user: String, tx: mpsc::Sender<Vec<u8>>) {
        self.connected.insert(user, tx);
    }

    pub async fn handle(&mut self) {
        tokio::select! {
            received_command = self.command_receiver.recv() => {
                let command = received_command.unwrap();
                info!("GameHandler {}: Received command from {}", self.id, command.user.clone());
                //self.command_history.push(command.clone());
                self.handle_command(command).await;
            }

            received_connexion = self.connexion_receiver.recv() => {
                info!("GameHander {}: Connected user", self.id);
                let info = received_connexion.unwrap(); 
                self.add_user(info.user, info.sender);
            }
        }
    }

    async fn handle_command(&mut self, command: Message) {
        //ghi: user request info
        if command.content[0..3] == [103, 104, 105] {
            info!("GameHander {}: {} requested info", self.id, command.user.clone());
            let mut to_send_back = vec![105];
            to_send_back.extend(self.info_as_bytes(20));
            self.connected.get(&command.user).unwrap().send(to_send_back).await.unwrap();
        }

        //ghj: user requests to join a game
        if command.content[0..3] == [103, 104, 106] {
            // first 32 bytes are game id
            let game_id = as_u32(command.content[3..7].to_vec());

            self.game_connection_receivers.get(&game_id).unwrap().send(UserSender { user: command.user.clone(), sender: self.connected.get(&command.user).unwrap().clone() }).await.unwrap();
        }

        //ghc: user requests to create a game
        if command.content[0..3] == [103, 104, 99] {
            // First 24 bytes are game title
            let game_title = String::from_utf8(command.content[3..27].to_vec()).unwrap();
            // 25th byte is the max number of players
            let maximum_players = command.content[27];

            let (mut game, tx, ctx) = Game::new(self.number_of_games, game_title, maximum_players.into());
            info!("GameHander {}: {} created a game", self.id, command.user.clone());

            //self.list_of_games.insert(self.number_of_games, game);
            self.game_command_senders.insert(self.number_of_games, tx);
            self.game_connection_receivers.insert(self.number_of_games, ctx);
            self.number_of_games += 1;

            // Start up the game ---
            tokio::spawn(async move {
                loop {
                    let _ = game.handle().await;
                }
            });

            // Send back something here...
        }
    }


}