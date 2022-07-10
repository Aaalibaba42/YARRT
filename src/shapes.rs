use crate::Ray;
use crate::VectorN;

pub struct HitRecord {
    pub pos: VectorN,
    pub norm: VectorN,
    pub t: f64,
    pub front_face: bool
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            pos: VectorN::new(Vec::new()),
            norm: VectorN::new(Vec::new()),
            t: 0.,
            front_face: false
        }
    }

    pub fn face_normal(&mut self, r: &Ray, out_norm: &VectorN) {
        self.front_face = r.dir.dot(out_norm) < 0.;
        self.norm = if self.front_face {out_norm.clone()} else {-out_norm.clone()}
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

    // TODO maybe return an option of HitRecord instead of ref to HitRecord and return bool
    pub fn hit(&self, ray: &Ray, res: &mut HitRecord, tmin: f64, tmax: f64) -> bool {
        let oc = &ray.pos - &self.coords;
        let a = ray.dir.lensqr();
        let b = oc.dot(&ray.dir);
        let c = oc.lensqr() - self.radius * self.radius;
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
        let on = (&res.pos - &self.coords) / self.radius;
        res.face_normal(ray, &on);

        true
    }
}
