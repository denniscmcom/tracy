mod ray;

use crate::ray::Ray;
use tracy_math::{ColorRGB, Point2D};
use tracy_scene::Scene;

pub type Buf = Vec<ColorRGB<u8>>;

pub fn render(scene: Scene) -> Vec<ColorRGB<u8>> {
    let mut buf: Buf = vec![];

    for y in 0..scene.cam.img_h {
        for x in 0..scene.cam.img_w {
            let pixel_center = scene.cam.compute_pixel_center(Point2D { x, y });
            let ray = Ray {
                orig: scene.cam.orig,
                dir: scene.cam.orig - pixel_center,
            };

            let pixel = ray.trace(&scene.sphere);
            buf.push(pixel.to_u8());
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
