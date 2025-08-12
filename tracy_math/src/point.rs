use crate::vec::Vec3D;
use tracy_macros::{Random, add, sub};

#[derive(Clone, Copy, Default, Random)]
#[sub(rhs = f64)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, Default)]
#[add(rhs = Vec3D)]
#[sub(rhs = Vec3D)]
#[sub(out = Vec3D)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}
