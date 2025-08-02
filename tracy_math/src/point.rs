use crate::vec::Vec3D;
use tracy_macros::{add, sub};

#[derive(Clone, Copy, Default)]
pub struct Point2D<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, Default)]
#[add(rhs = Vec3D<T>)]
#[sub(rhs = Vec3D<T>)]
#[sub(out = Vec3D<T>)]
pub struct Point3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Point3D<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}
