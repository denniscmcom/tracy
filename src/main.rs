use rand::prelude::*;
use rayon::prelude::*;
use std::f64::consts::PI;
use std::fs;
use std::io::prelude::*;
use std::ops;
use std::path;
use std::time;

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;
const MAX_RAY_DEPTH: u32 = 5;
const RAND_SCENES: usize = 10;

type Pixel = (u8, u8, u8);
type Buffer = Vec<Pixel>;

fn main() {
    let start = time::Instant::now();
    let default_camera = Camera::default();
    let default_scene = Scene::default();
    run(&default_scene, &default_camera, 0);

    (1..=RAND_SCENES).into_par_iter().for_each(|i| {
        let camera = Camera::default();
        let mut scene = Scene::new();
        scene.set_light(
            Vec3 {
                x: 0.0,
                y: 20.0,
                z: -30.0,
            },
            Vec3 {
                x: 3.0,
                y: 3.0,
                z: 3.0,
            },
            3.0,
        );

        run(&scene, &camera, i);
    });

    let end = start.elapsed();
    println!("[PROGRAM]: {:?}", end);
}

fn run(scene: &Scene, camera: &Camera, job_id: usize) {
    let start_render = time::Instant::now();
    let buffer = render(&scene, &camera);
    let end_render = start_render.elapsed();
    println!("[RENDER {job_id}]: {:?}", end_render);

    let start_export = time::Instant::now();
    let filename = format!("job_{job_id}");
    export(&buffer, &filename).expect("failed to export");
    let end_export = start_export.elapsed();
    println!("[EXPORT {job_id}]: {:?}", end_export);

    println!("[JOB {job_id}]: {:?}", end_render + end_export);
}

fn export(buffer: &Buffer, filename: &str) -> std::io::Result<()> {
    let folder_name = "render";
    fs::create_dir_all(folder_name)?;
    let mut file_path = path::PathBuf::from("render");
    file_path.push(format!("{}.ppm", filename));
    let mut file = fs::File::create(file_path)?;
    let header = format!("P6\n{} {}\n255\n", WIDTH, HEIGHT);
    file.write_all(header.as_bytes())?;

    for pixel in buffer {
        file.write_all(&[pixel.0, pixel.1, pixel.2])?;
    }

    Ok(())
}

fn render(scene: &Scene, camera: &Camera) -> Buffer {
    let mut buffer: Buffer = vec![];

    for pixel_y in 0..HEIGHT {
        for pixel_x in 0..WIDTH {
            let screen_x =
                (2.0 * ((pixel_x as f64 + 0.5) * (1.0 / WIDTH as f64)) - 1.0)
                    * camera.compute_angle()
                    * (WIDTH as f64 / HEIGHT as f64);

            let screen_y = (1.0
                - 2.0 * ((pixel_y as f64 + 0.5) * (1.0 / HEIGHT as f64)))
                * camera.compute_angle();

            let mut ray = Ray {
                direction: Vec3 {
                    x: screen_x,
                    y: screen_y,
                    z: -1.0,
                },
                origin: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                depth: 0,
            };

            ray.direction.normalize();
            let pixel = ray.trace(&scene).to_pixel();
            buffer.push(pixel);
        }
    }

    buffer
}

struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    fn to_pixel(&self) -> Pixel {
        (
            (self.x.clamp(0.0, 1.0) * 255.0) as u8,
            (self.y.clamp(0.0, 1.0) * 255.0) as u8,
            (self.z.clamp(0.0, 1.0) * 255.0) as u8,
        )
    }

    fn normalize(&mut self) {
        let norm2 = self.x * self.x + self.y * self.y + self.z * self.z;

        if norm2 > 0.0 {
            let inv_norm = 1.0 / norm2.sqrt();
            self.x *= inv_norm;
            self.y *= inv_norm;
            self.z *= inv_norm;
        }
    }

    fn dot(&self, v: &Vec3) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl ops::Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: &Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Mul<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Mul<&f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: &f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::MulAssign<&f64> for Vec3 {
    fn mul_assign(&mut self, rhs: &f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

struct Camera {
    pub fov: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Self { fov: 30.0 }
    }
}

impl Camera {
    fn compute_angle(&self) -> f64 {
        (PI * 0.5 * self.fov / 180.0).tan()
    }
}

struct Scene {
    pub spheres: Vec<Sphere>,
}

impl Scene {
    fn new() -> Self {
        let mut spheres: Vec<Sphere> = vec![Sphere {
            // Floor.
            position: Vec3 {
                x: 0.0,
                y: -10004.0,
                z: -20.0,
            },
            surface_color: Vec3 {
                x: 0.2,
                y: 0.2,
                z: 0.2,
            },
            emission_color: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            radius: 10000.0,
            transparency: 0.0,
            reflection: 0.0,
        }];

        let mut rng = rand::rng();
        let num_spheres = rng.random_range(4..=10);

        for _ in 0..=num_spheres {
            let sphere = Sphere {
                position: Vec3 {
                    x: rng.random_range(-7.0..=7.0),
                    y: rng.random_range(0.0..=2.0),
                    z: rng.random_range(-35.0..=-15.0),
                },
                surface_color: Vec3 {
                    x: rng.random_range(0.0..=1.0),
                    y: rng.random_range(0.0..=1.0),
                    z: rng.random_range(0.0..=1.0),
                },
                emission_color: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                radius: rng.random_range(1..=3) as f64,
                transparency: rng.random_range(0.0..=1.0),
                reflection: rng.random_range(0.0..=1.0),
            };

            spheres.push(sphere);
        }

        Self { spheres }
    }

    fn set_light(&mut self, position: Vec3, emission_color: Vec3, radius: f64) {
        self.spheres.push(Sphere {
            position,
            surface_color: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            emission_color,
            radius,
            transparency: 0.0,
            reflection: 0.0,
        });
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            spheres: vec![
                Sphere {
                    // Floor.
                    position: Vec3 {
                        x: 0.0,
                        y: -10004.0,
                        z: -20.0,
                    },
                    surface_color: Vec3 {
                        x: 0.2,
                        y: 0.2,
                        z: 0.2,
                    },
                    emission_color: Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    radius: 10000.0,
                    transparency: 0.0,
                    reflection: 0.0,
                },
                Sphere {
                    position: Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: -20.0,
                    },
                    surface_color: Vec3 {
                        x: 1.0,
                        y: 0.32,
                        z: 0.36,
                    },
                    emission_color: Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    radius: 4.0,
                    transparency: 0.5,
                    reflection: 1.0,
                },
                Sphere {
                    position: Vec3 {
                        x: 5.0,
                        y: -1.0,
                        z: -15.0,
                    },
                    surface_color: Vec3 {
                        x: 0.9,
                        y: 0.76,
                        z: 0.46,
                    },
                    emission_color: Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    radius: 2.0,
                    transparency: 0.0,
                    reflection: 1.0,
                },
                Sphere {
                    position: Vec3 {
                        x: 5.0,
                        y: 0.0,
                        z: -25.0,
                    },
                    surface_color: Vec3 {
                        x: 0.65,
                        y: 0.77,
                        z: 0.97,
                    },
                    emission_color: Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    radius: 3.0,
                    transparency: 0.0,
                    reflection: 1.0,
                },
                Sphere {
                    position: Vec3 {
                        x: -5.5,
                        y: 0.0,
                        z: -15.0,
                    },
                    surface_color: Vec3 {
                        x: 0.90,
                        y: 0.90,
                        z: 0.90,
                    },
                    emission_color: Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    radius: 3.0,
                    transparency: 0.0,
                    reflection: 1.0,
                },
                // Light.
                Sphere {
                    position: Vec3 {
                        x: 0.0,
                        y: 20.0,
                        z: -30.0,
                    },
                    surface_color: Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    emission_color: Vec3 {
                        x: 3.0,
                        y: 3.0,
                        z: 3.0,
                    },
                    radius: 3.0,
                    transparency: 0.0,
                    reflection: 0.0,
                },
            ],
        }
    }
}

struct Sphere {
    pub position: Vec3,
    pub surface_color: Vec3,
    pub emission_color: Vec3,
    pub radius: f64,
    pub transparency: f64,
    pub reflection: f64,
}

impl Sphere {
    fn intersect(&self, ray: &Ray) -> Option<(f64, f64)> {
        let l = &self.position - &ray.origin;
        let tca = l.dot(&ray.direction);

        if tca < 0.0 {
            return None;
        }

        let d2 = l.dot(&l) - tca * tca;
        let radius2 = self.radius * self.radius;

        if d2 > radius2 {
            return None;
        }

        let thc = (radius2 - d2).sqrt();
        let t0 = tca - thc;
        let t1 = tca + thc;
        Some((t0, t1))
    }
}

struct Ray {
    direction: Vec3,
    origin: Vec3,
    depth: i32,
}

impl Ray {
    fn trace(&self, scene: &Scene) -> Vec3 {
        let mut tnear = f64::MAX;
        let mut active_sphere: Option<&Sphere> = None;

        for sphere in scene.spheres.iter() {
            if let Some((mut t0, t1)) = sphere.intersect(self) {
                if t0 < 0.0 {
                    t0 = t1;
                }

                if t0 < tnear {
                    tnear = t0;
                    active_sphere = Some(sphere);
                }
            }
        }

        if let Some(active_sphere) = active_sphere {
            let mut surface_color = Vec3::default();
            let point_hit = &self.origin + &(&self.direction * &tnear);
            let mut normal_hit = &point_hit - &active_sphere.position;
            normal_hit.normalize();
            let bias = 1e-4;
            let mut is_inside_sphere = false;

            if self.direction.dot(&normal_hit) > 0.0 {
                normal_hit *= &-1.0;
                is_inside_sphere = true;
            }

            if (active_sphere.transparency > 0.0
                || active_sphere.reflection > 0.0)
                && self.depth < MAX_RAY_DEPTH as i32
            {
                let facing_ratio = self.direction.dot(&normal_hit) * -1.0;
                let fresnel_effect =
                    interpolate(&(1.0 - (&facing_ratio)).powf(3.0), &1.0, &0.1);

                let mut reflection_ray = Ray {
                    direction: &self.direction
                        - &(&normal_hit
                            * &(2.0 * self.direction.dot(&normal_hit))),
                    origin: &point_hit + &(&normal_hit * &bias),
                    depth: self.depth + 1,
                };

                reflection_ray.direction.normalize();
                let reflection_value = reflection_ray.trace(&scene);
                let mut refraction_value: Option<Vec3> = None;

                if active_sphere.transparency > 0.0 {
                    let ior = 1.1;
                    let eta = if is_inside_sphere {
                        &ior
                    } else {
                        &(1.0 / &ior)
                    };

                    let cosi = normal_hit.dot(&self.direction) * -1.0;
                    let k = 1.0 - eta * eta * (1.0 - cosi * cosi);
                    let mut refraction_ray = Ray {
                        direction: &(&self.direction * eta)
                            + &(&normal_hit * &(eta * cosi - k.sqrt())),
                        origin: &point_hit - &(&normal_hit * &bias),
                        depth: self.depth + 1,
                    };

                    refraction_ray.direction.normalize();
                    refraction_value = Some(refraction_ray.trace(&scene));
                }

                surface_color = &(&(&reflection_value * &fresnel_effect)
                    + &(&(&refraction_value.unwrap_or_default()
                        * &(1.0 - fresnel_effect))
                        * &active_sphere.transparency))
                    * &active_sphere.surface_color;
            } else {
                for (i, sphere) in scene.spheres.iter().enumerate() {
                    if sphere.emission_color.x > 0.0 {
                        let mut transmission = Vec3 {
                            x: 1.0,
                            y: 1.0,
                            z: 1.0,
                        };

                        let mut light_ray = Ray {
                            direction: &sphere.position - &point_hit,
                            origin: &point_hit + &(&normal_hit * &bias),
                            depth: 0,
                        };

                        light_ray.direction.normalize();

                        for (j, sphere) in scene.spheres.iter().enumerate() {
                            if i == j {
                                continue;
                            }

                            if let Some((_, _)) = sphere.intersect(&light_ray) {
                                transmission = Vec3::default();
                                break;
                            }
                        }

                        surface_color += &(&(&(&active_sphere.surface_color
                            * &transmission)
                            * &f64::max(
                                0.0,
                                normal_hit.dot(&light_ray.direction),
                            ))
                            * &scene.spheres[i].emission_color);
                    }
                }
            }

            surface_color += &active_sphere.emission_color;
            return surface_color;
        } else {
            return Vec3 {
                x: 2.0,
                y: 2.0,
                z: 2.0,
            };
        }
    }
}

fn interpolate(a: &f64, b: &f64, weight: &f64) -> f64 {
    b * *weight + a * (1.0 - *weight)
}
