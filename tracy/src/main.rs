use std::time;
use tracy_export::export;
use tracy_math::Point3D;
use tracy_render::Renderer;
use tracy_scene::{Cam, Scene, Sphere};

fn main() {
    let start = time::Instant::now();
    run();
    println!("Elapsed: {:?}", start.elapsed());
}

fn run() {
    let cam = Cam::new(400);
    let spheres = vec![
        Sphere::new(Point3D::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Point3D::new(0.0, -100.5, -1.0), 100.0),
    ];

    let scene = Scene::new(cam, spheres);
    let renderer = Renderer {
        samples_per_px: 100,
        depth: 50,
    };

    let buf = renderer.render(scene);
    export(buf, "test").expect("export failed");
}
