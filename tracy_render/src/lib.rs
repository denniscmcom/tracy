mod ray;

use crate::ray::Ray;
use rand::prelude::*;
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

pub struct Renderer {
    pub samples_per_px: usize,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            samples_per_px: 100,
        }
    }
    pub fn render(&self, scene: Scene) -> Buf {
        let cam = &scene.cam;
        let mut buf = Buf::new(cam.img_w, cam.img_h);
        let mut rng = rand::rng();

        for y in 0..cam.img_h {
            for x in 0..cam.img_w {
                let mut px = ColorRGB::new(0.0, 0.0, 0.0);

                for _ in 0..self.samples_per_px {
                    let px_idx = Point2D::new(x, y);
                    let offset = Point2D {
                        x: rng.random_range(0.0..1.0) - 0.5,
                        y: rng.random_range(0.0..1.0) - 0.5,
                    };

                    let px_sample = cam.sample_px(px_idx, offset);
                    let ray = Ray::new(cam.orig, px_sample - cam.orig);
                    px += ray.trace(&scene.spheres);
                }

                px *= 1.0 / self.samples_per_px as f64;
                buf.px_data.push(px.scale());
            }
        }

        buf
    }
}
