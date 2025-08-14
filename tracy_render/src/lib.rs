use tracy_macros::Random;
use tracy_math::{ColorRGB, Point2D, Ray};
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
    pub samples_per_px: usize,
    pub depth: usize,
}

impl Renderer {
    pub fn render<T: Geo>(&self, scene: Scene<T>) -> Buf {
        let cam = &scene.cam;
        let mut buf = Buf::new(cam.img_w, cam.img_h);

        for y in 0..cam.img_h {
            for x in 0..cam.img_w {
                let mut px = ColorRGB::new(0.0, 0.0, 0.0);

                for _ in 0..self.samples_per_px {
                    let px_idx = Point2D::new(x as f64, y as f64);
                    let offset = Point2D::random_range(0.0..1.0) - 0.5;
                    let px_sample = cam.sample_px(px_idx, offset);
                    let ray_dir = px_sample - cam.orig;
                    let ray = Ray::new(cam.orig, ray_dir, self.depth);
                    px += self.trace(ray, &scene.geo);
                }

                px *= 1.0 / self.samples_per_px as f64;
                buf.px_data.push(px.to_gamma().scale());
            }
        }

        buf
    }

    fn trace<T: Geo>(&self, ray: Ray, geo: &T) -> ColorRGB<f64> {
        if ray.depth == 0 {
            return ColorRGB::new(0.0, 0.0, 0.0);
        }

        if let Some((hit, mat)) = geo.hit(&ray, &(0.001..f64::MAX)) {
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
