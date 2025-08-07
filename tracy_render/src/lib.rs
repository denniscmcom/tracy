mod ray;

use crate::ray::Ray;
use tracy_math::{ColorRGB, Point2D};
use tracy_scene::Scene;

pub struct Buf {
    pub px_data: Vec<ColorRGB<u8>>,
    pub rows: usize,
}

impl Buf {
    fn new(cols: usize, rows: usize) -> Self {
        Self {
            px_data: Vec::with_capacity(cols * rows),
            rows,
        }
    }
}

pub struct Renderer {}

impl Renderer {
    pub fn new() -> Self {
        Self {}
    }
    pub fn render(&self, scene: Scene) -> Buf {
        let cam = &scene.cam;
        let mut buf = Buf::new(cam.img_w, cam.img_h);

        for y in 0..cam.img_h {
            for x in 0..cam.img_w {
                let px_center = cam.calc_px_center(Point2D { x, y });
                let ray = Ray::new(cam.orig, px_center - cam.orig);
                let px = ray.trace(&scene.spheres);
                buf.px_data.push(px.scale());
            }
        }

        buf
    }
}
