use std::fs::File;
use std::io::Write;
use crate::vector::*;
use crate::shapes::*;
use crate::ray::*;

pub fn write_color(mut out: File, color: (u8, u8, u8)) -> File {
    let (r, g, b) = color;
    out.write(&format!("{} {} {}\n", r, g, b).as_bytes()).unwrap();
    out
}

pub fn ray_color(ray: Ray) -> VectorN {
    let mut tmp = vec![0.; ray.dir.coords.len()];
    tmp[ray.dir.coords.len() - 1] = -1.;
    let s = Sphere::new(VectorN::new(tmp), 0.5);

    let mut hr = HitRecord::new();
    if s.hit(&ray, &mut hr, 0., f64::MAX) {
        let colors = [VectorN::new(vec![0., 0., 1.]),
                      VectorN::new(vec![0., 1., 0.]),
                      VectorN::new(vec![1., 0., 0.]),
                      VectorN::new(vec![1., 1., 0.]),
                      VectorN::new(vec![1., 0., 1.]),
                      VectorN::new(vec![0., 1., 1.])];
        let mut r = VectorN::new(vec![0., 0., 0.]);
        (0..hr.norm.coords.len()).map(|i| {
            r += &colors[i] * hr.norm.coords[i];
        }).count();
        return VectorN::new(r.unit().coords.iter().map(|&x| if x > 0. {x} else {-x}).collect::<Vec<f64>>());
    }

    let dir = ray.dir.unit();
    let t = 0.5 * (dir.coords[1] + 1.);
    VectorN::new(vec![1., 1., 1.]) * (1. - t)
    + VectorN::new(vec![0.5, 0.7, 1.]) * t
}

pub fn create_outfile(path: &str, dim: u8, resolution: &str) -> File {
    let mut f = File::create(path).unwrap();
    // encoding ascii
    f.write(b"P3\n").unwrap();
    // square image
    for _ in 1..dim {
        f.write(resolution.as_bytes()).unwrap();
        f.write(" ".as_bytes()).unwrap();
    }

    // for dim < 3, make representation as a 2d image (line for 2d->1d,
    // point for 1d->0d)
    // ppm don't support format > 2d
    if dim < 3 {
        for _ in dim..3 {
            f.write("1 ".as_bytes()).unwrap();
        }
    }

    // pixel's color max value
    f.write(b"\n255\n").unwrap();
    f
}
