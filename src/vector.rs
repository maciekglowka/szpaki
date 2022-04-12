use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

// Vector2

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
        if l == 0.0 {return};
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
        (dx*dx + dy*dy).sqrt()
    }
    pub fn angle(&self, other: Vector2) -> f32 {
        (other.y * self.x - other.x * self.y).atan2(other.x * self.x + other.y * self.y)
    }
    pub fn rotate(&mut self, angle: f32) {
        let cs = angle.cos();
        let sn = angle.sin();
        let x = self.x * cs - self.y * sn;
        let y = self.x * sn + self.y * cs;
        self.x = x;
        self.y = y;
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

// Vector3

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3{x: x, y: y, z: z}
    }
    pub fn len(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn normalize(&mut self) {
        let l = self.len();
        if l == 0.0 {return};
        self.x = self.x / l;
        self.y = self.y / l;
        self.z = self.z / l;
    }
    pub fn normalized(&self) -> Vector3 {
        let mut v = *self;
        v.normalize();
        v
    }
    pub fn clamp(&mut self, max: f32) {
        if self.len() < max {return}
        self.normalize();
        self.x = self.x * max;
        self.y = self.y * max;
        self.z = self.z * max;
    }
    pub fn dist(&self, other: Vector3) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let dz = other.z - self.z;
        (dx*dx + dy*dy + dz*dz).sqrt()
    }
    pub fn dot(&self, other: Vector3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn angle(&self, other: Vector3) -> f32 {
        let cs = self.dot(other) / (self.len() * other.len());
        cs.acos()
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        return Vector3::new(self.x + other.x, self.y + other.y, self.z + other.z);
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self{x: self.x + other.x, y: self.y + other.y, z: self.z + other.z};
    }
}

impl Div<f32> for Vector3 {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        return Vector3::new(self.x / other, self.y / other, self.z / other)
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        return Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl SubAssign for Vector3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self{x: self.x - other.x, y: self.y - other.y, z: self.z - other.z};
    }
}

impl Mul<Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3 {
        return Vector3::new(self * other.x, self * other.y, self * other.z)
    }
}