use crate::vec::{Vec2D, Vec3D};
use tracy_macros::{Neg, Random, add, sub};

#[derive(Clone, Copy, Default, Random, Neg)]
#[add(rhs = Vec3D, lhs = Vec3D)]
#[sub(rhs = Vec3D, lhs = Vec3D)]
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

#[derive(Clone, Copy, Default, Random, Neg)]
#[add(rhs = Vec2D, lhs = Vec2D)]
#[sub(rhs = Vec2D, lhs = Vec2D)]
#[sub(out = Vec2D)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

pub mod benchmarks {
    use super::*;

    pub fn point_3d_random() -> impl Fn() {
        move || {
            Point3D::random();
        }
    }

    pub fn point_3d_random_range() -> impl Fn() {
        move || {
            Point3D::random_range(0.0..1.0);
        }
    }

    pub fn point_2d_random() -> impl Fn() {
        move || {
            Point2D::random();
        }
    }

    pub fn point_2d_random_range() -> impl Fn() {
        move || {
            Point2D::random_range(0.0..1.0);
        }
    }
}
