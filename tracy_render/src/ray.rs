use tracy_math::{ColorRGB, Point3D, Vec3D};
use tracy_scene::Sphere;

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
    pub orig: Point3D<f64>,
    pub dir: Vec3D<f64>,
}

impl Ray {
    pub fn new(orig: Point3D<f64>, dir: Vec3D<f64>) -> Self {
        Self { orig, dir }
    }

    pub fn at(&self, t: f64) -> Point3D<f64> {
        self.orig + self.dir * t
    }

    pub fn trace(&self, sphere: &Sphere<f64>) -> ColorRGB<f64> {
        let t = self.hit_sphere(sphere);

        if t > 0.0 {
            let norm = self.at(t) - sphere.orig;
            let norm_u = norm / norm.len_2().sqrt();
            let (r, g, b) = (norm_u.x + 1.0, norm_u.y + 1.0, norm_u.z + 1.0);
            let color = ColorRGB::new(r, g, b);
            return color * 0.5;
        }

        let dir_u = self.dir / self.dir.len_2().sqrt();
        let a = 0.5 * (dir_u.y + 1.0);

        let start_color = ColorRGB::new(1.0, 1.0, 1.0);
        let end_color = ColorRGB::new(0.5, 0.7, 1.0);
        start_color * (1.0 - a) + end_color * a
    }

    pub fn hit_sphere(&self, sphere: &Sphere<f64>) -> f64 {
        let oc = sphere.orig - self.orig;
        let a = self.dir.dot(&self.dir);
        let b = -2.0 * self.dir.dot(&oc);
        let c = oc.dot(&oc) - sphere.r * sphere.r;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return -1.0;
        }

        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}
