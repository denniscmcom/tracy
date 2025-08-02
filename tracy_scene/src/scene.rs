use crate::{Cam, Sphere};

pub struct Scene {
    pub cam: Cam,
    pub sphere: Sphere<f64>,
}

impl Scene {
    pub fn new(cam: Cam, sphere: Sphere<f64>) -> Self {
        Self { cam, sphere }
    }
}
