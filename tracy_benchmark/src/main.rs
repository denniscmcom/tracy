use tracy_benchmark::bench;
use tracy_math::{
    color::benchmarks::{
        color_rgb_random, color_rgb_random_range, color_rgb_scale,
        color_rgb_to_gamma,
    },
    ray::bechmarks::ray_at,
    unit::benchmarks::{
        degrees_random, degrees_random_range, degrees_to_f64,
        degrees_to_radians, radians_random, radians_random_range, radians_tan,
        radians_to_degrees, radians_to_f64,
    },
    vec::benchmarks::{
        vec2d_len_2, vec2d_random, vec2d_random_range, vec3d_cross, vec3d_dot,
        vec3d_len_2, vec3d_normalize, vec3d_random, vec3d_random_range,
    },
};
use tracy_render::benchmarks::{renderer_render, renderer_trace};
use tracy_scene::geo::sphere::benchmarks::sphere_hit;

fn main() {
    bench_render();
    bench_math();
    bench_scene();
}

fn bench_render() {
    println!("---");
    println!("render");
    bench("Renderer.render", renderer_render());
    bench("Renderer.trace", renderer_trace());
}

fn bench_math() {
    println!("---");
    println!("math");
    bench("ColorRGB.scale", color_rgb_scale());
    bench("ColorRGB.to_gamma", color_rgb_to_gamma());
    bench("ColorRGB.random", color_rgb_random());
    bench("ColorRGB.random_range", color_rgb_random_range());
    bench("Ray.at", ray_at());
    bench("Degrees.to_radians", degrees_to_radians());
    bench("Degrees.to_f64", degrees_to_f64());
    bench("Degrees.random", degrees_random());
    bench("Degrees.random_range", degrees_random_range());
    bench("Radians.to_degrees", radians_to_degrees());
    bench("Radians.to_f64", radians_to_f64());
    bench("Radians.tan", radians_tan());
    bench("Radians.random", radians_random());
    bench("Radians.random_range", radians_random_range());
    bench("Vec3D.len_2", vec3d_len_2());
    bench("Vec3D.dot", vec3d_dot());
    bench("Vec3D.cross", vec3d_cross());
    bench("Vec3D.normalize", vec3d_normalize());
    bench("Vec3D.random", vec3d_random());
    bench("Vec3D.random_range", vec3d_random_range());
    bench("Vec2D.len_2", vec2d_len_2());
    bench("Vec2D.random", vec2d_random());
    bench("Vec2D.random_range", vec2d_random_range());
}

fn bench_scene() {
    println!("---");
    println!("math");
    bench("Sphere.hit", sphere_hit());
}
