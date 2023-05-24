use crate::link::userinfo::UserInfo;

#[derive(Clone)]
pub struct Message {
    pub info: UserInfo,
    pub bytes: Vec<u8>
}