use crate::{point::Point, PRECISION};
use std::path::Path;
use anyhow::Result;

use console::Term;
use image::{ImageBuffer, Rgb, RgbImage};
use num_complex::{Complex64, Complex};

const DIM: u32 = 20;
type ReFunc = fn(f64) -> f64;
type ReImFunc = fn(f64) -> Complex64;

pub struct Grapher {
    center: Point,
    buf: ScreenBuf,
    zoom_factor: f64,
    axis_enabled: bool,
}

impl Grapher {
    pub fn new(file: &Path) -> Self {
        Self {
            center: Point::new(0.0, 0.0),
            buf: ScreenBuf::new(100, file),
            zoom_factor: 1.0,
            axis_enabled: false,
        }
    }

        /// Updates the plot
        pub fn update_plot(&mut self, f: ReImFunc) -> anyhow::Result<()> {
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

        let sample_points: Vec<f64> = (0..(PRECISION as f64).round() as i32).map(|x| {
            a + x as f64 * (b - a) / PRECISION as f64
        }).collect();

        for x in sample_points {
            self.buf.set_pixel(x, f(x));
        }
    }

    pub fn draw_re_z_func(&mut self, f: ReImFunc) {

        let a = self.center.x - DIM as f64;
        let b = self.center.x + DIM as f64;

        let sample_points: Vec<f64> = (0..(PRECISION as f64).round() as i32).map(|x| {
            a + x as f64 * (b - a) / PRECISION as f64
        }).collect();

        for r in sample_points {

            let z = f(r * self.zoom_factor);

            self.buf.set_pixel(z.re / self.zoom_factor, z.im / self.zoom_factor);
        }
    }

    pub fn draw_axes(&mut self) {
        let black_pixel = Rgb([0, 0, 0]);

        let tick_space = 5;

        for y in 0..DIM {
            if y % tick_space == 0 {
                self.buf.buf.put_pixel((DIM / 2) + 1, y, black_pixel);
                self.buf.buf.put_pixel((DIM / 2) - 1, y, black_pixel);
            }

            self.buf.buf.put_pixel(DIM / 2, y, black_pixel);
        }

        for x in 0..DIM {
            if x % tick_space == 0 {
                self.buf.buf.put_pixel(x, (DIM / 2) + 1, black_pixel);
                self.buf.buf.put_pixel(x, (DIM / 2) - 1, black_pixel);
            }

            self.buf.buf.put_pixel(x, DIM / 2, black_pixel);
        }
    }

    pub fn run(&mut self, f: ReImFunc) -> anyhow::Result<()> {
        let stdout = Term::buffered_stdout();

        loop {
            if let Ok(character) = stdout.read_char() {
                match character {
                    'z' => {
                        self.zoom_factor *= 2.0;
                        self.center.x *= 2.0;
                        self.center.y *= 2.0;
                        println!("{}", self.zoom_factor);
                    }
                    'x' => {
                        self.zoom_factor /= 2.0;
                        self.center.x /= 2.0;
                        self.center.y /= 2.0;
                    }
                    'w' => self.center.y += 10.0,
                    'a' => self.center.x -= 10.0,
                    's' => self.center.y -= 10.0,
                    'd' => self.center.x += 10.0,
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

                print!("{}[2J", 27 as char);

                println!(
                    "ZOOM: {}\tCENTER (z): ({}, {})\t \tAXIS ENABLED: {}",
                    self.zoom_factor,
                    self.center.x,
                    self.center.y,
                    self.axis_enabled
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
struct ScreenBuf {
    buf: ImageBuffer<Rgb<u8>, Vec<u8>>,
    precision: usize,
    // file: Path,
}

impl ScreenBuf {
    fn new(precision: usize, file: &Path) -> Self {

        let mut buf = RgbImage::new(DIM, DIM);

        buf.fill(255);

        Self {
            buf,
            precision
            // file: *file.clone()
        }
    }

    fn draw_horizontal_line(&mut self, a: Point, b: Point) {
        let mut current_point = a.clone();

        let dx = a.distance_x(&b) / self.precision as f64;
        let dy = a.distance_y(&b) / self.precision as f64;

        for _ in 0..self.precision {
            self.set_pixel(current_point.x, current_point.y);

            current_point.x += dx;
            current_point.y += dy;
        }
    }

    fn set_pixel(&mut self, x: f64, y: f64) {
        let point = self.map_point(x.round() as i32, y.round() as i32);

        if point.0 < DIM && point.1 < DIM {
            self.buf.put_pixel(point.0, point.1, Rgb([0, 0, 0]));
        }
    }

    /// Maps a point from the coordinate system where `(0, 0)` is the center to the system where `(0, 0)` is the top-left corner
    /// of the screen
    fn map_point(&self, x: i32, y: i32) -> (u32, u32) {
        // Shifts the x coordinate 50 pixels to the left and flips the y coordinate around and shifts it up by 50 pixels as well
        (
            (x + (DIM / 2) as i32) as u32,
            (-y + (DIM / 2) as i32) as u32,
        )
    }

    fn write(&self, path: &Path) -> Result<()> {
        let img_buf = RgbImage::from_raw(
            DIM,
            DIM,
            self.buf.clone().to_vec(),
        );

        img_buf.unwrap().save_with_format(path, image::ImageFormat::Png)?;

        Ok(())
    }
}