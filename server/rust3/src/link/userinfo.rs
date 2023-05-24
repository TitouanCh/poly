#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct UserInfo {
    pub name: String,
    pub id: u32
}

impl UserInfo {
    pub fn to_string(&self) -> String {
        format!("{} (id: {})", self.name, self.id)
    }
}