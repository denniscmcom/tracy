use std::{f64::consts::PI, rc::Rc, time};
use tracy_export::export;
use tracy_math::{ColorRGB, Point3D};
use tracy_render::Renderer;
use tracy_scene::{Cam, Scene, cam, geo::Sphere, mat};

fn main() {
    let start = time::Instant::now();
    run();
    println!("Elapsed: {:?}", start.elapsed());
}

fn run() {
    let cam = Cam::new(400, cam::Degrees(90.0));
    let r = f64::cos(PI / 4.0);
    let spheres = vec![
        Sphere {
            orig: Point3D::new(-r, 0.0, -1.0),
            r,
            mat: Rc::new(mat::Lambert {
                albedo: ColorRGB::new(0.0, 0.0, 1.0),
            }),
        },
        Sphere {
            orig: Point3D::new(r, 0.0, -1.0),
            r,
            mat: Rc::new(mat::Lambert {
                albedo: ColorRGB::new(1.0, 0.0, 0.0),
            }),
        },
    ];

    let scene = Scene::new(cam, spheres);
    let renderer = Renderer {
        samples_per_px: 50,
        depth: 12,
    };

    let buf = renderer.render(scene);
    export(buf, "test").expect("export failed");
}
