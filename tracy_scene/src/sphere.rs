use tracy_math::Point3D;

pub struct Sphere<T> {
    pub orig: Point3D<T>,
    pub r: T,
}

impl<T> Sphere<T> {
    pub fn new(orig: Point3D<T>, r: T) -> Self {
        Self { orig, r }
    }
}
