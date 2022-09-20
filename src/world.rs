use crate::{
    ray::Ray,
    shapes::Sphere,
};

pub struct World {
    pub objs: Vec<Sphere>,
    pub cam: Ray,
    pub dim: u8,
    pub res: u32,
    pub viewport_size: f64,
    pub focal_len: f64,
}
