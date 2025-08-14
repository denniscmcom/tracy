use crate::{
    geo::{Face, Geo, Hit},
    mat::Mat,
};
use std::{ops::RangeInclusive, rc::Rc};
use tracy_math::{Point3D, Ray};

pub struct Sphere {
    pub orig: Point3D,
    pub r: f64,
    pub mat: Rc<dyn Mat>,
}

impl Geo for Sphere {
    fn hit(
        &self,
        ray: &Ray,
        range: &RangeInclusive<f64>,
    ) -> Option<(Hit, Rc<dyn Mat>)> {
        let oc = self.orig - ray.orig;
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

        let orig = ray.at(root);
        let norm = (orig - self.orig) / self.r;
        let (out_norm, face) = if ray.dir.dot(&norm) <= 0.0 {
            (norm, Face::Front)
        } else {
            (norm * -1.0, Face::Back)
        };

        let hit = Hit {
            norm: out_norm,
            orig,
            ray_t: root,
            face,
        };

        Some((hit, Rc::clone(&self.mat)))
    }
}

impl Geo for Vec<Sphere> {
    fn hit(
        &self,
        ray: &Ray,
        range: &RangeInclusive<f64>,
    ) -> Option<(Hit, Rc<dyn Mat>)> {
        let mut hits = Vec::new();
        let mut closest = *range.end();

        for sphere in self {
            let ray_range = *range.start()..=closest;
            if let Some((hit, mat)) = sphere.hit(ray, &ray_range) {
                closest = hit.ray_t;
                hits.push((hit, mat));
            }
        }

        hits.into_iter()
            .min_by(|a, b| a.0.ray_t.partial_cmp(&b.0.ray_t).unwrap())
    }
}
