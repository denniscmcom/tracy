use tracy_macros::Random;
use tracy_math::{ColorRGB, Point3D, Ray, Vec3D};

pub trait Mat {
    fn scatter(&self, ray: Ray, norm: Vec3D, orig: Point3D) -> ScatterData;
}

// TODO: I don't like this name.
pub struct ScatterData {
    pub ray: Ray,
    pub attenuation: ColorRGB<f64>,
}

pub struct Lambert {
    pub albedo: ColorRGB<f64>,
}

impl Mat for Lambert {
    fn scatter(&self, ray: Ray, norm: Vec3D, orig: Point3D) -> ScatterData {
        loop {
            let v = Vec3D::random_range(-1.0..1.0);
            let v_len2: f64 = v.len_2();

            if 1e-160 < v_len2 && v_len2 <= 1.0 {
                let v_u = v / v_len2.sqrt();
                let mut dir = norm + v_u;
                let thr = 1e-8;

                if dir.x.abs() < thr && dir.y.abs() < thr && dir.z.abs() < thr {
                    dir = norm;
                }

                return ScatterData {
                    ray: Ray::new(orig, dir, ray.depth - 1),
                    attenuation: self.albedo,
                };
            }
        }
    }
}

pub struct Metal {
    pub albedo: ColorRGB<f64>,
}

impl Mat for Metal {
    fn scatter(&self, ray: Ray, norm: Vec3D, orig: Point3D) -> ScatterData {
        let dir = ray.dir - norm * ray.dir.dot(&norm) * 2.0;
        ScatterData {
            ray: Ray::new(orig, dir, ray.depth - 1),
            attenuation: self.albedo,
        }
    }
}
