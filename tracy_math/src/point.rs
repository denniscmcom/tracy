use crate::vec::Vec3D;
use tracy_macros::{add, sub};

#[derive(Clone, Copy, Default)]
pub struct Point2D<T> {
    pub x: T,
    pub y: T,
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
