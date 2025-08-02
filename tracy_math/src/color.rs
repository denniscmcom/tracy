use tracy_macros::{add, mul};

#[derive(Clone, Copy, Default)]
#[add]
#[mul(rhs = T)]
pub struct ColorRGB<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl<T> ColorRGB<T> {
    pub fn new(r: T, g: T, b: T) -> Self {
        Self { r, g, b }
    }
}

impl ColorRGB<f64> {
    pub fn to_u8(self) -> ColorRGB<u8> {
        ColorRGB {
            r: (self.r.clamp(0.0, 1.0) * 255.0) as u8,
            g: (self.g.clamp(0.0, 1.0) * 255.0) as u8,
            b: (self.b.clamp(0.0, 1.0) * 255.0) as u8,
        }
    }
}
