use tracy_macros::{add, mul};

#[derive(Clone, Copy, Default)]
#[add]
#[mul]
#[mul(rhs = T)]
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
