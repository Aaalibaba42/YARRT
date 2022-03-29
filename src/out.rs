use std::fs::File;
use std::io::Write;
use crate::vector::*;
use crate::shapes::*;
use crate::ray::*;

pub fn write_color(mut out: File, color: (u8, u8, u8)) -> File {
    // TODO use/learn format!()
    let (r, g, b) = color;
    out.write(r.to_string().as_bytes()).unwrap();
    out.write(" ".as_bytes()).unwrap();
    out.write(g.to_string().as_bytes()).unwrap();
    out.write(" ".as_bytes()).unwrap();
    out.write(b.to_string().as_bytes()).unwrap();
    out.write("\n".as_bytes()).unwrap();

    out
}

pub fn ray_color(ray: Ray) -> VectorN {
    let mut tmp = vec![0.; ray.dir.coords.len()];
    tmp[ray.dir.coords.len() - 1] = -1.;
    let s = Sphere::new(VectorN::new(tmp), 0.5);
    let t = s.hit(&ray);
    if t > 0. {
        let r = (ray.at(t) - s.coords.clone()).unit();
        // temporary 3d only colors
        return VectorN::new(vec![r.coords[0] + 1., r.coords[1] + 1., r.coords[2] + 1.]) * 0.5;
    }
    // only works (as intended) in 3d (TODO nD)
    // simple 'gradiant'
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
