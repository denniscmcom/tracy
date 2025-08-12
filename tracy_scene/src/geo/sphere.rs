use crate::{geo, geo::Geo, mat::Mat};
use std::{ops::Range, rc::Rc};
use tracy_math::{Point3D, Ray};

pub struct Sphere<T>
where
    T: Mat,
{
    pub orig: Point3D,
    pub r: f64,
    pub mat: Rc<T>,
}

impl<T> Sphere<T>
where
    T: Mat,
{
    pub fn new(orig: Point3D, r: f64, mat: T) -> Self {
        Self {
            orig,
            r,
            mat: Rc::new(mat),
        }
    }
}

impl<T> Geo for Sphere<T>
where
    T: Mat,
{
    type MatTy = T;

    fn hit(&self, ray: &Ray, range: &Range<f64>) -> Option<geo::Hit<T>> {
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

        let orig = ray.at(root);
        let norm = (orig - self.orig) / self.r;
        let (out_norm, face) = if ray.dir.dot(&norm) < 0.0 {
            (norm, geo::Face::Front)
        } else {
            (norm * -1.0, geo::Face::Back)
        };

        Some(geo::Hit {
            norm: out_norm,
            orig,
            ray_t: root,
            face,
            mat: Rc::clone(&self.mat),
        })
    }
}

impl<T> Geo for Vec<Sphere<T>>
where
    T: Mat,
{
    type MatTy = T;

    fn hit(&self, ray: &Ray, range: &Range<f64>) -> Option<geo::Hit<T>> {
        let mut hits = Vec::new();
        let mut closest = range.end;

        for sphere in self {
            if let Some(hit_data) = sphere.hit(ray, &(range.start..closest)) {
                closest = hit_data.ray_t;
                hits.push(hit_data);
            }
        }

        hits.into_iter()
            .min_by(|a, b| a.ray_t.partial_cmp(&b.ray_t).unwrap())
    }
}
