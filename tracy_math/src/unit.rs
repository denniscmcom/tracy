use tracy_macros::{add, div, mul, sub};

#[add]
#[sub]
#[mul]
#[div]
pub struct Degrees(f64);

impl Degrees {
    pub fn new(degrees: f64) -> Self {
        Self(degrees)
    }

    pub fn to_radians(self) -> Radians {
        Radians(f64::to_radians(self.0))
    }
}

#[add]
#[sub]
#[mul]
#[div]
pub struct Radians(f64);

impl Radians {
    pub fn new(radians: f64) -> Self {
        Self(radians)
    }

    pub fn to_degrees(self) -> Degrees {
        Degrees(f64::to_degrees(self.0))
    }

    pub fn to_f64(self) -> f64 {
        self.0
    }

    pub fn tan(self) -> Radians {
        Radians(f64::tan(self.0))
    }
}
