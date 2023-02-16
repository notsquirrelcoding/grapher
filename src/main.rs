use std::path::Path;

use num_complex::{Complex, Complex64};

mod grapher;
mod point;

const PRECISION: usize = 1000;

const I: Complex64 = Complex::new(0.0, 1.0);

fn main() -> anyhow::Result<()> {
    let file = Path::new("a.png");
    let mut grapher = grapher::Grapher::new(file);

    grapher.draw_re_z_func(f);

    grapher.run(f)
}

fn f(x: f64) -> Complex64 {
    x.cos() + I * x.sin()
}
