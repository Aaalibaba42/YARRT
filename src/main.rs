mod vector;
mod out;

use std::env;
use std::io::{stderr, Write};
use crate::out::*;

fn main() {
    if env::args().count() != 4 {
        println!("Usage: ./yaart OutFile dimensions resolution");
        return;
    }
    let mut args = env::args();
    args.next();
    let path: String = args.next().unwrap();

    let dim = args.next().unwrap().parse::<u8>().unwrap();
    let resolution = args.next().unwrap();

    let mut out = create_outfile(&path, dim, &resolution);

    let res = resolution.parse::<u32>().unwrap();

    let nbpixel = res.pow((dim - 1) as u32);

    for i in 0..nbpixel {
        let r = ((i%res) as f64) / ((res - 1) as f64);
        let g = ((i/res) as f64) / ((res - 1) as f64);
        let b = (i as f64 / nbpixel as f64) - r + g;
        //let b = 0.25;

        out = write_color(out, (
            ((255.999 * r) as u8),
            ((255.999 * g) as u8),
            ((255.999 * b) as u8)
        ));

        if (i + 1) % res == 0 {
            eprint!("\rLoading: {:02.0}%", (i as f32/nbpixel as f32) * 100.);
            stderr().flush().unwrap();
        }
    }
    eprintln!();
}
