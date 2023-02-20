use crate::{point::Point, PRECISION};
use anyhow::Result;
use std::path::Path;

use console::Term;
use image::{ImageBuffer, Rgb, RgbImage};
use num_complex::Complex64;

const DIM: u32 = 50;
type ReFunc = fn(f64) -> f64;
type ReImFunc = fn(f64) -> Complex64;

pub struct Grapher {
    center: Point,
    buf: ScreenBuf,
    zoom_factor: f64,
    axis_enabled: bool,
}

impl Grapher {
    pub fn new(_file: &Path) -> Self {
        Self {
            center: Point::new(0.0, 0.0),
            buf: ScreenBuf::new(),
            zoom_factor: 1.0,
            axis_enabled: false,
        }
    }

    /// Updates the plot
    pub fn update_plot(&mut self, f: ReImFunc) -> anyhow::Result<()> {
        self.buf.buf.fill(255);
        self.draw_re_z_func(f);

        if self.axis_enabled {
            self.draw_axes();
        }

        self.save()?;
        Ok(())
    }

    pub fn draw_re_func(&mut self, f: ReFunc) {
        let a = self.center.x - DIM as f64;
        let b = self.center.x + DIM as f64;

        let sample_points: Vec<f64> = (0..PRECISION as i32)
            .map(|x| a + x as f64 * (b - a) / PRECISION)
            .collect();

        for x in sample_points {
            self.set_pixel(x, f(x));
        }
    }

    pub fn draw_re_z_func(&mut self, f: ReImFunc) {
        let start = self.center.x - DIM as f64 / self.zoom_factor;
        let end = self.center.x + DIM as f64 / self.zoom_factor;

        let dx = (end - start) / PRECISION;

        let sample_points: Vec<f64> = (0..(PRECISION as i32))
            .map(|i| start + i as f64 * dx)
            .collect();



        let first = f(sample_points.first().unwrap().clone());
        let mut prev = Point::new(first.re, first.im);

        for r in sample_points {
            
            let z = f(r);
            let curr = Point::new(z.re, z.im);
            // self.set_pixel(z.re, z.im);

            self.draw_line(prev, Point::new(z.re, z.im));

            prev = curr;
        }
    }

    fn set_pixel(&mut self, x: f64, y: f64) {
        let point = self.map_point(x, y);

        if point.0 < 2 * DIM && point.1 < 2 * DIM {
            self.buf.buf.put_pixel(point.0, point.1, Rgb([0, 0, 0]));
        }
    }

    pub fn draw_line(&mut self, a: Point, b: Point) {
        let mut current_point = a.clone();

        let dx = a.distance_x(&b) / PRECISION;
        let dy = a.distance_y(&b) / PRECISION;

        let black_pixel = Rgb([0, 0, 0]);

        for _ in 0..(PRECISION as i32) {
            self.set_pixel(current_point.x, current_point.y);

            current_point.x += dx;
            current_point.y += dy;
        }
    }

    /// Maps a point from the coordinate system where `(0, 0)` is the center to the system where `(0, 0)` is the top-left corner
    /// of the screen
    fn map_point(&self, x: f64, y: f64) -> (u32, u32) {
        // Unzoom
        let nx = ((x - self.center.x) * self.zoom_factor).round() + DIM as f64;
        let ny = DIM as f64 - ((y - self.center.y) * self.zoom_factor).round();

        // println!("({x}, {y}) -> ({nx} {ny})");

        (nx.round() as u32, (ny -1.0).round() as u32)
    }

    pub fn draw_axes(&mut self) {
        let black_pixel = Rgb([0, 0, 0]);

        let tick_space = 5;

        for y in 0..(2 * DIM) {
            if y % tick_space == 0 {
                self.buf.buf.put_pixel((DIM) + 1, y, black_pixel);
                self.buf.buf.put_pixel((DIM) - 1, y, black_pixel);
            }

            self.buf.buf.put_pixel(DIM, y, black_pixel);
        }

        for x in 0..(2 * DIM) {
            if x % tick_space == 0 {
                self.buf.buf.put_pixel(x, (DIM) + 1, black_pixel);
                self.buf.buf.put_pixel(x, (DIM) - 1, black_pixel);
            }

            self.buf.buf.put_pixel(x, DIM, black_pixel);
        }
    }

    pub fn run(&mut self, f: ReImFunc) -> anyhow::Result<()> {
        let stdout = Term::buffered_stdout();

        loop {
            if let Ok(character) = stdout.read_char() {
                match character {
                    'z' => {
                        self.zoom_factor *= 2.0;
                    }
                    'x' => {
                        self.zoom_factor /= 2.0;
                    }
                    'w' => self.center.y += 10.0 / self.zoom_factor,
                    'a' => self.center.x -= 10.0 / self.zoom_factor,
                    's' => self.center.y -= 10.0 / self.zoom_factor,
                    'd' => self.center.x += 10.0 / self.zoom_factor,
                    'e' => self.axis_enabled = !self.axis_enabled,
                    'r' => {
                        self.zoom_factor = 1.0;
                        self.center.x = 0.0;
                        self.center.y = 0.0;
                    }
                    'k' => break,
                    _ => {}
                }
                self.update_plot(f)?;

                println!(
                    "ZOOM: {}\tCENTER (z): ({}, {})\t \tAXIS ENABLED: {}",
                    self.zoom_factor, self.center.x, self.center.y, self.axis_enabled
                );
            }
        }

        Ok(())
    }

    pub fn save(&self) -> anyhow::Result<()> {
        self.buf.write(Path::new("a.png"))
    }
}

/// A struct representing the screen buffer
#[derive(Debug)]
struct ScreenBuf {
    buf: ImageBuffer<Rgb<u8>, Vec<u8>>,
    // file: Path,
}

impl ScreenBuf {
    fn new() -> Self {
        let mut buf = RgbImage::new(2 * DIM, 2 * DIM);

        buf.fill(255);

        Self { buf }
    }


    fn write(&self, path: &Path) -> Result<()> {
        self.buf
            .save_with_format(path, image::ImageFormat::Png)
            .map_err(|err| err.into())
    }
}
