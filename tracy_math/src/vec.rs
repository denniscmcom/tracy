use tracy_macros::{Random, add, div, mul, sub};

#[derive(Clone, Copy, Default, Random)]
#[add]
#[sub]
#[mul(rhs = f64)]
#[div(rhs = f64)]
pub struct Vec3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl Vec3D {
    pub fn len_2(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, v: &Vec3D) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }
}
