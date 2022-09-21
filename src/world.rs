use crate::{
    ray::Ray,
    shapes::{
        Sphere,
        HitRecord,
    },
};

pub struct World {
    pub objs: Vec<Sphere>,
    pub cam: Ray,
    pub dim: u8,
    pub res: u32,
    pub viewport_size: f64,
    pub focal_len: f64,
}

impl World {
    pub fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        // not a very rusty implem but whatever
        // okay it's very bad code.
        let mut tmp_hr: Option<HitRecord> = None;
        let mut closest: f64 = tmax;
        for obj in self.objs.iter() {
            match obj.hit(ray, tmin, closest) {
                Some(hr) => {
                    closest = hr.t;
                    tmp_hr = Some(hr);
                },
                _ => {},
            }
        }
        tmp_hr
    }
}
