#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn distance_x(&self, other: &Self) -> f64 {
        self.x.abs() - other.x.abs()
    }

    pub fn distance_y(&self, other: &Self) -> f64 {
        self.y.abs() - other.y.abs()
    }

    /// Calculates the distance between two points using the good old Pythagorean Theorem
    pub fn distance(&self, other: &Self) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}
