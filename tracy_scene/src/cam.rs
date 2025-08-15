use tracy_math::{Point2D, Point3D, Vec3D};

/// # Camera.
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
pub struct Cam {
    pub orig: Point3D,
    pub img_w: usize,
    pub img_h: usize,
    vw_du: Vec3D,
    vw_dv: Vec3D,
    px_00: Point3D,
}

impl Cam {
    pub fn new(img_w: usize, fov: Degrees) -> Self {
        let ideal_aspect_ratio = 16.0 / 9.0;
        let img_h = (img_w as f64 / ideal_aspect_ratio) as usize;
        let actual_aspect_ratio = img_w as f64 / img_h as f64;

        let orig = Point3D::new(-2.0, 2.0, 1.0);
        let at = Point3D::new(0.0, 0.0, -1.0);
        let up = Vec3D::new(0.0, 1.0, 0.0);
        let view = orig - at;

        let focal_len = (orig - at).len_2().sqrt();
        let theta = f64::to_radians(fov.0);
        let h = f64::tan(theta / 2.0);
        let vw_h = 2.0 * h * focal_len;
        let vw_w = vw_h * actual_aspect_ratio;

        let w = view.normalize();
        let u = up.cross(&w).normalize();
        let v = w.cross(&u);

        let vw_u = u * vw_w;
        let vw_v = -v * vw_h;

        let vw_du = vw_u / img_w as f64;
        let vw_dv = vw_v / img_h as f64;

        let vw_orig = orig - (w * focal_len) - vw_u / 2.0 - vw_v / 2.0;
        let px_00 = vw_orig + (vw_du + vw_dv) * 0.5;

        Self {
            orig,
            img_w,
            img_h,
            vw_du,
            vw_dv,
            px_00,
        }
    }

    pub fn sample_px(&self, px_idx: Point2D, offset: Point2D) -> Point3D {
        self.px_00
            + (self.vw_du * (px_idx.x as f64 + offset.x as f64))
            + (self.vw_dv * (px_idx.y as f64 + offset.y as f64))
    }
}

pub struct Degrees(pub f64);
