use std::time::Duration;

use rand::Rng;
use tracy_math::{
    ColorRGB, Point2D, Point3D, Ray, Vec2D, Vec3D, unit::Degrees,
};
use tracy_traits::Random;

use crate::Geo;

pub struct Cam {
    pub orig: Point3D<f64>,
    pub img_w: usize,
    pub img_h: usize,
    pub render_time: Duration,
    pub frames: usize,
    vw_du: Vec3D<f64>,
    vw_dv: Vec3D<f64>,
    px_00: Point3D<f64>,
    defocus_angle: Degrees<f64>,
    defocus_disk_u: Vec3D<f64>,
    defocus_disk_v: Vec3D<f64>,
    shutter_speed: Duration,
}

impl Cam {
    pub fn trace<T: Geo + Sync>(&self, ray: Ray, geo: &T) -> ColorRGB<f64> {
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
        let a = 0.5 * (dir_u.y() + 1.0);
        let start_color = ColorRGB::new(1.0, 1.0, 1.0);
        let end_color = ColorRGB::new(0.5, 0.7, 1.0);
        start_color * (1.0 - a) + end_color * a
    }

    pub fn sample_px(
        &self,
        px_idx: Point2D<f64>,
        offset: Point2D<f64>,
    ) -> Point3D<f64> {
        self.px_00
            + ((px_idx.x() as f64 + offset.x() as f64) * self.vw_du)
            + ((px_idx.y() as f64 + offset.y() as f64) * self.vw_dv)
    }

    pub fn sample_lens(&self) -> Point3D<f64> {
        if self.defocus_angle.raw() <= 0.0 {
            return self.orig;
        }

        let random_vec = || loop {
            let v = Vec2D::random_range(-1.0..1.0);

            if v.len_2() < 1.0 {
                return v;
            }
        };

        let p = random_vec();
        self.orig
            + (p.x() * self.defocus_disk_u)
            + (p.y() * self.defocus_disk_v)
    }

    pub fn sample_time(&self) -> Duration {
        let mut rng = rand::rng();
        Duration::from_secs_f64(
            rng.random_range(0.0..=self.shutter_speed.as_secs_f64()),
        )
    }
}

/// # Camera Builder.
///
/// ## Viewport diagram.
///
/// ```text
/// ┌───► vw_orig                        
/// │ ┌──► px_00                         
/// ├─┼────────────────────────────┐ vw_v
/// │ x  x  x  x  x                │ ▲   
/// │                              │ │   
/// │ x  x  x  x                   │ │   
/// │       ───► vw_du             │ │   
/// │ x  x │                       │ │   
/// │      │                       │ │   
/// │ x  x ▼ vw_dv                 │ │   
/// │                              │ │   
/// │ x                            │ │   
/// └──────────────────────────────┘     
///  ─────────────────────────► vw_u      
/// ```   
pub struct CamBuilder {
    pub orig: Point3D<f64>,
    pub at: Point3D<f64>,
    pub up: Vec3D<f64>,
    pub img_w: usize,
    pub fov: Degrees<f64>,
    pub defocus_angle: Degrees<f64>,
    pub focus_dist: f64,
    pub fps: usize,
    pub shutter_speed: Duration,
    pub frames: usize,
}

impl CamBuilder {
    pub fn new(orig: Point3D<f64>, at: Point3D<f64>) -> Self {
        Self {
            orig,
            at,
            up: Vec3D::new(0.0, 1.0, 0.0),
            img_w: 400,
            fov: Degrees::new(90.0),
            defocus_angle: Degrees::new(0.0),
            focus_dist: 10.0,
            fps: 24,
            shutter_speed: Duration::from_secs_f64(1.0 / 48.0),
            frames: 1,
        }
    }

    pub fn build(self) -> Cam {
        let img_h = (self.img_w as f64 / (16.0 / 9.0)) as usize;
        let aspect_ratio = self.img_w as f64 / img_h as f64;

        let theta = self.fov.to_radians();
        let h = (theta / 2.0).tan();
        let vw_h = 2.0 * h * self.focus_dist;
        let vw_w = vw_h * aspect_ratio;

        let w = (self.orig - self.at).norm();
        let u = self.up.cross(&w).norm();
        let v = w.cross(&u);

        let vw_u = vw_w * u;
        let vw_v = -vw_h * v;

        let vw_du = vw_u / self.img_w as f64;
        let vw_dv = vw_v / img_h as f64;

        let vw_orig =
            self.orig - (self.focus_dist * w) - (vw_u / 2.0) - (vw_v / 2.0);

        let px_00 = vw_orig + (0.5 * (vw_du + vw_dv));

        let defocus_r =
            self.focus_dist * (self.defocus_angle / 2.0).to_radians().tan();

        Cam {
            orig: self.orig,
            img_w: self.img_w,
            img_h,
            render_time: Duration::from_secs_f64(
                self.frames as f64 / self.fps as f64,
            ),
            frames: self.frames,
            vw_du,
            vw_dv,
            px_00,
            defocus_angle: self.defocus_angle,
            defocus_disk_u: defocus_r * u,
            defocus_disk_v: defocus_r * v,
            shutter_speed: self.shutter_speed,
        }
    }
}
