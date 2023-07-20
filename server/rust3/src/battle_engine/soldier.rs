use std::vec;

use super::punk_algebra::vector2::PunkVector2;
use crate::utilities::string_as_24_bytes;

#[derive(Clone)]
pub struct SoldierInfo {
    pub name: String,
    pub variety: u8,
    pub attack: f32,
    pub defense: f32,
    pub health: f32,
    pub mass: f32,
}

impl SoldierInfo {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(string_as_24_bytes(self.name.clone()));
        bytes.extend(self.variety.to_le_bytes());
        for a in [self.attack, self.defense, self.health, self.mass] {
            bytes.extend(a.to_le_bytes());
        }
        bytes
    }
}

#[derive(Clone)]
pub struct Soldier {
    pub info: SoldierInfo,
    pub position: PunkVector2,
    pub target_position: PunkVector2,
    pub combat_position: PunkVector2,
    pub incombat: bool,
    pub opponent: [u32; 2],
    pub alive: bool
}

impl Soldier {
    pub fn new(info: SoldierInfo, position: PunkVector2) -> Self {
        Soldier {info, position, target_position: PunkVector2::zero(), combat_position: PunkVector2::zero(), incombat: false, opponent: [0, 0], alive: true }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for vector in [self.position, self.target_position, self.combat_position] {
            bytes.extend(vector.to_bytes());
        }
        for bool in [self.incombat, self.alive] {
            bytes.push(bool as u8);
        }
        bytes.extend(self.opponent[0].to_le_bytes());
        bytes.extend(self.opponent[1].to_le_bytes());
        bytes
    }
}