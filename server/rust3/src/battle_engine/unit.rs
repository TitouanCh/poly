use std::collections::HashMap;

use crate::battle_engine::punk_algebra::{
    vector2::PunkVector2,
    lerp::punk_lerp
};

use crate::battle_engine::{order::{Order, OrderType}, soldier::{Soldier, SoldierInfo}};
use crate::utilities::string_as_24_bytes;

use super::soldier;

#[derive(Clone)]
pub struct UnitInfo {
    pub name: String,
    pub stance: u8,
    pub speed: f32,
    pub spacing: f32,
    pub width: u8,
    pub soldiers: Vec<u8>
}

impl UnitInfo {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        // First 24 bytes are name then (in order): stance, width, speed, spacing
        bytes.extend(string_as_24_bytes(self.name.clone()));
        for value in [self.stance, self.width] {
            bytes.extend(value.clone().to_le_bytes());
        }
        for value in [self.speed, self.spacing] {
            bytes.extend(value.clone().to_le_bytes());
        }

        // TODO: Finally, add soldier types

        bytes
    }
}

#[derive(Clone)]
pub struct Unit {
    info: UnitInfo,
    n: u16,

    // Realtime
    idx: u32,
    center_of_mass: PunkVector2,
    current_angle: f32,
    current_position: PunkVector2,
    team: u8,
    incombat: bool,
    soldiers_alive: u16,
    soldiers_incombat: u16,
    orders: Vec<Order>,

    // Per soldier info
    soldiers: Vec<Soldier>
}

impl Unit {
    pub fn new(idx: u32, info: UnitInfo, position: PunkVector2, angle: f32, team: u8, soldier_compendium: &HashMap<u8, SoldierInfo>) -> Self {
        let n: u16 = info.soldiers.len().try_into().unwrap();
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
                    PunkVector2::new((i % self.info.width as u16) as f32, f32::floor(i as f32 / self.info.width as f32))
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

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        // First 32 bytes are the unit idx
        bytes.extend(self.idx.to_le_bytes());

        // Next 16 bytes are the number of soldiers in the unit
        bytes.extend(self.n.to_le_bytes());

        // Next is in order: team, position, center of mass, angle, incombat, soldiers alive, soldiers in combat
        bytes.extend(self.team.to_le_bytes());
        for vector in [self.current_position, self.center_of_mass] {
            bytes.extend(vector.to_bytes());
        }
        bytes.extend(self.current_angle.to_le_bytes());
        bytes.push(self.incombat as u8);
        for a in [self.soldiers_alive, self.soldiers_incombat] {
            bytes.extend(a.to_le_bytes());
        }
        
        // Add a seperator for good measure
        bytes.push(255);

        // Next is orders
        for order in &self.orders {
            bytes.extend(order.to_bytes());
        }

        // And finally per soldier data
        for soldier in &self.soldiers {
            bytes.extend(soldier.to_bytes());
        }
        
        bytes
    }
}