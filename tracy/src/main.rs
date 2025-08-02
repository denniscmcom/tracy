use tracy_export::export;
use tracy_math::Point3D;
use tracy_render::render;
use tracy_scene::{Cam, Scene, Sphere};

fn main() {
    run();
}

fn run() {
    let cam = Cam::new(400);
    let img_w = cam.img_w;
    let img_h = cam.img_h;
    let sphere = Sphere::new(Point3D::new(0.0, 0.0, -1.0), 0.5);
    let scene = Scene::new(cam, sphere);
    let buf = render(scene);
    export(&buf, img_w, img_h, "test").expect("export failed");
}
