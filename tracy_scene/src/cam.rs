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
    pub orig: Point3D<f64>,
    pub img_w: usize,
    pub img_h: usize,
    vw_du: Vec3D<f64>,
    vw_dv: Vec3D<f64>,
    px_00: Point3D<f64>,
}

impl Cam {
    pub fn new(img_w: usize) -> Self {
        let ideal_aspect_ratio = 16.0 / 9.0;
        let img_h = (img_w as f64 / ideal_aspect_ratio) as usize;
        let actual_aspect_ratio = img_w as f64 / img_h as f64;

        let vw_h = 2.0;
        let vw_w = vw_h * actual_aspect_ratio;

        let vw_u = Vec3D::new(vw_w, 0.0, 0.0);
        let vw_v = Vec3D::new(0.0, -vw_h, 0.0);

        let vw_du = vw_u / img_w as f64;
        let vw_dv = vw_v / img_h as f64;

        let orig = Point3D::new(0.0, 0.0, 0.0);
        let focal_len = 1.0;
        let focal_len_v = Vec3D::new(0.0, 0.0, focal_len);
        let vw_orig = orig - focal_len_v - vw_u / 2.0 - vw_v / 2.0;
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

    pub fn sample_px(
        &self,
        px_idx: Point2D<usize>,
        offset: Point2D<f64>,
    ) -> Point3D<f64> {
        self.px_00
            + (self.vw_du * (px_idx.x as f64 + offset.x as f64))
            + (self.vw_dv * (px_idx.y as f64 + offset.y as f64))
    }
}
