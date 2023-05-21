use tokio::{
    sync::mpsc
};

use std::{
    collections::HashMap,
};

use log::info;

use crate::peer::UserSender;

#[derive(Clone, Debug)]
pub struct Message {
    pub user: String,
    pub content: Vec<u8>
}

pub struct Chat {
    id: String,
    message_history: Vec<Message>,
    message_receiver: mpsc::Receiver<Message>,
    connexion_receiver: mpsc::Receiver<UserSender>,
    connected: HashMap<String, mpsc::Sender<Vec<u8>>>
}

impl Chat {
    pub fn new(id: String) -> (Self, mpsc::Sender<Message>, mpsc::Sender<UserSender>) {
        let message_history = Vec::new();
        let connected = HashMap::new();
        let (tx, rx) = mpsc::channel(32);
        let (ctx, crx) = mpsc::channel(32);
        (Chat {id, message_history, message_receiver: rx, connexion_receiver: crx, connected}, tx, ctx)
    }

    fn add_sender(&mut self, user: String, tx: mpsc::Sender<Vec<u8>>) {
        self.connected.insert(user, tx);
    }
    
    pub async fn handle(&mut self) {
        tokio::select! {
            received_chat = self.message_receiver.recv() => {
                info!("{}: Received chat message", self.id);
                let message = received_chat.unwrap();
                self.message_history.push(message.clone());
                self.send_message_to_all(message).await;
            }

            received_connexion = self.connexion_receiver.recv() => {
                info!("{}: Received connexion", self.id);
                let info = received_connexion.unwrap(); 
                self.add_sender(info.user, info.sender);
            }
        }
    }

    async fn send_message_to_all(&mut self, msg: Message) {
        let mut users_to_remove = Vec::new();

        // Add unique identifier to content start
        let mut to_send_back = vec![103];
        to_send_back.extend(msg.content);
        
        // Send the message to every user connected to the chat
        for (user, peer) in &self.connected {
            info!("{}: Sent message back to {}", self.id, user);
            for to_send in [to_send_back.clone(), msg.user.as_bytes().to_vec()] {
                match peer.send(to_send).await {
                    Ok(_) => {},
                    Err(_) => {
                        users_to_remove.push(user.clone());
                    }
                };
            }
        }

        for user in users_to_remove.iter() {
            self.connected.remove_entry(user);
            info!("{}: {} appears to be disconnected, he was removed from the chat", self.id, user);
        }
    }
}