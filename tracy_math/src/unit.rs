use tracy_macros::{Random, add, div, mul, sub};

#[derive(Copy, Clone, Random)]
#[add]
#[add(lhs = f64, rhs = f64)]
#[sub]
#[sub(lhs = f64, rhs = f64)]
#[mul]
#[mul(lhs = f64, rhs = f64)]
#[div(lhs = f64, rhs = f64)]
pub struct Degrees(f64);

impl Degrees {
    pub fn new(degrees: f64) -> Self {
        Self(degrees)
    }

    pub fn to_radians(self) -> Radians {
        Radians(f64::to_radians(self.0))
    }

    pub fn to_f64(self) -> f64 {
        self.0
    }
}

#[derive(Copy, Clone, Random)]
#[add]
#[add(lhs = f64, rhs = f64)]
#[sub]
#[sub(lhs = f64, rhs = f64)]
#[mul]
#[mul(lhs = f64, rhs = f64)]
#[div(lhs = f64, rhs = f64)]
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

    pub fn tan(self) -> f64 {
        f64::tan(self.0)
    }
}

pub mod benchmarks {
    use super::*;

    pub fn degrees_to_radians() -> impl Fn() {
        let d = Degrees::new(45.0);
        move || {
            d.to_radians();
        }
    }

    pub fn degrees_to_f64() -> impl Fn() {
        let d = Degrees::new(45.0);
        move || {
            d.to_f64();
        }
    }

    pub fn degrees_random() -> impl Fn() {
        move || {
            Degrees::random();
        }
    }

    pub fn degrees_random_range() -> impl Fn() {
        move || {
            Degrees::random_range(0.0..1.0);
        }
    }

    pub fn radians_to_degrees() -> impl Fn() {
        let r = Radians::new(5.0);
        move || {
            r.to_degrees();
        }
    }

    pub fn radians_to_f64() -> impl Fn() {
        let r = Radians::new(5.0);
        move || {
            r.to_f64();
        }
    }

    pub fn radians_tan() -> impl Fn() {
        let r = Radians::new(5.0);
        move || {
            r.tan();
        }
    }

    pub fn radians_random() -> impl Fn() {
        move || {
            Radians::random();
        }
    }

    pub fn radians_random_range() -> impl Fn() {
        move || {
            Radians::random_range(0.0..1.0);
        }
    }
}
