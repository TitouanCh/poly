use::std::collections::HashMap;
use crate::battle_engine::{soldier::SoldierInfo, unit::{Unit, UnitInfo}, punk_algebra::vector2::PunkVector2};

pub struct BattleEngine {
    soldier_compendium: HashMap<i32, SoldierInfo>,
    unit_compendium: HashMap<i32, UnitInfo>,
    units: HashMap<i32, Unit>,
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

    pub fn process(&mut self, delta: f32) {
        for (idx, unit) in &mut self.units {
            unit.process(delta, &HashMap::new());
        }
    }

    pub fn add_unit(&mut self, variety: i32, team: i32, position: PunkVector2, angle: f32) {
        let idx: i32 = self.units.len().try_into().unwrap();
        let mut unit = Unit::new(idx, self.unit_compendium.get(&variety).unwrap().clone(), position, angle, team, &self.soldier_compendium);
        unit.setup();
        self.units.insert(idx, unit);
    }
}