use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2{x: x, y: y}
    }
    pub fn len(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    pub fn normalize(&mut self) {
        let l = self.len();
        self.x = self.x / l;
        self.y = self.y / l;
    }
    pub fn clamp(&mut self, max: f32) {
        if self.len() < max {return}
        self.normalize();
        self.x = self.x * max;
        self.y = self.y * max;
    }
    pub fn dist(&self, other: Vector2) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        (dx*dx + dy+dy).sqrt()
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        return Vector2::new(self.x + other.x, self.y + other.y);
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, other: Self) {
        *self = Self{x: self.x + other.x, y: self.y + other.y};
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        return Vector2::new(self.x - other.x, self.y - other.y)
    }
}

impl SubAssign for Vector2 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self{x: self.x - other.x, y: self.y - other.y};
    }
}

impl Div<f32> for Vector2 {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        return Vector2::new(self.x / other, self.y / other)
    }
}

impl Mul<f32> for Vector2 {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        return Vector2::new(self.x * other, self.y * other)
    }
}