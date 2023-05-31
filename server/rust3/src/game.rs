use async_trait::async_trait;
use tokio::{sync::mpsc, time};
use std::{collections::HashMap};
use bimap::BiMap;
use log::info;

use crate::{link::{
    userinfo::UserInfo,
    user::User,
    link::Link,
    linkable::Linkable,
    message::Message,
    satellite::Satellite
}, utilities::as_u32};

pub struct Entity {
    position: Vec<u32>,
    owner: u32,
    position_matters: bool
}

impl Entity {
    fn to_bytes(&self) {
    
    }
}

pub struct PlayerState {
    ready: bool,
    connected: bool,
    in_game_id: u32,
    spectator: bool,
}

impl PlayerState {
    fn new(in_game_id: u32, spectator: bool) -> Self {
        PlayerState { ready: false, connected: true, in_game_id, spectator}
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![self.ready as u8, self.connected as u8];
        bytes.extend(self.in_game_id.to_le_bytes().to_vec());
        bytes.push(self.spectator as u8);
        bytes
    }
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
        let mut bytes = Vec::new();
        for b in [self.phase, self.number_of_cities, self.maximum_players, self.number_of_players] {
            bytes.extend(&b.to_le_bytes());
        }
        bytes
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        let phase = as_u32(bytes[0..4].to_vec());
        let number_of_cities = as_u32(bytes[4..8].to_vec());
        let maximum_players = as_u32(bytes[8..12].to_vec());
        let number_of_players = as_u32(bytes[12..16].to_vec());
        GameState { phase, number_of_cities, maximum_players, number_of_players }
    }
}

pub struct Game {
    user: User,
    message_history: Vec<Message>,

    connected: HashMap<UserInfo, mpsc::Sender<Message>>,
    connected_link_senders: HashMap<UserInfo, mpsc::Sender<Link>>,
    game_handler_info: Option<UserInfo>,

    player_states: HashMap<UserInfo, PlayerState>,

    game_state: GameState,
    entities: HashMap<String, Vec<Entity>>,

    username_ids: BiMap<u32, UserInfo>,

    update_interval: time::Interval
}

#[async_trait]
impl Linkable for Game {
    fn id(&self) -> u32 { self.user.info.id.clone() }
    fn info(&self) -> UserInfo { self.user.info.clone() }
    fn message_sender(&self) -> mpsc::Sender<Message> { self.user.message.sender.clone() }
    fn link_sender(&self) -> mpsc::Sender<Link> { self.user.link.sender.clone() }
    fn mut_message(&mut self) -> &mut Satellite<Message> { &mut self.user.message}
    fn mut_link(&mut self) -> &mut Satellite<Link> { &mut self.user.link }
    fn mut_message_and_link(&mut self) -> (&mut Satellite<Message>, &mut Satellite<Link>) { (&mut self.user.message, &mut self.user.link) }
    fn mut_connected(&mut self) -> &mut HashMap<UserInfo,mpsc::Sender<Message> > { &mut self.connected }
    
    fn add_to_history(&mut self, message:Message) {
        self.message_history.push(message);
    }

    async fn handle(&mut self) -> bool {
        tokio::select! {
            received_message = self.user.message.receiver.recv() => {
                info!("{}: Received message", self.info().to_string());
                let message = received_message.unwrap();
                self.add_to_history(message.clone());

                 // Disconnected
                 if message.bytes[0] == 60 {
                    self.unlink(message.info);
                    return  true;
                }

                // Received message
                self.handle_message(message).await;
                
                true
            }

            received_link = self.user.link.receiver.recv() => {
                let link = received_link.unwrap(); 
                info!("{}: Received new link: {}", self.info().to_string(), link.info.to_string());
                self.add_linked(link).await;
                true
            }

            _ = self.update_interval.tick() => {
                self.send_player_states_to_all().await;
                self.send_info_to_gh().await;
                true
            }
        }
    }

    async fn add_linked(&mut self, link: Link) {
        let self_link = self.as_link(true);
        
        // Save player links (for some reason)
        self.connected_link_senders.insert(link.info.clone(), link.connexion_sendback.clone());

        self.connected.insert(link.info.clone(), link.message_sendback);
        
        if !link.dont_respond {
            link.connexion_sendback.send(self_link).await.unwrap();
        }

        // Save game_handle info
        if link.info.what == "game_handler" {
            self.game_handler_info = Some(link.info);
        }
    }

    async fn handle_message(&mut self, message: Message) {
        if message.info.what == "peer" {
            if message.bytes[0..3] == vec![106, 111, 105] {
                info!("{}: {} is joining", self.info().to_string(), message.info().to_string());
                // Try to add the player
                self.add_player(message.info());
            }
        }
    }
}

impl Game {
    pub fn new(id: u32, name: Option<String>, maximum_players: u32) -> (Self, mpsc::Sender<Link>) {
        let (user, link_sender) = User::new(id, name.unwrap_or("Game".to_string()), "game".to_string());
        let message_history = Vec::new();
        let connected = HashMap::new();
        let connected_link_senders = HashMap::new();
        
        let player_states = HashMap::new();

        // Set starting game state
        let game_state = GameState {
            phase: 0,
            number_of_cities: 0,
            maximum_players: maximum_players,
            number_of_players: 0
        };

        let entities = HashMap::new();
        let username_ids = BiMap::new();
        
        let update_interval = time::interval(time::Duration::from_secs(1));

        (Game {user, message_history, connected, connected_link_senders, game_handler_info: None, player_states, game_state, entities, username_ids, update_interval}, link_sender)
    }

    fn add_player(&mut self, user: UserInfo) {
        // Check if the player was previously in the game
        match self.username_ids.get_by_right(&user) {
            // If he never was in the game
            None => {
                // Check if there are any spots left for the player
                if self.game_state.number_of_players < self.game_state.maximum_players {
                    let new_user_id = self.game_state.number_of_players;
                    
                    self.username_ids.insert(new_user_id, user.clone());
                    self.player_states.insert(user.clone(), PlayerState::new(new_user_id, false));
                    info!("{}: {} was added to the game", self.info().to_string(), user.to_string());

                    self.game_state.number_of_players += 1;
                } else {
                    info!("{}: tried adding {} but there is not enought room left in the game", self.info().to_string(), user.to_string());
                }
            }

            // If he was previously in the game
            Some(_id) => {
                info!("{}: {} rejoined", self.info().to_string(), user.to_string());
            }
        };
    }

    async fn send_entities(&self, user: UserInfo, entity_type: String, identifier: u8) {
        match self.entities.get(&entity_type) {
            Some(list) => {
                let mut bytes = vec![identifier];
                for entity in list.into_iter() {
                    //bytes.push(entity.to_bytes());
                }
            }
            None => {}
        }
    }

    async fn send_player_states_to_all(&self) {
        info!("{}: sending back player states", self.info().to_string());
        for (_user, sender) in &self.connected {
            self.send_player_state(sender.clone()).await;
        }
    }

    // Send back state of the players
    async fn send_player_state(&self, tx: mpsc::Sender<Message>) { 
        for (from_user, player_state) in &self.player_states {
            let mut bytes = vec![108];
            bytes.extend(player_state.to_bytes());
            tx.send(Message { info: from_user.clone(), bytes }).await.unwrap();
        }
    }

    async fn send_info_to_gh(&mut self) {
        match &self.game_handler_info {
            Some(info) => {
                let mut bytes = vec![105];
                bytes.extend(self.game_state.to_bytes());
                self.connected.get(&info).unwrap().send(Message { info: self.info(), bytes }).await.unwrap();
            }
            None => {
                info!("{}: Not connected to game handler", self.info().to_string());
            }
        }
    }

}

