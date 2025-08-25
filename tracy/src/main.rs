use rand::prelude::*;
use std::{sync::Arc, time};
use tracy_export::export;
use tracy_macros::Random;
use tracy_math::{ColorRGB, Point3D, Vec3D, unit::Degrees};
use tracy_render::Renderer;
use tracy_scene::{Scene, cam::CamBuilder, geo::Sphere, mat};

fn main() {
    let start = time::Instant::now();
    run();
    println!("Total: {:?}", start.elapsed());
}

fn run() {
    let cam_builder = CamBuilder {
        orig: Point3D::new(13.0, 2.0, 3.0),
        at: Point3D::new(0.0, 0.0, 0.0),
        up: Vec3D::new(0.0, 1.0, 0.0),
        img_w: 400,
        fov: Degrees::new(20.0),
        defocus_angle: Degrees::new(0.6),
        focus_dist: 10.0,
        fps: 24,
        shutter_speed: time::Duration::from_secs_f64(1.0 / 48.0),
        frames: 1,
    };

    let cam = cam_builder.build();
    let mut spheres = Vec::new();

    // Ground.
    spheres.push(Sphere {
        orig: Point3D::new(0.0, -1000.0, 0.0),
        r: 1000.0,
        mat: Arc::new(mat::Lambert {
            albedo: ColorRGB::new(0.5, 0.5, 0.5),
        }),
        orig_2: None,
    });

    let mut rng = rand::rng();
    rng.random_range(0..1);

    for a in -11..11 {
        for b in -11..11 {
            let orig = Point3D {
                x: a as f64 + 0.9 * rng.random_range(0.0..1.0),
                y: 0.2,
                z: b as f64 + 0.9 * rng.random_range(0.0..1.0),
            };

            if (orig - Point3D::new(4.0, 0.2, 0.0)).len_2().sqrt() > 0.9 {
                match rng.random_range(0.0..1.0) {
                    // Diffuse.
                    0.0..0.8 => {
                        let color_a = ColorRGB::random_range(0.0..1.0);
                        let color_b = ColorRGB::random_range(0.0..1.0);
                        spheres.push(Sphere {
                            orig,
                            r: 0.2,
                            mat: Arc::new(mat::Lambert {
                                albedo: color_a * color_b,
                            }),
                            orig_2: Some(orig + Vec3D::new(0.0, 0.5, 0.0)),
                            // orig_2: None,
                        });
                    }
                    // Metal.
                    0.0..0.95 => {
                        spheres.push(Sphere {
                            orig,
                            r: 0.2,
                            mat: Arc::new(mat::Metal {
                                albedo: ColorRGB::random_range(0.0..1.0),
                                fuzz: rng.random_range(0.0..0.5),
                            }),
                            orig_2: None,
                        });
                    }
                    // Glass.
                    _ => {
                        spheres.push(Sphere {
                            orig,
                            r: 0.2,
                            mat: Arc::new(mat::Dielectric { refract_idx: 1.5 }),
                            orig_2: None,
                        });
                    }
                }
            }
        }
    }

    spheres.push(Sphere {
        orig: Point3D::new(0.0, 1.0, 0.0),
        r: 1.0,
        mat: Arc::new(mat::Dielectric { refract_idx: 1.5 }),
        orig_2: None,
    });

    spheres.push(Sphere {
        orig: Point3D::new(-4.0, 1.0, 0.0),
        r: 1.0,
        mat: Arc::new(mat::Lambert {
            albedo: ColorRGB::new(0.4, 0.2, 0.1),
        }),
        orig_2: None,
    });

    spheres.push(Sphere {
        orig: Point3D::new(4.0, 1.0, 0.0),
        r: 1.0,
        mat: Arc::new(mat::Metal {
            albedo: ColorRGB::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        }),
        orig_2: None,
    });

    let scene = Scene::new(cam, spheres);
    let renderer = Renderer {
        spp: 100,
        depth: 50,
    };

    let start_render = time::Instant::now();
    let buf = renderer.render(&scene);
    println!("Render: {:?}", start_render.elapsed());

    let start_export = time::Instant::now();
    export(buf, "test").expect("export failed");
    println!("Export: {:?}", start_export.elapsed());
}
