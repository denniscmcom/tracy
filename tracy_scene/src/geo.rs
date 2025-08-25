pub mod sphere;

pub use sphere::Sphere;

use crate::Mat;
use std::{ops::RangeInclusive, sync::Arc, time::Duration};
use tracy_math::{Point3D, Ray, Vec3D};

type HitData = Option<(Hit, Arc<dyn Mat + Sync + Send>)>;

pub trait Geo {
    // TODO: Make Range generic to accept multiple range types.
    fn hit(&self, ray: &Ray, range: RangeInclusive<f64>) -> HitData;
    fn at(&self, ts: Duration, total: Duration) -> Self;
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
