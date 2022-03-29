use crate::Ray;
use crate::VectorN;

pub struct hit_record {
    pub pos: VectorN,
    pub norm: VectorN,
    pub t: f64
}

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

    // maybe return an option of hit_record instead of ref to hit_record and return bool
    pub fn hit(&self, ray: &Ray, res: &mut hit_record, tmin: f64, tmax: f64) -> bool {
        let oc = ray.pos.clone() - self.coords.clone();
        let a = r.dir.clone().lensqr();
        let b = oc.clone().dot(&r.dir);
        let c = oc.clone().lensqr() - self.radius * self.radius;
        let d = b * b - a * c;
        if d < 0. {
            return false;
        }
        let sqrd = d.sqrt();
        let root = (-b - sqrd) / a;
        if root < tmin || tmax < root {
            root = (-b + sqrd) / a;
            if (root < tmin || tmax < root) {
                return false;
            }
        }

        res.t = root;
        res.pos = r.at(root);
        rec.norm = (res.p - self.pos) / radius;
        return true;
    }
}
