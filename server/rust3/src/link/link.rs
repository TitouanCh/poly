use tokio::sync::mpsc;
use crate::link::{
    userinfo::UserInfo,
    message::Message
};

#[derive(Debug)]
pub struct Link {
    pub info: UserInfo,
    pub message_sendback: mpsc::Sender<Message>,
    pub connexion_sendback: mpsc::Sender<Link>,
    pub dont_respond: bool
}