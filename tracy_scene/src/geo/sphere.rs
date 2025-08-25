use crate::{
    geo::{Face, Geo, Hit, HitData},
    mat::{self, Mat},
};
use std::{ops::RangeInclusive, sync::Arc, time::Duration};
use tracy_math::{ColorRGB, Point3D, Ray};

pub struct Sphere {
    pub orig: Point3D,
    pub r: f64,
    pub mat: Arc<dyn Mat + Sync + Send>,
    pub orig_2: Option<Point3D>,
}

impl Sphere {
    pub fn new(orig: Point3D, r: f64) -> Self {
        Self {
            orig,
            r,
            mat: Arc::new(mat::Lambert {
                albedo: ColorRGB::new(1.0, 0.0, 0.0),
            }),
            orig_2: None,
        }
    }

    pub fn orig_at(&self, norm_ts: Duration) -> Point3D {
        if let Some(orig_2) = self.orig_2 {
            return self.orig.lerp(&orig_2, norm_ts.as_secs_f64());
        }

        self.orig
    }
}

impl Geo for Sphere {
    fn hit(&self, ray: &Ray, range: RangeInclusive<f64>) -> HitData {
        let lerp_orig = self.orig_at(ray.norm_ts);
        let oc = lerp_orig - ray.orig;
        let a = ray.dir.len_2();
        let h = ray.dir.dot(&oc);
        let c = oc.len_2() - self.r * self.r;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        };

        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;

        if !range.contains(&root) {
            root = (h + sqrtd) / a;

            if !range.contains(&root) {
                return None;
            }
        }

        let p = ray.at(root);
        let norm = (p - lerp_orig) / self.r;
        let (out_norm, face) = if ray.dir.dot(&norm) <= 0.0 {
            (norm, Face::Front)
        } else {
            (norm * -1.0, Face::Back)
        };

        let hit = Hit {
            norm: out_norm,
            orig: p,
            ray_t: root,
            face,
        };

        Some((hit, Arc::clone(&self.mat)))
    }
}

impl Geo for Vec<Sphere> {
    fn hit(&self, ray: &Ray, range: RangeInclusive<f64>) -> HitData {
        let mut hits = Vec::new();
        let mut closest = *range.end();

        for sphere in self {
            let ray_range = *range.start()..=closest;
            if let Some((hit, mat)) = sphere.hit(ray, ray_range) {
                closest = hit.ray_t;
                hits.push((hit, mat));
            }
        }

        hits.into_iter()
            .min_by(|a, b| a.0.ray_t.partial_cmp(&b.0.ray_t).unwrap())
    }
}

pub mod benchmarks {
    use super::*;
    use tracy_math::Vec3D;

    pub fn sphere_hit() -> impl Fn() {
        let sphere = Sphere::new(Point3D::new(0.0, 0.0, -10.0), 1.0);
        let ray =
            Ray::new(Point3D::new(0.0, 0.0, 0.0), Vec3D::new(0.0, 0.0, -1.0));

        move || {
            sphere.hit(&ray, 0.0..=f64::MAX);
        }
    }
}
