use crate::Ray;
use crate::VectorN;

pub struct hit_record {
    pub pos: VectorN,
    pub norm: VectorN,
    pub t: f64,
    pub front_face: bool
}

impl hit_record {
    pub fn new() -> hit_record {
        hit_record {
            pos: VectorN::new(Vec::new()),
            norm: VectorN::new(Vec::new()),
            t: 0.,
            front_face: false
        }
    }

    pub fn face_normal(&mut self, r: &Ray, out_norm: &VectorN) {
        self.front_face = r.dir.dot(out_norm) < 0.;
        self.norm = if self.front_face {out_norm.clone()} else {out_norm.clone() * -1.}
    }
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
        let a = ray.dir.clone().lensqr();
        let b = oc.clone().dot(&ray.dir);
        let c = oc.clone().lensqr() - self.radius * self.radius;
        let d = b * b - a * c;
        if d < 0. {
            return false;
        }

        let sqrd = d.sqrt();
        let mut root = (-b - sqrd) / a;
        if root < tmin || tmax < root {
            root = (-b + sqrd) / a;
            if root < tmin || tmax < root {
                return false;
            }
        }

        res.t = root;
        res.pos = ray.at(root);
        let on = (res.pos.clone() - self.coords.clone()) / self.radius;
        res.face_normal(ray, &on);

        true
    }
}
