use std::path::Path;

mod point;
mod grapher;

const DIMENSIONS: u32 = 100;
const PRECISION: usize = 1000;


fn main() -> anyhow::Result<()> {
    let file = Path::new("a.png");
    let mut grapher = grapher::Grapher::new(file);

    grapher.draw_func(f);

    grapher.save()
}

fn f(x: f64) -> f64 {
    x.powi(2)
}