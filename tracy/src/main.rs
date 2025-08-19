use std::{rc::Rc, time};
use tracy_export::export;
use tracy_math::{ColorRGB, Point3D, Vec3D, unit::Degrees};
use tracy_render::Renderer;
use tracy_scene::{Scene, cam::CamBuilder, geo::Sphere, mat};

fn main() {
    let start = time::Instant::now();
    run();
    println!("Elapsed: {:?}", start.elapsed());
}

fn run() {
    let cam_builder = CamBuilder {
        orig: Point3D::new(-2.0, 2.0, 1.0),
        at: Point3D::new(0.0, 0.0, -1.0),
        up: Vec3D::new(0.0, 1.0, 0.0),
        img_w: 400,
        fov: Degrees::new(20.0),
        defocus_angle: Degrees::new(10.0),
        focus_dist: 3.4,
    };

    let cam = cam_builder.build();

    let spheres = vec![
        Sphere {
            orig: Point3D::new(0.0, -100.5, -1.0),
            r: 100.0,
            mat: Rc::new(mat::Lambert {
                albedo: ColorRGB::new(0.8, 0.8, 0.0),
            }),
        },
        Sphere {
            orig: Point3D::new(0.0, 0.0, -1.2),
            r: 0.5,
            mat: Rc::new(mat::Lambert {
                albedo: ColorRGB::new(0.1, 0.2, 0.5),
            }),
        },
        Sphere {
            orig: Point3D::new(-1.0, 0.0, -1.0),
            r: 0.5,
            mat: Rc::new(mat::Dielectric { refract_idx: 1.5 }),
        },
        Sphere {
            orig: Point3D::new(-1.0, 0.0, -1.0),
            r: 0.4,
            mat: Rc::new(mat::Dielectric {
                refract_idx: 1.0 / 1.5,
            }),
        },
        Sphere {
            orig: Point3D::new(1.0, 0.0, -1.0),
            r: 0.5,
            mat: Rc::new(mat::Metal {
                albedo: ColorRGB::new(0.8, 0.6, 0.2),
                fuzz: 1.0,
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
