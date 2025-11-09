pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    if t > 1.0 {
        panic!("`t` should be 0.0..=1.0");
    }

    (1.0 - t) * a + t * b
}
