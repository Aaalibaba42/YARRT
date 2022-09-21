mod vector;
mod ray;
mod world;
mod shapes;
mod out;
mod my_error_ts;

use std::{
    env,
    io::{
        stderr,
        Write
    },
};
use crate::{
    out::*,
    vector::VectorN,
    ray::Ray,
    my_error_ts::MyErrorTs,
    shapes::Sphere,
    world::World,
};

fn incr(v: &mut Vec<u32>, size: u32) {
    let mut tmp = 1;
    for nb in v.iter_mut() {
        *nb += tmp;
        tmp = *nb/size;
        *nb %= size;
    }
}

fn main() -> Result<(), MyErrorTs> {
    // -- Bad command --
    (if env::args().count() != 4 { Err(MyErrorTs::UsageError) } else { Ok(()) })?;

    // -- prase command --
    let mut args = env::args();
    args.next();
    let path: String = args.next().ok_or(MyErrorTs::UsageError)?;

    let dim = args.next().ok_or(MyErrorTs::UsageError)?
                  .parse::<u8>().map_err(MyErrorTs::PIE)?;
    let resolution = args.next().ok_or(MyErrorTs::UsageError)?;

    // -- creating and initialising output file --
    let mut out = create_outfile(&path, dim, &resolution)?;

    // -- useful values --
    let res = resolution.parse::<u32>().map_err(MyErrorTs::PIE)?;
    let nbpixel = (res as u128).pow((dim - 1) as u32);

    // camera is basicly an object somewhere oriented to a dir
    // so assimilable to a Ray
    let mut camera = Ray {
        pos: VectorN::new(vec![0.; dim as usize]),
        dir: VectorN::new(vec![0.; dim as usize])
    };
    // looking forward in the last coord, 0 otherwise
    camera.dir.coords[dim as usize - 1] = 1.;

    // world
    let mut world = World {
        objs: vec![],
        cam: camera,
        dim: dim,
        res: res,
        viewport_size: 2.,
        focal_len: 1.,
    };

    // example sphere(s)
    {
        let mut tmp = vec![0.; world.dim as usize];
        tmp[world.dim as usize - 1] = -1.;
        world.objs.push(Sphere::new(VectorN::new(tmp), 0.5));

        tmp = vec![0.; world.dim as usize];
        tmp[world.dim as usize - 1] = -1.;
        tmp[world.dim as usize - 2] = -100.5;
        world.objs.push(Sphere::new(VectorN::new(tmp), 100.))
    }

    // couldn't hardcode axis vectors
    let axis = |n: usize| -> VectorN {
        let mut r = VectorN::new(vec![0.; world.dim as usize]);
        r.coords[n] = world.viewport_size;
        r
    };

    // calculating starting point
    let mut tmp = VectorN::new(vec![0.; world.dim as usize]);
    tmp.coords[(world.dim as usize)-1] = world.focal_len;
    let mut firstcorner = &world.cam.pos - &tmp;
    for i in 0..(world.dim as usize)-1 {
        firstcorner -= &(axis(i)/2.);
    }

    // -- render --
    let mut loopvars = vec![0; world.dim as usize - 1];
    for i in 0..nbpixel {
        tmp = &firstcorner - &world.cam.pos;
        for j in 0..(world.dim as usize) - 1 {
            tmp += axis(j) * (1. - loopvars[j] as f64/world.res as f64);
        }
        let r = Ray::new(&world.cam.pos, &tmp);

        write_color(&mut out, ray_color(r, &world).get_color())?;

        if (i + 1)%(world.res as u128) == 0 {
            eprint!("\rLoading: {:02.1}%", (i as f32/nbpixel as f32) * 100.);
            stderr().flush().map_err(MyErrorTs::IO)?;
        }
        incr(&mut loopvars, world.res);
    }
    eprintln!();

    Ok(())
}
