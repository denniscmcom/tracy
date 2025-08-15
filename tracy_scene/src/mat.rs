use crate::geo::{Face, Hit};
use rand::Rng;
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
        let mut dir = reflect(&ray.dir, &hit.norm);
        let dir_u = dir / dir.len_2().sqrt();
        dir = dir_u + (random_vec_u() * self.fuzz);

        if dir.dot(&hit.norm) <= 0.0 {
            return None;
        }

        Some(ScatterData {
            ray: Ray::new(hit.orig, dir, ray.depth - 1),
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

        let ray_dir_u = ray.dir / ray.dir.len_2().sqrt();
        let cos_theta = f64::min(-ray_dir_u.dot(&hit.norm), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);
        let is_reflected = ri * sin_theta > 1.0;
        let reflectance = self.comp_reflectance(cos_theta);
        let mut rng = rand::rng();
        let rand_f64 = rng.random_range(0.0..1.0);

        let dir = if is_reflected || reflectance > rand_f64 {
            reflect(&ray_dir_u, &hit.norm)
        } else {
            refract(&ray_dir_u, &hit.norm, ri)
        };

        Some(ScatterData {
            ray: Ray::new(hit.orig, dir, ray.depth - 1),
            attenuation: ColorRGB::new(1.0, 1.0, 1.0),
        })
    }
}

impl Dielectric {
    fn comp_reflectance(&self, cos: f64) -> f64 {
        let mut r0 = (1.0 - self.refract_idx) / (1.0 + self.refract_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cos).powf(5.0)
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

fn reflect(v: &Vec3D, norm: &Vec3D) -> Vec3D {
    *v - *norm * v.dot(&norm) * 2.0
}

fn refract(v_u: &Vec3D, norm: &Vec3D, etai_over_etat: f64) -> Vec3D {
    let cos_theta = f64::min(-v_u.dot(&norm), 1.0);
    let r_out_perp = (*v_u + *norm * cos_theta) * etai_over_etat;
    let r_out_parallel = *norm * -f64::sqrt(f64::abs(1.0 - r_out_perp.len_2()));
    r_out_perp + r_out_parallel
}
