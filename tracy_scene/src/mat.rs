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
        let mut dir = norm + random_vec_u();
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

pub struct Metal {
    pub albedo: ColorRGB<f64>,
    pub fuzz: f64,
}

impl Mat for Metal {
    fn scatter(&self, ray: Ray, norm: Vec3D, orig: Point3D) -> ScatterData {
        let dir = ray.dir - norm * ray.dir.dot(&norm) * 2.0;
        let dir_u = dir / dir.len_2().sqrt();
        let fuzz_dir = dir_u + (random_vec_u() * self.fuzz);
        ScatterData {
            ray: Ray::new(orig, fuzz_dir, ray.depth - 1),
            attenuation: self.albedo,
        }
    }
}

fn random_vec_u() -> Vec3D {
    loop {
        let v = Vec3D::random_range(-1.0..1.0);
        let v_len2: f64 = v.len_2();

        // Handle floating-point precision overflow.
        if 1e-160 < v_len2 && v_len2 <= 1.0 {
            return v / v_len2.sqrt();
        }
    }
}
