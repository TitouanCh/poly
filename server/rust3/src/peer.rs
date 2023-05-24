use async_trait::async_trait;
use tokio::io::AsyncReadExt;
use tokio::sync::mpsc;
use std::{collections::HashMap, net::IpAddr};
use log::info;

use crate::link::message;
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
    connected: HashMap<UserInfo, mpsc::Sender<Message>>
}

#[async_trait]
impl Linkable for Peer {
    fn id(&self) -> u32 { self.user.info.id.clone() }
    fn info(&self) -> UserInfo { self.user.info.clone() }
    fn mut_message(&mut self) -> &mut Satellite<Message> { &mut self.user.message}
    fn mut_link(&mut self) -> &mut Satellite<Link> { &mut self.user.link }
    fn mut_message_and_link(&mut self) -> (&mut Satellite<Message>, &mut Satellite<Link>) { (&mut self.user.message, &mut self.user.link) }
    fn mut_connected(&mut self) -> &mut HashMap<UserInfo,mpsc::Sender<Message> > { &mut self.connected }
    
    fn add_to_history(&mut self, message:Message) {
        self.message_history.push(message);
    }

    async fn handle(&mut self) {
        let mut buffer = [0u8; 1024];
        tokio::select! {
            received_message = self.user.message.receiver.recv() => {
                info!("{}: Received message", self.info().to_string());
                let message = received_message.unwrap();
                self.add_to_history(message.clone());
                self.handle_message(message).await;
            }

            received_link = self.user.link.receiver.recv() => {
                info!("{}: Received new link", self.info().to_string());
                let link = received_link.unwrap(); 
                self.add_linked(link).await;
            }

            received_socket = self.connexion.socket.read(&mut buffer) => {
                let bytes_read = received_socket.unwrap();
                if bytes_read == 0 {
                    // Disconnected
                }
                info!("{}: Received bytes", self.user.info.to_string());
                let bytes_read = buffer[0..bytes_read].to_vec();
                match String::from_utf8(bytes_read[0..3].to_vec()) {
                    Ok(t) => info!("They sent: {} | {:?}", t, bytes_read[3..].to_vec()),
                    Err(_e) => info!("They sent: {:?}", bytes_read.clone())
                }
                self.interpret_bytes(bytes_read).await;
            }
        }
    }

    async fn handle_message(&mut self, message: Message) {}
}

impl Peer {
    pub fn new(connexion: Connexion, username: String) -> Self {
        let remote_ip = connexion.socket.peer_addr().unwrap().ip();
        info!("Connection from {}", remote_ip);
        let (user, link_sender) = User::new(0, username);
        let message_history = Vec::new();
        let connected = HashMap::new();
        Peer { connexion, user, _ip: remote_ip, connected, message_history}
    }

    async fn interpret_bytes(&self, bytes: Vec<u8>) {
        /*
        if bytes[0..3] == [103, 108, 111] {
            match self.global_chat_sender.clone() {
                Some(tx) => {
                    tx.send(
                        Message { user: self.username.clone(), content: bytes[3..].to_vec() }  
                    ).await.unwrap();
                }
                None => {}
            }
        }
         */
    }
}