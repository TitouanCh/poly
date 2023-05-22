use tokio::{
    sync::mpsc
};

use std::{
    collections::HashMap,
};

use bimap::BiMap;

use log::info;

use crate::peer::UserSender;
use crate::chat::Message;
use crate::game::Game;

pub struct GameHandler {
    id: String,
    list_of_games: HashMap<u32, Game>,

    command_receiver: mpsc::Receiver<Message>,
    connexion_receiver: mpsc::Receiver<UserSender>,

    connected: HashMap<String, mpsc::Sender<Vec<u8>>>
}

impl GameHandler {
    pub fn new(id: String) -> (Self, mpsc::Sender<Message>, mpsc::Sender<UserSender>) {
        let list_of_games = HashMap::new();
        let connected = HashMap::new();
        let (tx, rx) = mpsc::channel(32);
        let (ctx, crx) = mpsc::channel(32);
        (GameHandler {id, list_of_games, command_receiver: rx, connexion_receiver: crx, connected}, tx, ctx)
    }

    fn info_as_bytes(&self, n: u32) -> Vec<u8> {
        let mut counter = 0;
        let mut bytes = Vec::new();
        for (id, game) in &self.list_of_games {
            bytes.append(&mut game.game_state.to_bytes().clone());
            counter += 1;
            if counter >= n {
                break;
            }
        }
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
        //ghi : user request info
        if command.content[0..3] == [103, 104, 105] {
            info!("GameHander {}: {} requested info", self.id, command.user.clone());
            let mut to_send_back = vec![105];
            to_send_back.extend(self.info_as_bytes(20));
            self.connected.get(&command.user).unwrap().send(to_send_back).await.unwrap();
        }
    }


}