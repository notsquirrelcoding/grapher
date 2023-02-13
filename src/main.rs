const DIMENSIONS: usize = 100;
const PRECISION: usize = 1000;
fn main() {
    let mut buf = ScreenBuf::new();

    let a = Point::new(0.0, 0.0);
    let b = Point::new(3.14, 2.72);

    buf.draw_horizontal_line(a, b);

    println!("{:?}", buf.buf)
}

/// A struct representing the screen buffer
struct ScreenBuf {
    pub buf: [usize; (DIMENSIONS * DIMENSIONS)],
}

impl ScreenBuf {
    fn new() -> Self {
        Self {
            buf: [0; (DIMENSIONS * DIMENSIONS)],
        }
    }

    fn draw_horizontal_line(&mut self, a: Point, b: Point) {
        let mut current_point = a.clone();
        let distance = a.distance(&b);

        let dx = a.distance_x(&b) / PRECISION as f64;
        let dy = a.distance_y(&b) / PRECISION as f64;

        for _ in 0..PRECISION {

            self.set_pixel(current_point.x, current_point.y);
        
           current_point.x += dx;             
           current_point.y += dy;             
        }


    }

    fn set_pixel(&mut self, x: f64, y: f64) {
        let point = self.map_point(x.round() as i32, y.round() as i32);

        // Convert the point into an index
        let index = point.0 + DIMENSIONS * point.1;

        self.buf[index] = 1;
    }

    /// Maps a point from the coordinate system where `(0, 0)` is the center to the system where `(0, 0)` is the top-left corner
    /// of the screen
    fn map_point(&self, x: i32, y: i32) -> (usize, usize) {
        // Shifts the x coordinate 50 pixels to the left and flips the y coordinate around and shifts it up by 50 pixels as well
        (
            (x + (DIMENSIONS / 2) as i32) as usize,
            (-y + (DIMENSIONS / 2) as i32) as usize,
        )
    }
}

#[derive(Debug, Clone)]
struct Point {
    pub x: f64,
    pub y: f64,
}
impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn distance_x(&self, other: &Self) -> f64 {
        self.x.abs() - other.x.abs()
    }

    fn distance_y(&self, other: &Self) -> f64 {
        self.y.abs() - other.y.abs()
    }

    /// Calculates the distance between two points using the good old Pythagorean Theorem
    fn distance(&self, other: &Self) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}


