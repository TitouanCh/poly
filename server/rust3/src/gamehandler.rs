use async_trait::async_trait;
use tokio::sync::mpsc;
use std::clone;
use std::collections::HashMap;
use log::info;

use crate::link::{
    userinfo::UserInfo,
    user::User,
    link::Link,
    linkable::Linkable,
    message::Message,
    satellite::Satellite
};

use crate::game::Game;
use crate::utilities::as_u32;

pub struct GameHandler {
    user: User,
    message_history: Vec<Message>,
    connected: HashMap<UserInfo, mpsc::Sender<Message>>,
    connected_link_senders: HashMap<UserInfo, mpsc::Sender<Link>>,
    game_links: HashMap<UserInfo, Link>,
    number_of_games: u32
}

#[async_trait]
impl Linkable for GameHandler {
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

    async fn handle_message(&mut self, message: Message) {
        //i: user request info
        if message.bytes[0] == 105 {
            info!("{}: {} requested info", self.user.info.to_string(), message.info.to_string());
            let mut bytes = vec![105];
            bytes.extend(self.info_as_bytes(20));
            self.connected.get(&message.info).unwrap().send(Message {info: self.info(), bytes} ).await.unwrap();
        }

        //j: user requests to join a game
        if message.bytes[0] == 106 {
            // first 4 bytes are game id
            let game_id = as_u32(message.bytes[1..5].to_vec());

            //self.game_connection_receivers.get(&game_id).unwrap().send(UserSender { user: command.user.clone(), sender: self.connected.get(&command.user).unwrap().clone() }).await.unwrap();
        }

        //c: user requests to create a game
        if message.bytes[0] == 99 {
            let game_id = self.number_of_games;
            // First 24 bytes are game title
            let game_title = String::from_utf8(message.bytes[1..25].to_vec()).unwrap();
            // 25th byte is the max number of players
            let maximum_players = message.bytes[25];

            let (mut game, _game_sender) = Game::new(game_id, Some(game_title), maximum_players.into());
            // Save a link to the game
            self.game_links.insert(game.info(), game.as_link(false));
            
            info!("{}: {} created a game", self.user.info.to_string(), message.info.to_string());

            self.number_of_games += 1;

            // Send game link to the creator to the game
            self.connected_link_senders.get(&message.info).unwrap().send(game.as_link(false)).await.unwrap();

            // Start up the game ---
            tokio::spawn(async move {
                loop {
                    let _ = game.handle().await;
                }
            });
        }
    }
}

impl GameHandler {
    pub fn new(id: u32, name: Option<String>) -> (Self, mpsc::Sender<Link>) {
        let (user, link_sender) = User::new(id, name.unwrap_or("GameHandler".to_string()), "game_handler".to_string());
        let message_history = Vec::new();
        let connected = HashMap::new();
        let connected_link_senders = HashMap::new();
        let game_links = HashMap::new();
        (GameHandler {user, message_history, connected, connected_link_senders, game_links, number_of_games: 0}, link_sender)
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
}