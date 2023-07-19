use super::punk_algebra::vector2::PunkVector2;

#[derive(Clone)]
pub struct SoldierInfo {
    name: String,
    variety: i32,
    attack: f32,
    defense: f32,
    health: f32,
    mass: f32,
}

#[derive(Clone)]
pub struct Soldier {
    pub info: SoldierInfo,
    pub position: PunkVector2,
    pub target_position: PunkVector2,
    pub combat_position: PunkVector2,
    pub incombat: bool,
    pub opponent: [i32; 2],
    pub alive: bool
}

impl Soldier {
    pub fn new(info: SoldierInfo, position: PunkVector2) -> Self {
        Soldier {info, position, target_position: PunkVector2::zero(), combat_position: PunkVector2::zero(), incombat: false, opponent: [0, 0], alive: true }
    }
}