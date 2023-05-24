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

pub struct Game {
    user: User,
    message_history: Vec<Message>,
    connected: HashMap<UserInfo, mpsc::Sender<Message>>
}