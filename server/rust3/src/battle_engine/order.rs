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