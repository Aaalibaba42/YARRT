use crate::Ray;
use crate::VectorN;

#[derive(Clone)]
pub struct Sphere {
    coords: VectorN,
    radius: f64
}

impl Sphere {
    pub fn new(v: VectorN, r: f64) -> Sphere{
        Sphere {
            coords: v,
            radius: r
        }
    }

    // wanted to call it ishit but I had to change it
    pub fn is_hit(&self, r: &Ray) -> bool {
        let oc = r.pos.clone() - self.coords.clone();
        let a = r.dir.clone().dot(&r.dir);
        let b = 2. * oc.clone().dot(&r.dir);
        let c = oc.clone().dot(&oc) - self.radius * self.radius;
        b * b - 4. * a * c > 0.
    }
}
