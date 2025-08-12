use crate::{cam::Cam, geo::Geo};

pub struct Scene<T>
where
    T: Geo,
{
    pub cam: Cam,
    pub geo: T,
}

impl<T> Scene<T>
where
    T: Geo,
{
    pub fn new(cam: Cam, geo: T) -> Self {
        Self { cam, geo }
    }
}
