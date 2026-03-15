use std::time::Duration;

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
#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub orig: Point3D<f64>,
    pub dir: Vec3D<f64>,
    pub depth: usize,
    pub norm_ts: Duration,
}

impl Ray {
    pub fn new(orig: Point3D<f64>, dir: Vec3D<f64>) -> Self {
        Self {
            orig,
            dir,
            depth: 1,
            norm_ts: Duration::from_secs(0),
        }
    }

    pub fn at(&self, t: f64) -> Point3D<f64> {
        self.orig + (t * self.dir)
    }
}
