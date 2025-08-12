pub mod sphere;

pub use sphere::Sphere;

use crate::Mat;
use std::{ops::Range, rc::Rc};
use tracy_math::{Point3D, Ray, Vec3D};

pub trait Geo {
    type MatTy: Mat;
    fn hit(&self, ray: &Ray, range: &Range<f64>) -> Option<Hit<Self::MatTy>>;
}

pub enum Face {
    Front,
    Back,
}

pub struct Hit<T>
where
    T: Mat,
{
    pub norm: Vec3D, // It points out and assumed unit length.
    pub orig: Point3D,
    pub ray_t: f64,
    pub face: Face,
    pub mat: Rc<T>,
}
