use std::ops::Range;
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

    pub fn trace(&self, spheres: &Vec<Sphere<f64>>) -> ColorRGB<f64> {
        let hit_data = spheres.hit(&self, &(0.0..f64::MAX));

        if let Some(hit_data) = hit_data {
            let norm = hit_data.out_norm;
            let (r, g, b) = (norm.x + 1.0, norm.y + 1.0, norm.z + 1.0);
            let color = ColorRGB::new(r, g, b);
            return color * 0.5;
        }

        let dir_u = self.dir / self.dir.len_2().sqrt();
        let a = 0.5 * (dir_u.y + 0.5);
        let start_color = ColorRGB::new(1.0, 1.0, 1.0);
        let end_color = ColorRGB::new(0.5, 0.7, 1.0);
        start_color * (1.0 - a) + end_color * a
    }
}

#[derive(Clone)]
enum Face {
    Front,
    Back,
}

#[derive(Clone)]
struct HitData {
    // out_norm always points out and assumed unit lenght.
    pub out_norm: Vec3D<f64>,
    pub p: Point3D<f64>,
    pub t: f64,
    pub face: Face,
}

trait Hit {
    fn hit(&self, ray: &Ray, range: &Range<f64>) -> Option<HitData>;
}

impl Hit for Sphere<f64> {
    fn hit(&self, ray: &Ray, range: &Range<f64>) -> Option<HitData> {
        let oc = self.orig - ray.orig;
        let a = ray.dir.len_2();
        let h = ray.dir.dot(&oc);
        let c = oc.len_2() - self.r * self.r;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        };

        let sqrtd = discriminant.sqrt();
        let root = (h - sqrtd) / a;

        if !range.contains(&root) {
            return None;
        };

        let p = ray.at(root);
        let norm = (p - self.orig) / self.r;
        let (out_norm, face) = if ray.dir.dot(&norm) < 0.0 {
            (norm, Face::Front)
        } else {
            (norm * -1.0, Face::Back)
        };

        Some(HitData {
            out_norm,
            p,
            t: root,
            face,
        })
    }
}

impl Hit for Vec<Sphere<f64>> {
    fn hit(&self, ray: &Ray, range: &Range<f64>) -> Option<HitData> {
        let mut hits = Vec::new();
        let mut closest = range.end;

        for sphere in self {
            if let Some(hit_data) = sphere.hit(ray, &(range.start..closest)) {
                closest = hit_data.t;
                hits.push(hit_data);
            }
        }

        hits.iter()
            .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap())
            .cloned()
    }
}
