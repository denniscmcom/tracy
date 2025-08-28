use crate::graphics::Frame;
use tracy_math::ColorRGBA;
use tracy_scene::{Geo, Scene};

pub struct State<T>
where
    T: Geo,
{
    pub scene: Scene<T>,
    pub spp: usize,
    pub current_spp: usize,
    pub frame: Frame,
}

impl<T> State<T>
where
    T: Geo,
{
    pub fn new(scene: Scene<T>, spp: usize) -> Self {
        let w = scene.cam.img_w;
        let h = scene.cam.img_h;
        Self {
            scene,
            spp,
            current_spp: 0,
            frame: vec![ColorRGBA::new(0.0, 0.0, 0.0, 1.0); w * h],
        }
    }
}
