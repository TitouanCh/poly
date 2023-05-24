use async_trait::async_trait;
use tokio::sync::mpsc;
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

pub struct Chat {
    user: User,
    message_history: Vec<Message>,
    connected: HashMap<UserInfo, mpsc::Sender<Message>>
}

#[async_trait]
impl Linkable for Chat {
    fn id(&self) -> u32 { self.user.info.id.clone() }
    fn info(&self) -> UserInfo { self.user.info.clone() }
    fn mut_message(&mut self) -> &mut Satellite<Message> { &mut self.user.message}
    fn mut_link(&mut self) -> &mut Satellite<Link> { &mut self.user.link }
    fn mut_message_and_link(&mut self) -> (&mut Satellite<Message>, &mut Satellite<Link>) { (&mut self.user.message, &mut self.user.link) }
    fn mut_connected(&mut self) -> &mut HashMap<UserInfo,mpsc::Sender<Message> > { &mut self.connected }
    
    fn add_to_history(&mut self, message:Message) {
        self.message_history.push(message);
    }

    async fn handle_message(&mut self, message: Message) {
        self.send_message_to_all(message).await;
    }
}

impl Chat {
    pub fn new(id: u32) -> (Self, mpsc::Sender<Link>) {
        let (user, link_sender) = User::new(id, "Chat".to_string());
        let message_history = Vec::new();
        let connected = HashMap::new();
        (Chat {user, message_history, connected}, link_sender)
    }

    async fn send_message_to_all(&mut self, message: Message) {
        let mut users_to_remove = Vec::new();

        // Add unique identifier to content start
        let mut message = message.clone();
        let mut send_back_bytes = vec![103];
        send_back_bytes.extend(message.bytes);
        
        message.bytes = send_back_bytes;

        // Send the message to every user connected to the chat
        for (user, peer) in &self.connected {
            match peer.send(message.clone()).await {
                Ok(_) => {info!("{}: Sent message back to {}", self.user.info.to_string(), user.to_string());},
                Err(_) => {users_to_remove.push(user.clone());}
            };
        }

        for user in users_to_remove.iter() {
            self.connected.remove_entry(user);
            info!("{}: {} appears to be disconnected, he was removed from the chat", self.user.info.to_string(), user.to_string());
        }
    }
}