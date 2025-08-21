use tracy_macros::{Random, add, div, mul, sub};

#[derive(Clone, Copy, Default, Random)]
#[add]
#[sub]
#[mul]
#[mul(rhs = T)]
#[div(rhs = T)]
pub struct ColorRGB<T>
where
    T: Clone,
{
    pub r: T,
    pub g: T,
    pub b: T,
}

impl<T> ColorRGB<T>
where
    T: Clone,
{
    pub fn new(r: T, g: T, b: T) -> Self {
        Self { r, g, b }
    }
}

impl ColorRGB<f64> {
    pub fn scale(self) -> ColorRGB<u8> {
        ColorRGB {
            r: (self.r.clamp(0.0, 1.0) * 255.0) as u8,
            g: (self.g.clamp(0.0, 1.0) * 255.0) as u8,
            b: (self.b.clamp(0.0, 1.0) * 255.0) as u8,
        }
    }

    pub fn to_gamma(&self) -> ColorRGB<f64> {
        Self {
            r: f64::max(self.r, 0.0).sqrt(),
            g: f64::max(self.g, 0.0).sqrt(),
            b: f64::max(self.b, 0.0).sqrt(),
        }
    }
}

pub mod benchmarks {
    use super::*;

    pub fn color_rgb_scale() -> impl Fn() {
        let color = ColorRGB::new(1.0, 2.0, 3.0);
        move || {
            color.scale();
        }
    }

    pub fn color_rgb_to_gamma() -> impl Fn() {
        let color = ColorRGB::new(1.0, 2.0, 3.0);
        move || {
            color.to_gamma();
        }
    }

    pub fn color_rgb_random() -> impl Fn() {
        move || {
            ColorRGB::<f64>::random();
        }
    }

    pub fn color_rgb_random_range() -> impl Fn() {
        move || {
            ColorRGB::<f64>::random_range(0.0..1.0);
        }
    }
}
