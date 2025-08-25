use std::time;

pub struct Keyframe<T> {
    pub val: T,
    pub ts: time::Duration,
}
