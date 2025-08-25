use rayon::prelude::*;
use std::time::Duration;
use tracy_macros::Random;
use tracy_math::{ColorRGB, Point2D, Ray, Vec2D};
use tracy_scene::{Geo, Scene};

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
    pub spp: usize,
    pub depth: usize,
}

impl Renderer {
    pub fn render<T>(&self, scene: &Scene<T>) -> Buf
    where
        T: Geo + Sync,
    {
        let cam = &scene.cam;
        let mut buf = Buf::new(cam.img_w, cam.img_h);
        buf.px_data = (0..cam.img_w * cam.img_h)
            .into_par_iter()
            .map(|i| {
                let (x, y) = (i % cam.img_w, i / cam.img_w);
                let mut px = ColorRGB::new(0.0, 0.0, 0.0);

                for _ in 0..self.spp {
                    let px_idx = Point2D::new(x as f64, y as f64);
                    let offset =
                        Point2D::random_range(0.0..1.0) - Vec2D::new(0.5, 0.5);

                    let px_sample = cam.sample_px(px_idx, offset);
                    let ray_orig = cam.sample_lens();
                    let ray = Ray {
                        orig: ray_orig,
                        dir: px_sample - ray_orig,
                        depth: self.depth,
                        norm_ts: Duration::from_secs_f64(
                            cam.sample_time().as_secs_f64()
                                / cam.render_time.as_secs_f64(),
                        ),
                    };

                    px += cam.trace(ray, &scene.geo);
                }

                px *= 1.0 / self.spp as f64;
                px.to_gamma().scale()
            })
            .collect();

        buf
    }
}

pub mod benchmarks {
    use super::*;
    use std::time;
    use tracy_math::{Point3D, Vec3D, unit::Degrees};
    use tracy_scene::{Scene, cam::CamBuilder, geo::Sphere};

    pub fn renderer_render() -> impl Fn() {
        let cam_builder = CamBuilder {
            orig: Point3D::new(0.0, 0.0, 0.0),
            at: Point3D::new(0.0, 0.0, -10.0),
            up: Vec3D::new(0.0, 1.0, 0.0),
            img_w: 400,
            fov: Degrees::new(90.0),
            defocus_angle: Degrees::new(0.6),
            focus_dist: 10.0,
            fps: 24,
            shutter_speed: time::Duration::from_secs_f64(1.0 / 48.0),
            frames: 1,
        };

        let cam = cam_builder.build();
        let sphere = Sphere::new(Point3D::new(0.0, 0.0, -10.0), 1.0);
        let scene = Scene::new(cam, sphere);
        let renderer = Renderer { spp: 10, depth: 2 };
        move || {
            let _ = renderer.render(&scene);
        }
    }
}
