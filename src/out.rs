use std::{
    fs::File,
    io::Write,
};
use crate::{
    vector::VectorN,
    ray::Ray,
    my_error_ts::MyErrorTs,
    world::World,
};

pub fn write_color(out: &mut File, color: (u8, u8, u8)) -> Result<(), MyErrorTs> {
    let (r, g, b) = color;
    out.write(&format!("{} {} {}\n", r, g, b).as_bytes()).map_err(MyErrorTs::IO)?;
    Ok(())
}

pub fn ray_color(ray: Ray, world: &World) -> VectorN {
    let hit = world.objs[0].hit(&ray, 0., f64::MAX);
    return match hit {
        Some(hr) => {
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
            VectorN::new(
                r.unit().coords.iter().map(|&x| if x > 0. {x} else {-x}
            ).collect::<Vec<f64>>())
        },
        None => {
            let dir = ray.dir.unit();
            let t = 0.5 * (dir.coords[1] + 1.);
            VectorN::new(vec![1., 1., 1.]) * (1. - t)
            + VectorN::new(vec![0.5, 0.7, 1.]) * t
        }
    }
}

pub fn create_outfile(path: &str, dim: u8, resolution: &str) -> Result<File, MyErrorTs> {
    let mut f = File::create(path).map_err(MyErrorTs::IO)?;
    // encoding ascii
    f.write(b"P3\n").map_err(MyErrorTs::IO)?;
    // square image
    for _ in 1..dim {
        f.write(resolution.as_bytes()).map_err(MyErrorTs::IO)?;
        f.write(" ".as_bytes()).map_err(MyErrorTs::IO)?;
    }

    // for dim < 3, make representation as a 2d image (line for 2d->1d,
    // point for 1d->0d)
    // ppm don't support format > 2d
    if dim < 3 {
        for _ in dim..3 {
            f.write("1 ".as_bytes()).map_err(MyErrorTs::IO)?;
        }
    }

    // pixel's color max value
    f.write(b"\n255\n").map_err(MyErrorTs::IO)?;
    Ok(f)
}
