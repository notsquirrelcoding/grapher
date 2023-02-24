use std::path::Path;

use num_complex::{Complex, Complex64, ComplexFloat};
use point::Point;

mod grapher;
mod point;

const PHI: f64 = 1.61803399;

const PRECISION: f64 = 100.0;

const I: Complex64 = Complex::new(0.0, 1.0);

fn main() -> anyhow::Result<()> {
    let file = Path::new("a.png");
    let mut grapher = grapher::Grapher::new(file);

    // grapher.run(f)?;

    grapher.draw_line(Point::new(0.0, 0.0), Point::new(-10.0, 10.0));
    grapher.set_pixel(0.0, 0.0);
    grapher.save()?;

    Ok(())
}

fn f(x: f64) -> Complex64 {
    (1.0 - I) * (PHI * I).powf(x)
}
