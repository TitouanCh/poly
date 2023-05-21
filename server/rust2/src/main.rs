use tokio::{
    net::{TcpListener, TcpStream},
    io::{AsyncReadExt, AsyncWriteExt},
    sync::{mpsc}
};

use std::{
    collections::HashMap,
    net::{SocketAddr, IpAddr}
};

use log::info;
use env_logger::Env;

struct Connexion {
    socket: TcpStream,
    addr: SocketAddr
}

impl Connexion {
    fn new(socket: TcpStream, addr: SocketAddr) -> Self {
        Connexion {socket, addr}
    }

    async fn get_peer(mut self) -> Peer {
        let mut buffer = [0u8; 1024];
        tokio::select!(
            result = self.socket.read(&mut buffer) => {
                let bytes_read = result.unwrap();
                let received = buffer[0..bytes_read].to_vec();
                let received = String::from_utf8(received).unwrap();
                info!("Connexion from username: {}", received.clone());
                Peer::new(self, received)
            }
        )
        
    }
}

struct Peer {
    connexion: Connexion,
    username: String,
    ip: IpAddr,
    receiver: mpsc::Receiver<Vec<u8>>,
    sender: mpsc::Sender<Vec<u8>>,
    
    global_chat_sender: Option<mpsc::Sender<Message>>
}

impl Peer {
    fn new(connexion: Connexion, username: String) -> Self {
        let remote_ip = connexion.socket.peer_addr().unwrap().ip();
        info!("Connection from {}", remote_ip);
        let (tx, rx) = mpsc::channel(32);

        Peer { connexion, username, ip: remote_ip, receiver: rx, sender: tx, global_chat_sender: None}
    }

    async fn connect_to_global_chat(&mut self, global_chat_tx: mpsc::Sender<Message>, global_chat_connector: mpsc::Sender<UserSender>) {
        let to_send = UserSender {user: self.username.clone(), sender: self.sender.clone()};
        global_chat_connector.send(to_send).await.unwrap();
        self.global_chat_sender = Some(global_chat_tx);
    }

    async fn handle(&mut self) -> bool { // returns false if disconnect
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

#[derive(Clone, Debug)]
struct Message {
    user: String,
    content: Vec<u8>
}

#[derive(Clone, Debug)]
struct UserSender {
    user: String,
    sender: mpsc::Sender<Vec<u8>>
}

struct Chat {
    id: String,
    message_history: Vec<Message>,
    message_receiver: mpsc::Receiver<Message>,
    connexion_receiver: mpsc::Receiver<UserSender>,
    connected: HashMap<String, mpsc::Sender<Vec<u8>>>
}

impl Chat {
    fn new(id: String) -> (Self, mpsc::Sender<Message>, mpsc::Sender<UserSender>) {
        let message_history = Vec::new();
        let connected = HashMap::new();
        let (tx, rx) = mpsc::channel(32);
        let (ctx, crx) = mpsc::channel(32);
        (Chat {id, message_history, message_receiver: rx, connexion_receiver: crx, connected}, tx, ctx)
    }

    fn add_sender(&mut self, user: String, tx: mpsc::Sender<Vec<u8>>) {
        self.connected.insert(user, tx);
    }
    
    async fn handle(&mut self) {
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

        for (user, peer) in &self.connected {
            info!("{}: Sent message back to {}", self.id, user);
            for to_send in [msg.content.clone(), msg.user.as_bytes().to_vec()] {
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

#[tokio::main]
async fn main() {
    // Logging setup
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);

    info!("Starting Global Chat");

    // Start global chat
    let (mut global_chat, global_chat_sender, global_chat_connector) = Chat::new("global".to_string());
    tokio::spawn(async move {
        loop {
            let _ = global_chat.handle().await;
        }
    });

    // Start listening to port 3000
    let listener: TcpListener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    info!("Listening");

    loop {
        // Accept a connexion (Tcp stream), returns socket and address
        let (socket, addr) = listener.accept().await.unwrap();
        let connexion = Connexion::new(socket, addr);

        // Get another sender for the global chat
        let global_chat_sender = global_chat_sender.clone();
        let global_chat_connector = global_chat_connector.clone();

        tokio::spawn(async move {
            let mut peer = connexion.get_peer().await;
            peer.connect_to_global_chat(global_chat_sender, global_chat_connector).await;

            loop {
                let connected = peer.handle().await;
                if !connected {
                    break
                }
            }

            info!("{} has disconnected", peer.username);
        });
    }

    //print!("{:?}", buffer);
}