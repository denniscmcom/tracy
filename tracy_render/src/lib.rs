use rayon::prelude::*;
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
                    let ray_dir = px_sample - ray_orig;
                    let ray = Ray::new(ray_orig, ray_dir, self.depth);
                    px += self.trace(ray, &scene.geo);
                }

                px *= 1.0 / self.spp as f64;
                px.to_gamma().scale()
            })
            .collect();

        buf
    }

    fn trace<T: Geo>(&self, ray: Ray, geo: &T) -> ColorRGB<f64> {
        if ray.depth == 0 {
            return ColorRGB::new(0.0, 0.0, 0.0);
        }

        if let Some((hit, mat)) = geo.hit(&ray, 0.001..=f64::MAX) {
            loop {
                if let Some(scatter_data) = mat.scatter(ray, hit) {
                    return self.trace(scatter_data.ray, geo)
                        * scatter_data.attenuation;
                }

                return ColorRGB::new(0.0, 0.0, 0.0);
            }
        }

        // Gradient background.
        let dir_u = ray.dir / ray.dir.len_2().sqrt();
        let a = 0.5 * (dir_u.y + 1.0);
        let start_color = ColorRGB::new(1.0, 1.0, 1.0);
        let end_color = ColorRGB::new(0.5, 0.7, 1.0);
        start_color * (1.0 - a) + end_color * a
    }
}

pub mod benchmarks {
    use super::*;
    use std::sync::Arc;
    use tracy_math::{ColorRGB, Point3D, Vec3D, unit::Degrees};
    use tracy_scene::{Scene, cam::CamBuilder, geo::Sphere, mat};

    pub fn renderer_render() -> impl Fn() {
        let cam_builder = CamBuilder {
            orig: Point3D::new(0.0, 0.0, 0.0),
            at: Point3D::new(0.0, 0.0, -10.0),
            up: Vec3D::new(0.0, 1.0, 0.0),
            img_w: 400,
            fov: Degrees::new(90.0),
            defocus_angle: Degrees::new(0.6),
            focus_dist: 10.0,
        };

        let cam = cam_builder.build();
        let sphere = Sphere {
            orig: Point3D::new(0.0, 0.0, -10.0),
            r: 1.0,
            mat: Arc::new(mat::Lambert {
                albedo: ColorRGB::new(1.0, 0.0, 0.0),
            }),
        };

        let scene = Scene::new(cam, sphere);
        let renderer = Renderer { spp: 10, depth: 2 };
        move || {
            let _ = renderer.render(&scene);
        }
    }

    pub fn renderer_trace() -> impl Fn() {
        let sphere = Sphere {
            orig: Point3D::new(0.0, 0.0, -10.0),
            r: 1.0,
            mat: Arc::new(mat::Lambert {
                albedo: ColorRGB::new(1.0, 0.0, 0.0),
            }),
        };

        let ray = Ray {
            orig: Point3D::new(0.0, 0.0, 0.0),
            dir: Vec3D::new(0.0, 0.0, -1.0),
            depth: 1,
        };

        let renderer = Renderer { spp: 10, depth: 2 };
        move || {
            renderer.trace(ray, &sphere);
        }
    }
}
