mod ray;

use crate::ray::Ray;
use tracy_math::{ColorRGB, Point2D};
use tracy_scene::Scene;

pub type Buf = Vec<ColorRGB<u8>>;

pub fn render(scene: Scene) -> Buf {
    let mut buf: Buf = vec![];
    let cam = scene.cam;

    for y in 0..cam.img_h {
        for x in 0..cam.img_w {
            let px_center = cam.calc_px_center(Point2D { x, y });
            let ray = Ray::new(cam.orig, px_center - cam.orig);
            let px = ray.trace(&scene.sphere);
            buf.push(px.to_u8());
        }
    }

    buf
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         assert_eq!(1, 0);
//     }
// }
