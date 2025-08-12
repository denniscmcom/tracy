use std::{rc::Rc, time};
use tracy_export::export;
use tracy_math::{ColorRGB, Point3D};
use tracy_render::Renderer;
use tracy_scene::{Cam, Scene, geo::Sphere, mat};

fn main() {
    let start = time::Instant::now();
    run();
    println!("Elapsed: {:?}", start.elapsed());
}

fn run() {
    let cam = Cam::new(400);
    let spheres = vec![
        Sphere {
            orig: Point3D::new(0.0, 0.0, -1.0),
            r: 0.5,
            mat: Rc::new(mat::Lambert {
                albedo: ColorRGB::new(0.0, 0.0, 0.0),
            }),
        },
        Sphere {
            orig: Point3D::new(0.0, -100.5, -1.0),
            r: 100.0,
            mat: Rc::new(mat::Lambert {
                albedo: ColorRGB::new(0.0, 0.0, 0.0),
            }),
        },
    ];

    let scene = Scene::new(cam, spheres);
    let renderer = Renderer {
        samples_per_px: 100,
        depth: 50,
    };

    let buf = renderer.render(scene);
    export(buf, "test").expect("export failed");
}
