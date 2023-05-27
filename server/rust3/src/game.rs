use async_trait::async_trait;
use tokio::sync::mpsc;
use std::collections::HashMap;
use bimap::BiMap;
use log::info;

use crate::link::{
    userinfo::UserInfo,
    user::User,
    link::Link,
    linkable::Linkable,
    message::Message,
    satellite::Satellite
};

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
    user: User,
    message_history: Vec<Message>,

    connected: HashMap<UserInfo, mpsc::Sender<Message>>,
    connected_link_senders: HashMap<UserInfo, mpsc::Sender<Link>>,

    game_state: GameState,
    entities: HashMap<String, Vec<Entity>>,

    username_ids: BiMap<u32, String>
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

    async fn add_linked(&mut self, link: Link) {
        let self_link = self.as_link(true);
        
        // Only difference is that we save the link sender so we can use it to send game links
        self.connected_link_senders.insert(link.info.clone(), link.connexion_sendback.clone());

        self.connected.insert(link.info, link.message_sendback);
        
        if !link.dont_respond {
            link.connexion_sendback.send(self_link).await.unwrap();
        }
    }

    async fn handle_message(&mut self, _message: Message) {
        
    }
}

impl Game {
    pub fn new(id: u32, name: Option<String>, maximum_players: u32) -> (Self, mpsc::Sender<Link>) {
        let (user, link_sender) = User::new(id, name.unwrap_or("Game".to_string()), "game".to_string());
        let message_history = Vec::new();
        let connected = HashMap::new();
        let connected_link_senders = HashMap::new();
        
        // Set starting game state
        let game_state = GameState {
            phase: 0,
            number_of_cities: 0,
            maximum_players: maximum_players,
            number_of_players: 0
        };

        let entities = HashMap::new();
        let username_ids = BiMap::new();
        
        (Game {user, message_history, connected, connected_link_senders, game_state, entities, username_ids}, link_sender)
    }
}

