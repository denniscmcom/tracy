use tracy_math::{
    Point2D, Point3D, Vec3D,
    unit::{Degrees, Radians},
};

pub struct Cam {
    pub orig: Point3D,
    pub img_w: usize,
    pub img_h: usize,
    vw_du: Vec3D,
    vw_dv: Vec3D,
    px_00: Point3D,
}

impl Cam {
    pub fn sample_px(&self, px_idx: Point2D, offset: Point2D) -> Point3D {
        self.px_00
            + (self.vw_du * (px_idx.x as f64 + offset.x as f64))
            + (self.vw_dv * (px_idx.y as f64 + offset.y as f64))
    }
}

/// # CamBuilder.
///
/// ## Viewport diagram.
///
/// ```
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
    pub orig: Point3D,
    pub at: Point3D,
    pub up: Vec3D,
    pub img_w: usize,
    pub fov: Degrees,
}

impl CamBuilder {
    pub fn new(orig: Point3D, at: Point3D) -> Self {
        Self {
            orig,
            at,
            up: Vec3D::new(0.0, 1.0, 0.0),
            img_w: 400,
            fov: Degrees::new(90.0),
        }
    }

    pub fn build(self) -> Cam {
        let img_h = (self.img_w as f64 / (16.0 / 9.0)) as usize;
        let aspect_ratio = self.img_w as f64 / img_h as f64;

        let focal_len = (self.orig - self.at).len_2().sqrt();
        let theta = self.fov.to_radians();
        let h = (theta / Radians::new(2.0)).tan();
        let vw_h = focal_len * h.to_f64() * 2.0;
        let vw_w = vw_h * aspect_ratio;

        let w = (self.orig - self.at).normalize();
        let u = self.up.cross(&w).normalize();
        let v = w.cross(&u);

        let vw_u = u * vw_w;
        let vw_v = -v * vw_h;

        let vw_du = vw_u / self.img_w as f64;
        let vw_dv = vw_v / img_h as f64;

        let vw_orig = self.orig - (w * focal_len) - vw_u / 2.0 - vw_v / 2.0;
        let px_00 = vw_orig + (vw_du + vw_dv) * 0.5;

        Cam {
            orig: self.orig,
            img_w: self.img_w,
            img_h,
            vw_du,
            vw_dv,
            px_00,
        }
    }
}
