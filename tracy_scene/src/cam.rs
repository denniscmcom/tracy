use tracy_math::{Point2D, Point3D, Vec3D};

pub struct Cam {
    pub focal_len: f64,
    pub orig: Point3D<f64>,
    pub img_w: usize,
    pub img_h: usize,
    pub viewport_w: f64,
    pub viewport_h: f64,
    pub viewport_u: Vec3D<f64>,
    pub viewport_v: Vec3D<f64>,
    pub viewport_orig: Point3D<f64>,
    pub pixel_delta_u: Vec3D<f64>,
    pub pixel_delta_v: Vec3D<f64>,
    pub pixel_0_orig: Point3D<f64>,
}

impl Cam {
    pub fn new(img_w: usize) -> Self {
        let ideal_aspect_ratio = 16.0 / 9.0;
        let img_h = (img_w as f64 / ideal_aspect_ratio) as usize;
        let actual_aspect_ratio = img_w as f64 / img_h as f64;

        let viewport_h = 2.0;
        let viewport_w = viewport_h * actual_aspect_ratio;

        let viewport_u = Vec3D {
            x: viewport_w,
            y: 0.0,
            z: 0.0,
        };

        let viewport_v = Vec3D {
            x: 0.0,
            y: -viewport_h,
            z: 0.0,
        };

        let pixel_delta_u = viewport_u / img_w as f64;
        let pixel_delta_v = viewport_v / img_h as f64;

        let orig = Point3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        let focal_len = 1.0;
        let viewport_orig =
            orig - Vec3D {
                x: 0.0,
                y: 0.0,
                z: focal_len,
            } - viewport_u / 2.0
                - viewport_v / 2.0;

        let pixel_0_orig =
            viewport_orig + (pixel_delta_u + pixel_delta_v) * 0.5;

        Self {
            focal_len,
            orig,
            img_w,
            img_h,
            viewport_w,
            viewport_h,
            viewport_u,
            viewport_v,
            viewport_orig,
            pixel_delta_u,
            pixel_delta_v,
            pixel_0_orig,
        }
    }

    pub fn compute_pixel_center(&self, orig: Point2D<usize>) -> Point3D<f64> {
        self.pixel_0_orig
            + (self.pixel_delta_u * orig.x as f64)
            + (self.pixel_delta_v * orig.y as f64)
    }
}
