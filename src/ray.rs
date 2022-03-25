use crate::vector::*;

#[derive(Clone)]
pub struct Ray {
    pub pos: VectorN,
    pub dir: VectorN
}

impl Ray {
    pub fn new(p: VectorN, d: VectorN) -> Ray {
        Ray {
            pos: p,
            dir: d
        }
    }

    pub fn at(&self, t: f64) -> VectorN {
        return self.pos.clone() + self.dir.clone() * t;
    }
}
