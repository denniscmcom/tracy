pub mod sphere;

pub use sphere::Sphere;

use crate::Mat;
use std::{ops::RangeInclusive, rc::Rc};
use tracy_math::{Point3D, Ray, Vec3D};

pub trait Geo {
    fn hit(
        &self,
        ray: &Ray,
        range: &RangeInclusive<f64>,
    ) -> Option<(Hit, Rc<dyn Mat>)>;
}

pub enum Face {
    Front,
    Back,
}

pub struct Hit {
    pub norm: Vec3D, // It points out and assumed unit length.
    pub orig: Point3D,
    pub ray_t: f64,
    pub face: Face,
}
