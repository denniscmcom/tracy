use tracy_macros::{Color, Convert, Random, ops};
use tracy_traits::ColorType;

#[derive(Clone, Copy, Default, Random, Convert, Color)]
#[ops(add, sub, mul, mul(rhs = T), div(rhs = T))]
pub struct ColorRGB<T>
where
    T: Clone + Copy + ColorType,
{
    pub r: T,
    pub g: T,
    pub b: T,
}

impl<T> ColorRGB<T>
where
    T: Clone + Copy + ColorType,
{
    pub fn new(r: T, g: T, b: T) -> Self {
        Self { r, g, b }
    }

    pub fn to_rgba(&self, a: T) -> ColorRGBA<T> {
        ColorRGBA::new(self.r, self.g, self.b, a)
    }
}

#[derive(Clone, Copy, Default, Random, Convert, Color)]
#[ops(add, sub, mul, mul(rhs = T), div(rhs = T))]
pub struct ColorRGBA<T>
where
    T: Clone + Copy + ColorType,
{
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T,
}

impl<T> ColorRGBA<T>
where
    T: Clone + Copy + ColorType,
{
    pub fn new(r: T, g: T, b: T, a: T) -> Self {
        Self { r, g, b, a }
    }

    pub fn to_rgb(&self) -> ColorRGB<T> {
        ColorRGB::new(self.r, self.g, self.b)
    }
}

pub mod benchmarks {
    use super::*;

    pub fn color_rgb_scale() -> impl Fn() {
        let color = ColorRGB::new(1.0, 2.0, 3.0);
        move || {
            color.to_u8();
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
