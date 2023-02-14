use std::path::Path;

use num_complex::{Complex64, Complex};

mod point;
mod grapher;

const DIMENSIONS: u32 = 100;
const PRECISION: usize = 1000;

const I: Complex64 = Complex::new(0.0, 1.0);

fn main() -> anyhow::Result<()> {
    let file = Path::new("a.png");
    let mut grapher = grapher::Grapher::new(file);

    grapher.draw_re_z_func(f);

    grapher.save()
}

fn f(x: f64) -> Complex64 {
    x.sin()* I
}