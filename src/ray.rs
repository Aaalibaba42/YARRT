use crate::{
    vector::VectorN,
};

#[derive(Clone)]
pub struct Ray {
    pub pos: VectorN,
    pub dir: VectorN
}

impl Ray {
    pub fn new(p: &VectorN, d: &VectorN) -> Ray {
        Ray {
            pos: p.clone(),
            dir: d.clone(),
        }
    }

    pub fn at(&self, t: f64) -> VectorN {
        &self.dir * t + &self.pos
    }
}
