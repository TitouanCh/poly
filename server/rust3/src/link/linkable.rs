use async_trait::async_trait;
use tokio::sync::mpsc;
use std::collections::HashMap;
use log::info;

use crate::link::{
    userinfo::UserInfo,
    link::Link,
    message::Message,
    satellite::Satellite
};

#[async_trait]
pub trait Linkable {
    fn id(&self) -> u32;
    
    fn info(&self) -> UserInfo;

    fn message_sender(&self) -> mpsc::Sender<Message>;

    fn link_sender(&self) -> mpsc::Sender<Link>;

    fn mut_message(&mut self) -> &mut Satellite<Message>;

    fn mut_link(&mut self) -> &mut Satellite<Link>;

    fn mut_message_and_link(&mut self) -> (&mut Satellite<Message>, &mut Satellite<Link>);

    fn mut_connected(&mut self) -> &mut HashMap<UserInfo, mpsc::Sender<Message>>;

    fn add_to_history(&mut self, message: Message);

    async fn handle(&mut self) {
        let (message, link) = self.mut_message_and_link();
        tokio::select! {
            received_message = message.receiver.recv() => {
                info!("{}: Received message", self.info().to_string());
                let message = received_message.unwrap();
                self.add_to_history(message.clone());
                self.handle_message(message).await;
            }

            received_link = link.receiver.recv() => {
                let link = received_link.unwrap(); 
                info!("{}: Received new link: {}", self.info().to_string(), link.info.to_string());
                self.add_linked(link).await;
            }
        }
    }

    async fn add_linked(&mut self, link: Link) {
        let self_link = self.as_link(true);
        self.mut_connected().insert(link.info, link.message_sendback);
        
        if !link.dont_respond {
            link.connexion_sendback.send(self_link).await.unwrap();
        }
    }

    async fn handle_message(&mut self, _message: Message) {}

    fn as_link(&self, dont_respond: bool) -> Link {
        Link { 
            info: self.info(),
            message_sendback: self.message_sender(),
            connexion_sendback: self.link_sender(),
            dont_respond: dont_respond
        }
    }
}