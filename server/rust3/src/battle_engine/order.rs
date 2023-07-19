use crate::battle_engine::punk_algebra::vector2::PunkVector2;

#[derive(Clone, Copy)]
pub enum OrderType {
    Position,
    Rotate,
    Empty
}

#[derive(Clone)]
pub struct Order {
    pub what: OrderType,
    pub position: Option<PunkVector2>,
    pub angle: Option<f32>
}

impl Order {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.push(self.what as u8);
        match self.position {
            Some(vector) => {
                bytes.extend(vector.to_bytes());
            }
            None => {}
        }
        match self.angle {
            Some(a) => {
                bytes.extend(a.to_le_bytes());
            }
            None => {}
        }
        bytes
    }
}