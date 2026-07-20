use std::fmt;

use bodium_core::data_types::Point;

#[derive(Debug, Clone)]
pub struct LineSegment {
    p: Point,
    q: Point,
}

impl fmt::Display for LineSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.p, self.q)
    }
}

impl LineSegment {
    pub fn new(p: Point, q: Point) -> Self {
        Self { p, q }
    }

    pub fn to_f64(&self) -> ((f64, f64), (f64, f64)) {
        (self.p.to_f64(), self.q.to_f64())
    }
}
