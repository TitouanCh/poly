use crate::link::userinfo::UserInfo;

#[derive(Clone, Debug)]
pub struct Message {
    pub info: UserInfo,
    pub bytes: Vec<u8>
}

impl Message {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = self.info.as_bytes();
        bytes.append(&mut "|user|".as_bytes().to_vec());
        bytes.append(&mut self.bytes.clone());
        bytes
    }
}