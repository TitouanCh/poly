use async_trait::async_trait;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;
use std::{collections::HashMap, net::IpAddr};
use log::info;

use crate::link::{
    userinfo::UserInfo,
    user::User,
    link::Link,
    linkable::Linkable,
    message::Message,
    satellite::Satellite
};

use crate::connexion::Connexion;

pub struct Peer {
    user: User,
    message_history: Vec<Message>,
    _ip: IpAddr,
    connexion: Connexion,
    connected: HashMap<UserInfo, mpsc::Sender<Message>>,
    active_game: Option<UserInfo>
}

#[async_trait]
impl Linkable for Peer {
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
        let mut buffer = [0u8; 1024];
        tokio::select! {
            received_message = self.user.message.receiver.recv() => {
                info!("{}: Received message", self.info().to_string());
                let message = received_message.unwrap();
                self.add_to_history(message.clone());
                self.handle_message(message).await;
                true
            }

            received_link = self.user.link.receiver.recv() => {
                info!("{}: Received new link", self.info().to_string());
                let link = received_link.unwrap(); 
                self.add_linked(link).await;
                true
            }

            received_socket = self.connexion.socket.read(&mut buffer) => {
                let bytes_read = received_socket.unwrap();
                if bytes_read == 0 {
                    info!("{}: Has disconnected", self.info().to_string());
                    self.unlink_from_all().await;
                    return false
                }
                info!("{}: Received bytes", self.user.info.to_string());
                let bytes_read = buffer[0..bytes_read].to_vec();
                match String::from_utf8(bytes_read[0..3].to_vec()) {
                    Ok(t) => info!("They sent: {} | {:?}", t, bytes_read[3..].to_vec()),
                    Err(_e) => info!("They sent: {:?}", bytes_read.clone())
                }
                self.interpret_bytes(bytes_read).await;
                true
            }
        }
    }

    async fn add_linked(&mut self, link: Link) {
        let self_link = self.as_link(true);
        self.mut_connected().insert(link.info.clone(), link.message_sendback.clone());
        
        if !link.dont_respond {
            link.connexion_sendback.send(self_link).await.unwrap();
        }

        if link.info.what == "game" {
            info!("{}: Received game link, joining", self.info().to_string());
            self.join_game(link).await;
        }
    }

    async fn handle_message(&mut self, message: Message) {
        let mut bytes = message.as_bytes();
        let mut end : Vec<u8> = vec![124, 101, 110, 100, 124]; //|end|
        bytes.append(&mut end);
        let bytes: &[u8] = &bytes;
        self.connexion.socket.write_all(bytes).await.unwrap();
        info!("{}: We sent back: {:?} or {}", self.info().to_string(), bytes, String::from_utf8(bytes.to_vec()).unwrap());
    }
}

impl Peer {
    pub fn new(connexion: Connexion, username: String) -> (Self, mpsc::Sender<Link>) {
        let remote_ip = connexion.socket.peer_addr().unwrap().ip();
        info!("Connection from {}", remote_ip);
        let (user, link_sender) = User::new(0, username, "peer".to_string());
        let message_history = Vec::new();
        let connected = HashMap::new();
        (Peer { connexion, user, _ip: remote_ip, connected, message_history, active_game: None}, link_sender)
    }

    async fn interpret_bytes(&self, bytes: Vec<u8>) {
        // glo: send to global to chat
        if bytes[0..3] == [103, 108, 111] {
            let global_chat_sender = self.get_sender("Global".to_string(), "chat".to_string());
            match global_chat_sender {
                Some(tx) => {
                    tx.send(
                        Message { info: self.info(), bytes: bytes[3..].to_vec() }  
                    ).await.unwrap();
                }
                None => { info!("{}: Is not linked to global chat", self.info().to_string()); }
            }
        }

        // gh: send to gamehandler
        if bytes[0..2] == [103, 104] {
            let game_handler_sender = self.get_sender("GameHandler".to_string(), "game_handler".to_string());
            match game_handler_sender {
                Some(tx) => {
                    tx.send(
                        Message { info: self.info(), bytes: bytes[2..].to_vec() }  
                    ).await.unwrap();
                }
                None => { info!("{}: Is not linked to game handler", self.info().to_string()); }
            }
        }

        // rea: ready active game
        if bytes[0..3] == [114, 101, 97] {
            match &self.active_game {
                Some(info) => {
                    match self.connected.get(&info) {
                        Some(sender) => {
                            let _ = sender.send(Message {info: self.info(), bytes: vec![114, 101, 97]}).await;
                            info!("{}: Ready!", self.info().to_string())
                        }
                        None => { info!("{}: Tried to ready but no longer connected to active game", self.info().to_string()) }
                    }
                }
                None => { info!("{}: Tried to ready but no active game", self.info().to_string()) }
            }
        }

        // lau: try to launch active game
        if bytes[0..3] == [108, 97, 117] {
            match &self.active_game {
                Some(info) => {
                    match self.connected.get(&info) {
                        Some(sender) => {
                            let _ = sender.send(Message {info: self.info(), bytes: vec![108, 97, 117]}).await;
                            info!("{}: Launching!", self.info().to_string())
                        }
                        None => { info!("{}: Tried to launch but no longer connected to active game", self.info().to_string()) }
                    }
                }
                None => { info!("{}: Tried to launch but no active game", self.info().to_string()) }
            }
        }

        // lea: try to leave active game
        if bytes[0..3] == [108, 101, 97] {
            match  &self.active_game {
                Some(info) => {
                    match self.connected.get(&info) {
                        Some(sender) => {
                            let _ = sender.send(Message { info: self.info(), bytes: vec![108, 101, 97] }).await;
                            info!("{}: Attempting to leave active game {}", self.info().to_string(), info.to_string());
                        }
                        None => { info!("{}: Tried to leave but no longer connected to active game", self.info().to_string()) }
                    }
                }
                None => { info!("{}: Tried to leave but no active game", self.info().to_string()) }
            }
        }
    }

    fn get_sender(&self, name: String, what: String) -> Option<mpsc::Sender<Message>> {
        match self.connected.get(&UserInfo { name: name, id: 0, what}) {
            Some(t) => {Some(t.clone())}
            None => {None}
        }
    }

    async fn join_game(&mut self, link: Link) {
        // Send joining game to socket | j
        self.handle_message(Message { info: link.info.clone(), bytes: vec![106] }).await;
        
        // Send to the game that we are joining
        link.message_sendback.send(Message { info: self.info(), bytes: vec![106, 111, 105] }).await.unwrap();

        // Set game as active
        self.active_game = Some(link.info);
    }
}