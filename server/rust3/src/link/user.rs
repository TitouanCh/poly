use tokio::sync::mpsc;

use crate::link::{
    userinfo::UserInfo,
    link::Link,
    message::Message,
    satellite::Satellite
};

pub struct User {
    pub info: UserInfo,
    pub message: Satellite<Message>,
    pub link: Satellite<Link>,
}

impl User {
    pub fn new(id: u32, name: String) -> (Self, mpsc::Sender<Link>) {
        let info = UserInfo {id, name};
        let message = Satellite::new();
        let link = Satellite::new();
        let link_sender = link.sender.clone();
        (User { info, message, link }, link_sender)
    }
}
