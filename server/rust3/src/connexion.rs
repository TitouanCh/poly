use tokio::{
    net::TcpStream,
    io::AsyncReadExt,
    sync::mpsc
};

use std::{
    net::SocketAddr
};

use log::info;

use crate::peer::Peer;

use crate::link::link::Link;

pub struct Connexion {
    pub socket: TcpStream,
    pub addr: SocketAddr
}

impl Connexion {
    pub fn new(socket: TcpStream, addr: SocketAddr) -> Self {
        Connexion {socket, addr}
    }

    pub async fn get_peer(mut self) -> (Peer, mpsc::Sender<Link>) {
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
