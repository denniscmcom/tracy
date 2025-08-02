use std::ops;
use tracy_macros::{add, div, mul, sub};

#[derive(Clone, Copy, Default)]
#[add]
#[sub]
#[mul(rhs = T)]
#[div(rhs = T)]
pub struct Vec3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3D<T>
where
    T: Copy + ops::Add<Output = T> + ops::Mul<Output = T>,
{
    pub fn len_2(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, v: &Vec3D<T>) -> T {
        self.x * v.x + self.y * v.y + self.z * v.z
    }
}
