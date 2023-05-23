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

pub struct Entity {
    position: Vec<u32>,
    owner: u32
}

pub struct GameState {
    phase: u32,
    number_of_cities: u32,
    maximum_players: u32,
    number_of_players: u32
}

impl GameState {
    pub fn to_string(&self) -> String {
        let mut a = "".to_string();
        for b in [self.phase, self.number_of_cities, self.maximum_players, self.number_of_players] {
            a += &b.to_string();
        }
        a
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        String::into_bytes(self.to_string())
    }
}

pub struct Game {
    id: u32,
    title: String,

    pub game_state: GameState,
    entities: HashMap<String, Vec<Entity>>,
    
    command_history: Vec<Message>,
    command_receiver: mpsc::Receiver<Message>,
    connexion_receiver: mpsc::Receiver<UserSender>,

    connected: HashMap<String, mpsc::Sender<Vec<u8>>>,

    username_ids: BiMap<u32, String>
}

impl Game {
    pub fn new(id: u32, title: String, maximum_players: u32) -> (Self, mpsc::Sender<Message>, mpsc::Sender<UserSender>) {
        let command_history = Vec::new();
        let entities = HashMap::new();
        let connected = HashMap::new();
        let (tx, rx) = mpsc::channel(32);
        let (ctx, crx) = mpsc::channel(32);
        let username_ids = BiMap::new();

        // Set starting game state
        let game_state = GameState {
            phase: 0,
            number_of_cities: 0,
            maximum_players: maximum_players,
            number_of_players: 0
        };

        (Game {id, title, game_state, entities, command_history, command_receiver: rx, connexion_receiver: crx, connected, username_ids}, tx, ctx)
    }

    fn add_player(&mut self, user: String, tx: mpsc::Sender<Vec<u8>>) {
        // Check if player is already in the game
        match self.connected.get(&user) {
            None => { let i = 0;
                // Check if the player was previously in the game
                match self.username_ids.get_by_right(&user) {
                    // If he never was in the game
                    None => {
                        // Check if there are any spots left for the player
                        if self.game_state.number_of_players < self.game_state.maximum_players {
                            let new_user_id = self.game_state.number_of_players;
                            
                            self.username_ids.insert(new_user_id, user.clone());
                            info!("Game {}: {} was added to the game", self.id, user.clone());
                            self.connected.insert(user, tx);

                            self.game_state.number_of_players += 1;
                        } else {
                            info!("Game {}: tried adding {} but there is not enought room left in the game", self.id, user);
                        }
                    }

                    // If he was previously in the game
                    Some(id) => {
                        self.connected.insert(user.clone(), tx);
                        info!("Game {}: {} rejoined", self.id, user);
                    }
                };
            }

            Some(_) => {
                info!("Game {}: tried adding {} but he is already in the game", self.id, user);
            }
        };
    }
    
    pub async fn handle(&mut self) {
        tokio::select! {
            received_command = self.command_receiver.recv() => {
                let command = received_command.unwrap();
                info!("Game {}: Received command from {}", self.id, command.user.clone());
                self.command_history.push(command.clone());
                self.handle_command(command).await;
            }

            received_connexion = self.connexion_receiver.recv() => {
                info!("Game {}: Someone is trying to join", self.id);
                let info = received_connexion.unwrap(); 
                self.add_player(info.user, info.sender);
            }
        }
    }

    async fn handle_command(&mut self, command: Message) {

    }

    async fn send_game_state_to_all(&mut self) {
        let mut users_to_remove = Vec::new();

        // Add unique identifier to content start
        let mut to_send_back = vec![97];
        to_send_back.extend(self.game_state.to_bytes());
        
        // Send gamestate to everybody
        for (user, peer) in &self.connected {
            match peer.send(to_send_back.clone()).await {
                Ok(_) => {
                    info!("Game {}: Sent gamestate back to {}", self.id, user);
                },
                Err(_) => {
                    users_to_remove.push(user.clone());
                }
            };
        }
        
        for user in users_to_remove.iter() {
            self.connected.remove_entry(user);
            info!("Game {}: {} appears to be disconnected, he was removed from the game", self.id, user);
        }
    }

    async fn send_entities_to_all(&mut self) {

    }
}