use crate::Ray;
use crate::VectorN;

#[derive(Clone)]
pub struct Sphere {
    pub coords: VectorN,
    pub radius: f64
}

impl Sphere {
    pub fn new(v: VectorN, r: f64) -> Sphere{
        Sphere {
            coords: v,
            radius: r
        }
    }

    // wanted to call it ishit but I had to change it
    pub fn is_hit(&self, r: &Ray) -> f64 {
        let oc = r.pos.clone() - self.coords.clone();
        let a = r.dir.clone().dot(&r.dir);
        let b = 2. * oc.clone().dot(&r.dir);
        let c = oc.clone().dot(&oc) - self.radius * self.radius;
        let d = b * b - 4. * a * c;
        if d < 0. {
            -1.
        }
        else {
            (-b - d.sqrt()) / (2. * a)
        }
    }
}
