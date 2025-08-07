use crate::{Cam, Sphere};

pub struct Scene {
    pub cam: Cam,
    pub spheres: Vec<Sphere<f64>>,
}

impl Scene {
    pub fn new(cam: Cam, spheres: Vec<Sphere<f64>>) -> Self {
        Self { cam, spheres }
    }
}
