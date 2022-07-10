mod vector;
mod ray;
mod shapes;
mod out;

use std::env;
use std::io::{stderr, Write};
use crate::out::*;
use crate::vector::*;
use crate::ray::*;

fn incr(v: &mut Vec<u32>, size: u32) {
    let mut tmp = 1;
    for nb in v.iter_mut() {
        *nb += tmp;
        tmp = *nb/size;
        *nb %= size;
    }
}

fn main() {
    // -- Bad command --
    if env::args().count() != 4 {
        println!("Usage: ./yarrt OutFile dimensions resolution");
        return;
    }

    // -- prase command --
    let mut args = env::args();
    args.next();
    let path: String = args.next().unwrap();

    let dim = args.next().unwrap().parse::<u8>().unwrap();
    let resolution = args.next().unwrap();

    // -- creating and initialising output file --
    let mut out = create_outfile(&path, dim, &resolution);

    // -- useful values --
    let res = resolution.parse::<u32>().unwrap();
    let nbpixel = (res as u128).pow((dim - 1) as u32);

    let viewport_size = 2.0;
    let focal_len = 1.0;
    // camera is basicly an object somewhere oriented to a dir
    // so assimilable to a Ray
    let mut camera = Ray {
        pos: VectorN::new(vec![0.; dim as usize]),
        dir: VectorN::new(vec![0.; dim as usize])
    };
    // looking forward in the last coord, 0 otherwise
    camera.dir.coords[dim as usize - 1] = 1.;

    // couldn't hardcode axis vectors
    let axis = |n: usize| -> VectorN {
        let mut r = VectorN::new(vec![0.; dim as usize]);
        r.coords[n] = viewport_size;
        r
    };

    // calculating starting point
    let mut tmp = VectorN::new(vec![0.; dim as usize]);
    tmp.coords[(dim as usize)-1] = focal_len;
    let mut firstcorner = &camera.pos - &tmp;
    for i in 0..(dim as usize)-1 {
        firstcorner -= &(axis(i)/2.);
    }

    // -- render --
    let mut loopvars = vec![0; dim as usize - 1];
    for i in 0..nbpixel {
        tmp = &firstcorner - &camera.pos;
        for j in 0..(dim as usize) - 1 {
            tmp += axis(j) * (1. - loopvars[j] as f64/res as f64);
        }
        let r = Ray::new(&camera.pos, &tmp);

        out = write_color(out, ray_color(r).get_color());

        if (i + 1)%(res as u128) == 0 {
            eprint!("\rLoading: {:02.1}%", (i as f32/nbpixel as f32) * 100.);
            stderr().flush().unwrap();
        }
        incr(&mut loopvars, res);
    }
    eprintln!();
}
