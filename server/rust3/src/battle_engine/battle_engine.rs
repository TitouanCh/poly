use::std::collections::HashMap;
use log::info;

use crate::{battle_engine::{soldier::SoldierInfo, unit::{Unit, UnitInfo}, punk_algebra::vector2::PunkVector2}, link::userinfo::UserInfo};

pub struct BattleEngine {
    soldier_compendium: HashMap<u8, SoldierInfo>,
    unit_compendium: HashMap<u8, UnitInfo>,
    units: HashMap<u32, Unit>,
}

impl BattleEngine {
    pub fn new() -> Self {
        let units = HashMap::new();
        let mut unit_compendium = HashMap::new();
        unit_compendium.insert(0, UnitInfo {name: "Swordmen".to_string(), stance: 0, speed: 100.0, spacing: 50.0, width: 6, soldiers: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]});
        let mut soldier_compendium = HashMap::new();
        soldier_compendium.insert(0, SoldierInfo {name: "Swordman".to_string(), attack: 2.0, defense: 2.0, health: 100.0, variety: 0, mass: 8.0});
        BattleEngine { units, soldier_compendium, unit_compendium }
    }

    pub fn ready(&mut self) {
        self.add_unit(0, 9, PunkVector2::new(0.0, 0.0), 45.0);
        self.add_unit(0, 0, PunkVector2::new(2000.0, 2000.0), 0.0);
        info!("{}: Ready", "Battle Engine".to_string());
    }

    pub fn process(&mut self, delta: f32) {
        for (idx, unit) in &mut self.units {
            unit.process(delta, &HashMap::new());
        }
    }

    pub fn process_by_intervall(&mut self, delta: f32, intervall: f32) {
        let mut sum = 0.0;
        while sum < delta {
            self.process(intervall);
            sum += intervall;
        }
        info!("{}: Processed {}s ({}s)", "Battle Engine".to_string(), delta, intervall);
    }

    pub fn add_unit(&mut self, variety: u8, team: u8, position: PunkVector2, angle: f32) {
        let idx: u32 = self.units.len().try_into().unwrap();
        let mut unit = Unit::new(idx, self.unit_compendium.get(&variety).unwrap().clone(), position, angle, team, &self.soldier_compendium);
        unit.setup();
        self.units.insert(idx, unit);
    }

    // Gives a list of packets to send, each unit is it's own packet because the packets are big af
    pub fn units_as_bytes(&self, _player: u8) -> Vec<Vec<u8>> {
        let mut bytes = Vec::new();
        for (_, unit) in &self.units {
            bytes.push(unit.to_bytes());
        }
        bytes
    }
}