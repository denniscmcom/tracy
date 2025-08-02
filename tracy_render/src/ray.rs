use tracy_math::{ColorRGB, Point3D, Vec3D};
use tracy_scene::Sphere;

pub struct Ray {
    pub orig: Point3D<f64>,
    pub dir: Vec3D<f64>,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point3D<f64> {
        self.orig + self.dir * t
    }

    pub fn trace(&self, sphere: &Sphere<f64>) -> ColorRGB<f64> {
        if self.hit_sphere(sphere) {
            return ColorRGB {
                r: 1.0,
                g: 0.0,
                b: 0.0,
            };
        }

        let len = self.dir.len_2().sqrt();
        let unit_dir = self.dir / len;
        let a = 0.5 * (unit_dir.y + 1.0);

        let start_color = ColorRGB {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        };

        let end_color = ColorRGB {
            r: 0.5,
            g: 0.7,
            b: 1.0,
        };

        start_color * (1.0 - a) + end_color * a
    }

    pub fn hit_sphere(&self, sphere: &Sphere<f64>) -> bool {
        let oc = sphere.orig - self.orig;
        let a = self.dir.dot(&self.dir);
        let b = -2.0 * self.dir.dot(&oc);
        let c = oc.dot(&oc) - sphere.radius * sphere.radius;
        let discriminant = b * b - 4.0 * a * c;
        discriminant >= 0.0
    }
}
