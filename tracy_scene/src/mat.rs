use crate::geo::{Face, Hit};
use tracy_macros::Random;
use tracy_math::{ColorRGB, Ray, Vec3D};

pub trait Mat {
    fn scatter(&self, ray: Ray, hit: Hit) -> Option<ScatterData>;
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
    fn scatter(&self, ray: Ray, hit: Hit) -> Option<ScatterData> {
        let mut dir = hit.norm + random_vec_u();
        let thr = 1e-8;

        if dir.x.abs() < thr && dir.y.abs() < thr && dir.z.abs() < thr {
            dir = hit.norm;
        }

        return Some(ScatterData {
            ray: Ray::new(hit.orig, dir, ray.depth - 1),
            attenuation: self.albedo,
        });
    }
}

pub struct Metal {
    pub albedo: ColorRGB<f64>,
    pub fuzz: f64,
}

impl Mat for Metal {
    fn scatter(&self, ray: Ray, hit: Hit) -> Option<ScatterData> {
        let dir = ray.dir - hit.norm * ray.dir.dot(&hit.norm) * 2.0;
        let dir_u = dir / dir.len_2().sqrt();
        let reflected_dir = dir_u + (random_vec_u() * self.fuzz);

        if reflected_dir.dot(&hit.norm) <= 0.0 {
            return None;
        }

        Some(ScatterData {
            ray: Ray::new(hit.orig, reflected_dir, ray.depth - 1),
            attenuation: self.albedo,
        })
    }
}

pub struct Dielectric {
    pub refract_idx: f64,
}

impl Mat for Dielectric {
    fn scatter(&self, ray: Ray, hit: Hit) -> Option<ScatterData> {
        let ri = match hit.face {
            Face::Front => 1.0 / self.refract_idx,
            Face::Back => self.refract_idx,
        };

        let dir_u = ray.dir / ray.dir.len_2().sqrt();
        let n_dir_u = dir_u * -1.0;
        let cos_theta = f64::min(n_dir_u.dot(&hit.norm), 1.0);
        let r_out_perp = (dir_u + hit.norm * cos_theta) * ri;
        let r_out_parallel =
            hit.norm * -f64::sqrt(f64::abs(1.0 - r_out_perp.len_2()));

        let refracted_dir = r_out_perp + r_out_parallel;
        Some(ScatterData {
            ray: Ray::new(hit.orig, refracted_dir, ray.depth - 1),
            attenuation: ColorRGB::new(1.0, 1.0, 1.0),
        })
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
