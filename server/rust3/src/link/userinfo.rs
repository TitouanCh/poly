#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct UserInfo {
    pub name: String,
    pub id: u32,
    pub what: String
}

impl UserInfo {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn to_string(&self) -> String {
        format!("{} (id: {})", self.name, self.id)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let bytes = self.id.to_le_bytes();
        let mut bytes = bytes.to_vec();
        bytes.append(&mut self.name.clone().as_bytes().to_vec());
        bytes
    }
}
