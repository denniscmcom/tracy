pub mod sphere;

pub use sphere::Sphere;

use std::{ops::RangeInclusive, sync::Arc};

use tracy_math::{Point3D, Ray, Vec3D};

use crate::Mat;

type HitData = Option<(Hit, Arc<dyn Mat + Sync + Send>)>;

pub trait Geo {
    // TODO: Make Range generic to accept multiple range types.
    fn hit(&self, ray: &Ray, range: RangeInclusive<f64>) -> HitData;
}

pub enum Face {
    Front,
    Back,
}

pub struct Hit {
    pub norm: Vec3D<f64>, // It points out and assumed unit length.
    pub orig: Point3D<f64>,
    pub ray_t: f64,
    pub face: Face,
}
