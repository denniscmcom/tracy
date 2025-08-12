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
/// ```
/// t=-1  t=0   t=1  t=2
/// ◄│─────│─────│────│►
///        ──────►      
///        A     b            
/// ```
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
