use std::ops::{Add, Sub, Mul, Div, AddAssign};

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
        PunkVector2{ x, y }
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

    pub fn to_bytes(&self) -> Vec<u8> {
        // 4 + 4 = 8 bytes
        // x: f32, y: f32
        let mut bytes = Vec::new();
        for a in [self.x, self.y] {
            bytes.extend(a.to_le_bytes());
        }
        bytes
    }
}