use crate::{Point3D, Vec3D};

/// # Ray.
///
/// ## Mathematical function.
///
/// `P(t) = A + t * b`, where:
/// - `P(t)`: 3D position along a line.
/// - `A`: Ray origin.
/// - `b`: Ray direction.
///
/// ## Diagram.
///
/// ```text
/// t=-1  t=0   t=1  t=2
/// ◄│─────│─────│────│►
///        ──────►      
///        A     b            
/// ```
#[derive(Clone, Copy)]
pub struct Ray {
    pub orig: Point3D,
    pub dir: Vec3D,
    pub depth: usize,
}

impl Ray {
    pub fn new(orig: Point3D, dir: Vec3D, depth: usize) -> Self {
        Self { orig, dir, depth }
    }

    pub fn at(&self, t: f64) -> Point3D {
        self.orig + self.dir * t
    }
}

pub mod bechmarks {
    use super::*;

    pub fn ray_at() -> impl Fn() {
        let ray = Ray {
            orig: Point3D::new(1.0, 2.0, 3.0),
            dir: Vec3D::new(3.0, 2.0, 1.0),
            depth: 1,
        };

        move || {
            ray.at(10.0);
        }
    }
}
