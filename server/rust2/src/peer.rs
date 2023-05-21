use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    sync::{mpsc}
};

use std::{
    net::IpAddr
};

use log::info;


use crate::connexion::Connexion;
use crate::chat::{Message};

#[derive(Clone, Debug)]
pub struct UserSender {
    pub user: String,
    pub sender: mpsc::Sender<Vec<u8>>
}

pub struct Peer {
    connexion: Connexion,
    pub username: String,
    _ip: IpAddr,
    receiver: mpsc::Receiver<Vec<u8>>,
    sender: mpsc::Sender<Vec<u8>>,
    
    global_chat_sender: Option<mpsc::Sender<Message>>
}

impl Peer {
    pub fn new(connexion: Connexion, username: String) -> Self {
        let remote_ip = connexion.socket.peer_addr().unwrap().ip();
        info!("Connection from {}", remote_ip);
        let (tx, rx) = mpsc::channel(32);

        Peer { connexion, username, _ip: remote_ip, receiver: rx, sender: tx, global_chat_sender: None}
    }

    pub async fn connect_to_global_chat(&mut self, global_chat_tx: mpsc::Sender<Message>, global_chat_connector: mpsc::Sender<UserSender>) {
        let to_send = UserSender {user: self.username.clone(), sender: self.sender.clone()};
        global_chat_connector.send(to_send).await.unwrap();
        self.global_chat_sender = Some(global_chat_tx);
    }

    pub async fn handle(&mut self) -> bool { // returns false if disconnect
        let mut buffer = [0u8; 1024];
        tokio::select! {
            received = self.receiver.recv() => {
                let mut received = received.unwrap();
                let mut end : Vec<u8> = vec![124, 101, 110, 100, 124]; //|end|
                received.append(&mut end);
                let bytes: &[u8] = &received;
                self.connexion.socket.write_all(bytes).await.unwrap();
                info!("We sent back: {:?} or {}", bytes, String::from_utf8(received.clone()).unwrap());
                true
            }

            incoming = self.connexion.socket.read(&mut buffer) => {
                let bytes_read = incoming.unwrap();

                if bytes_read == 0 {
                    return false;
                }

                let incoming = buffer[0..bytes_read].to_vec();

                match String::from_utf8(incoming[0..3].to_vec()) {
                    Ok(t) => info!("They sent: {} | {:?}", t, incoming[3..].to_vec()),
                    Err(_e) => info!("They sent: {:?}", incoming.clone())
                }

                self.interpret_bytes(incoming).await;
                true
            }
        }
    }

    async fn interpret_bytes(&self, bytes: Vec<u8>) {
        if bytes[0..3] == [103, 108, 111] {
            match self.global_chat_sender.clone() {
                Some(tx) => {
                    let mut to_send_back = vec![67];
                    to_send_back.extend(bytes[3..].to_vec());
                    tx.send(
                        Message { user: self.username.clone(), content: to_send_back }  
                    ).await.unwrap();
                }
                None => {}
            }
        }
    }

}
