use crate::vec::Vec3D;
use num_traits::Num;
use rand::Rng;
use tracy_macros::{Random, add, sub};

#[derive(Clone, Copy, Default, Random)]
#[sub(rhs = T)]
pub struct Point2D<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point2D<T>
where
    T: Num,
{
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

impl<T> Point3D<T>
where
    T: Num,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}
