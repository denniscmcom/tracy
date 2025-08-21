use std::ops::Neg;
use tracy_macros::{Random, add, div, mul, sub};

// TODO: Add a separate type for unit vector.

#[derive(Clone, Copy, Default, Random)]
#[add]
#[sub]
#[mul(lhs = f64, rhs = f64)]
#[div(lhs = f64, rhs = f64)]
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

    pub fn cross(&self, v: &Vec3D) -> Vec3D {
        let x = self.y * v.z - self.z * v.y;
        let y = self.z * v.x - self.x * v.z;
        let z = self.x * v.y - self.y * v.x;
        Vec3D::new(x, y, z)
    }

    pub fn normalize(&self) -> Vec3D {
        *self / self.len_2().sqrt()
    }
}

// TODO: Move Neg to a Derive macro.
impl Neg for Vec3D {
    type Output = Vec3D;

    fn neg(self) -> Self::Output {
        Vec3D::new(-self.x, -self.y, -self.z)
    }
}

#[derive(Clone, Copy, Default, Random)]
#[add]
#[sub]
#[mul(lhs = f64)]
#[div(lhs = f64)]
pub struct Vec2D {
    pub x: f64,
    pub y: f64,
}

impl Vec2D {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn len_2(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }
}

impl Neg for Vec2D {
    type Output = Vec2D;

    fn neg(self) -> Self::Output {
        Vec2D::new(-self.x, -self.y)
    }
}

pub mod benchmarks {
    use super::*;

    pub fn vec3d_len_2() -> impl Fn() {
        let v = Vec3D::new(1.0, 2.0, 3.0);
        move || {
            v.len_2();
        }
    }

    pub fn vec3d_dot() -> impl Fn() {
        let v = Vec3D::new(1.0, 2.0, 3.0);
        let u = Vec3D::new(4.0, 5.0, 6.0);
        move || {
            v.dot(&u);
        }
    }

    pub fn vec3d_cross() -> impl Fn() {
        let v = Vec3D::new(1.0, 2.0, 3.0);
        let u = Vec3D::new(4.0, 5.0, 6.0);
        move || {
            v.cross(&u);
        }
    }

    pub fn vec3d_normalize() -> impl Fn() {
        let v = Vec3D::new(1.0, 2.0, 3.0);
        move || {
            v.normalize();
        }
    }

    pub fn vec3d_random() -> impl Fn() {
        move || {
            Vec3D::random();
        }
    }

    pub fn vec3d_random_range() -> impl Fn() {
        move || {
            Vec3D::random_range(0.0..1.0);
        }
    }

    pub fn vec2d_len_2() -> impl Fn() {
        let v = Vec2D::new(1.0, 2.0);
        move || {
            v.len_2();
        }
    }

    pub fn vec2d_random() -> impl Fn() {
        move || {
            Vec2D::random();
        }
    }

    pub fn vec2d_random_range() -> impl Fn() {
        move || {
            Vec2D::random_range(0.0..1.0);
        }
    }
}
