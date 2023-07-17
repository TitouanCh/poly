use std::{ops::{Add, Sub}, array, collections::{HashMap}};
use log::info;
use env_logger::Env;

fn main() {
    // Logging setup
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);

    info!("Starting!!!");

    let mut battle_engine = BattleEngine::new();
    battle_engine.add_unit(0, 0, PunkVector2::zero(), 0.0);

    info!("First steps!!");
}

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

    pub fn process(delta: f32) {
         
    }

    pub fn add_unit(&mut self, variety: i32, team: i32, position: PunkVector2, angle: f32) {
        let idx: i32 = self.units.len().try_into().unwrap();
        let unit = Unit::new(idx, self.unit_compendium.get(&variety).unwrap().clone(), position, angle, team, &self.soldier_compendium);
        self.units.insert(idx, unit);
    }
}

#[derive(Clone)]
pub struct UnitInfo {
    name: String,
    stance: i32,
    speed: f32,
    spacing: f32,
    width: i32,
    soldiers: Vec<i32>
}

pub struct Unit {
    info: UnitInfo,
    n: i32,

    // Realtime
    idx: i32,
    center_of_mass: PunkVector2,
    current_angle: f32,
    current_position: PunkVector2,
    team: i32,
    incombat: bool,
    soldiers_alive: i32,

    // Per soldier info
    soldiers: Vec<Soldier>
}

impl Unit {
    pub fn new(idx: i32, info: UnitInfo, position: PunkVector2, angle: f32, team: i32, soldier_compendium: &HashMap<i32, SoldierInfo>) -> Self {
        let n: i32 = info.soldiers.len().try_into().unwrap();
        let center_of_mass = position;
        let current_position: PunkVector2 = position;
        let current_angle = angle;
        let team = team;
        let incombat = false;
        let soldiers_alive = n;
        let mut soldiers = Vec::new();

        for soldier_variety in &info.soldiers {
            soldiers.push(Soldier::new(soldier_compendium.get(soldier_variety).unwrap().clone(), position));
        }

        Unit {info, n, idx, center_of_mass, current_angle, current_position, team, incombat, soldiers_alive, soldiers}
    }
}

#[derive(Clone)]
pub struct SoldierInfo {
    name: String,
    variety: i32,
    attack: f32,
    defense: f32,
    health: f32,
    mass: f32,
}

pub struct Soldier {
    info: SoldierInfo,
    position: PunkVector2,
    target_position: PunkVector2,
    combat_position: PunkVector2,
    incombat: bool,
    opponent: [i32; 2],
    alive: bool
}

impl Soldier {
    pub fn new(info: SoldierInfo, position: PunkVector2) -> Self {
        Soldier {info, position, target_position: PunkVector2::zero(), combat_position: PunkVector2::zero(), incombat: false, opponent: [0, 0], alive: true }
    }
}

#[derive(Copy, Clone)]
pub struct PunkVector2 {
    x: f32,
    y: f32
}

impl Add for PunkVector2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { x: self.x + other.x, y: self.y + other.y }
    }
}

impl Sub for PunkVector2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {x: self.x - other.x, y: self.y - other.y}
    }
}

impl PunkVector2 {
    pub fn new(x: f32, y: f32) -> Self {
        PunkVector2 { x, y }
    }

    pub fn zero() -> Self {
        PunkVector2::new(0.0, 0.0)
    }
}