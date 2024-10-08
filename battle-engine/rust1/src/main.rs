use std::{ops::{Add, Sub, Mul, Div, AddAssign}, array, collections::{HashMap}};
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

    /*
    let test = PunkVector2::new(2.0, 3.0);
    let test = test.rotated(2.0);

    info!("Punk: {}, {}", test.x, test.y);

     */

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

#[derive(Clone)]
pub struct UnitInfo {
    name: String,
    stance: i32,
    speed: f32,
    spacing: f32,
    width: i32,
    soldiers: Vec<i32>
}

#[derive(Clone, Copy)]
enum OrderType {
    Position,
    Rotate,
    Empty
}

#[derive(Clone)]
pub struct Order {
    what: OrderType,
    position: Option<PunkVector2>,
    angle: Option<f32>
}

#[derive(Clone)]
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
    soldiers_incombat: i32,
    orders: Vec<Order>,

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
        let soldiers_incombat = 0;
        let orders = Vec::new();
        let mut soldiers = Vec::new();

        for soldier_variety in &info.soldiers {
            soldiers.push(Soldier::new(soldier_compendium.get(soldier_variety).unwrap().clone(), position));
        }

        Unit {info, n, idx, center_of_mass, current_angle, current_position, team, incombat, soldiers_alive, soldiers_incombat, orders, soldiers}
    }

    pub fn setup(&mut self) {
        // Set soldier positions
        let soldier_positions = self.get_soldier_positions_at(self.current_position, self.current_angle);
        for i in 0..self.n {
            let l = i as usize;
            self.soldiers[l].position = soldier_positions[l];
        }
    }

    pub fn process(&mut self, delta: f32, other_units: &HashMap<i32, Unit>) {
        self.movement(delta);
        self.order_check(delta);
    }

    pub fn movement(&mut self, delta: f32) {
        let mut sum = PunkVector2::zero();
        let deaccel_epsilon = 50.0;

        for soldier in &mut self.soldiers {
            if soldier.incombat && soldier.alive {
                // Combat mvt
            } else {
                // Regular mvt
                let direction = soldier.target_position - soldier.position;
                let distance = direction.length();
                let direction = direction.normalized();
                let mut speed_modifier = self.info.speed;
                if distance < deaccel_epsilon {
                    speed_modifier = f32::max(punk_lerp(0.0, self.info.speed, distance/deaccel_epsilon), 1.0);
                }
                // soldier.speed = direction * speed_modifier * delta - soldier.position;
                soldier.position += direction * speed_modifier * delta;
                sum += soldier.position;
            }
        }

        self.center_of_mass = sum / (self.soldiers_alive - self.soldiers_incombat) as f32
    }

    pub fn order_check(&mut self, _delta: f32) {
        let order_epsilon = 20.0;
        let mut sum = 0.0;

        for soldier in &self.soldiers {
            if soldier.alive && !soldier.incombat {
                sum += soldier.position.distance_to(soldier.target_position);
            }
        }
        
        if sum < order_epsilon {
            self.queue_next_order();
        }
    }

    pub fn queue_next_order(&mut self) {
        if self.orders.len() > 0 {
            // Check previous order
            match self.orders[0].what {
                OrderType::Position => {
                    self.current_position = self.orders[0].position.unwrap();
                }
                OrderType::Rotate => {
                    self.current_angle = self.orders[0].angle.unwrap();
                }
                OrderType::Empty => {}
            }
            // Remove previous order from queue
            self.orders.remove(0);

            if self.orders.len() > 0 {
                // Start next order
                match self.orders[0].what {
                    OrderType::Position => {
                        self.assign_target_positions_to_soldiers(self.orders[0].position.unwrap(), self.current_angle);
                    }
                    OrderType::Rotate => {
                        self.assign_target_positions_to_soldiers(self.current_position, self.orders[0].angle.unwrap());
                    }
                    OrderType::Empty => {}
                }
            }
        }

        if self.orders.len() == 0 {
            self.orders.push(Order{ what: OrderType::Empty, position: None, angle: None});
        }
    }

    pub fn assign_target_positions_to_soldiers(&mut self, position: PunkVector2, angle: f32) {
        let targets = self.get_soldier_positions_at(position, angle);

        for i in 0..self.n {
            let l = i as usize;
            self.soldiers[l].target_position = targets[l];
        }
    }

    pub fn get_soldier_positions_at(&self, position: PunkVector2, angle: f32) -> Vec<PunkVector2> {
        let mut soldier_positions = Vec::new();

        for i in 0..self.n {
            // Offset from center
            let soldier_position = 
                (
                    PunkVector2::new((i % self.info.width) as f32, f32::floor(i as f32 / self.info.width as f32))
                    - PunkVector2::new((self.info.width as f32 - 1.0 ) / 2.0, (f32::ceil(self.n as f32 / self.info.width as f32) - 1.0) / 2.0)
                ) * self.info.spacing
            ;

            // Rotation
            let soldier_position = soldier_position.rotated(angle);

            // Offset from unit position
            let soldier_position = soldier_position + position;

            soldier_positions.push(soldier_position);
        }

        soldier_positions
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

#[derive(Clone)]
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

impl Mul<PunkVector2> for PunkVector2 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self { x: self.x * other.x, y: self.y * other.y }
    }
}

impl Mul<f32> for PunkVector2 {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self { x: self.x * other, y: self.y * other }
    }
}

impl Div<PunkVector2> for PunkVector2 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self { x: self.x / other.x, y: self.y / other.y }
    }
}

impl Div<f32> for PunkVector2 {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        Self { x: self.x / other, y: self.y / other }
    }
}

impl AddAssign for PunkVector2 {
    fn add_assign(&mut self, other: Self) {
        *self = PunkVector2{ x: self.x + other.x, y: self.y + other.y};
    }
}

impl PunkVector2 {
    pub fn new(x: f32, y: f32) -> Self {
        PunkVector2 { x, y }
    }

    pub fn zero() -> Self {
        PunkVector2::new(0.0, 0.0)
    }

    pub fn length(&self) -> f32 {
        f32::sqrt(f32::powi(self.x, 2) + f32::powi(self.y, 2))
    }

    pub fn normalized(&self) -> PunkVector2 {
        let length = self.length();
        PunkVector2 { x : self.x / length, y: self.y / length }
    }

    pub fn rotated(&self, angle: f32) -> PunkVector2 {
        let x = f32::cos(angle)  * self.x - f32::sin(angle) * self.y;
        let y = f32::sin(angle)  * self.x + f32::cos(angle) * self.y;
        PunkVector2 { x, y }
    }

    pub fn distance_to(&self, point: PunkVector2) -> f32 {
        f32::sqrt(f32::powi(self.x - point.x, 2) + f32::powi(self.y - point.y, 2))
    }
}

pub fn punk_lerp(a: f32, b: f32, t: f32) -> f32 {
    a * (1.0 - t) + b * t
}