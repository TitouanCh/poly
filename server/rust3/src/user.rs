use tokio::sync::mpsc;

use std::collections::HashMap;

use log::info;

#[derive(Clone)]
pub struct Message {
    info: UserInfo,
    bytes: Vec<u8>
}
#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct UserInfo {
    username: String
}

#[derive(Debug)]
pub struct Link {
    info: UserInfo,
    message_sendback: mpsc::Sender<Message>,
    connexion_sendback: mpsc::Sender<Link>,
    dont_respond: bool
}

struct MessageSatellite {
    receiver: mpsc::Receiver<Message>,
    sender: mpsc::Sender<Message>,
}

struct LinkSatellite {
    receiver: mpsc::Receiver<Link>,
    sender: mpsc::Sender<Link>
}

struct User {
    info: UserInfo,
    message: MessageSatellite,
    link: LinkSatellite,
}

struct UserHandler {
    id: String,
    info: UserInfo,
    message_history: Vec<Message>,
    message: MessageSatellite,
    link: LinkSatellite,
    connected: HashMap<UserInfo, mpsc::Sender<Message>>
}


// turn this into trait linkable
impl UserHandler {
    pub async fn handle(&mut self) {
        tokio::select! {
            received_message = self.message.receiver.recv() => {
                info!("{}: Received message", self.id);
                let message = received_message.unwrap();
                self.message_history.push(message.clone());
                self.handle_message(message).await;
            }

            received_link = self.link.receiver.recv() => {
                info!("{}: Received new link", self.id);
                let link = received_link.unwrap(); 
                self.add_linked(link).await;
            }
        }
    }

    async fn add_linked(&mut self, link: Link) {
        self.connected.insert(link.info, link.message_sendback);
        if !link.dont_respond {
            link.connexion_sendback.send(self.as_link(true)).await.unwrap();
        }
    }

    async fn handle_message(&mut self, message: Message) {
        
    }

    fn as_link(&self, dont_respond: bool) -> Link {
        Link { 
            info: self.info.clone(),
            message_sendback: self.message.sender.clone(),
            connexion_sendback: self.link.sender.clone(),
            dont_respond: dont_respond
        }
    }
}