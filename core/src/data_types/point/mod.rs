use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

pub mod slope_order;

use slope_order::SlopeOrder;
use std::{cmp::Ordering, fmt::Display};

/// Custom error type to handle failed text-to-point transformations
#[derive(Error, Debug)]
pub enum PointParseError {
    #[error("Missing coordinate separator: Expected format 'X,Y'")]
    MissingSeparator,
    #[error("Failed to parse integer coordinate: {0}")]
    InvalidCoordinate(#[from] ParseIntError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    x: i32,
    y: i32,
}

/**
 * Initialises a new point.
 *
 * @param x the x-coordinate of the point
 * @param y the y-coordinate of the point
 */
impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// implement [FromStr] enables serialization like serde
impl FromStr for Point {
    type Err = PointParseError;

    /// Automatically converts a text coordinate token like "5,12" into a Point struct
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x_str, y_str) = s.split_once(',').ok_or(PointParseError::MissingSeparator)?;

        let x = x_str.trim().parse::<i32>()?;
        let y = y_str.trim().parse::<i32>()?;

        Ok(Point::new(x, y))
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/**
 * Total ordering for type Point, breaking ties by x-coordinate.
 * Formally, the invoking point (x0, y0) is less than the argument point
 * (x1, y1) iff either y0 < y1 or if y0 = y1 and x0 < x1.
 *
 * @param other point
 * @return the total order betweeen this and the other point
*/
impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        // Tuples compare elements left-to-right natively
        (self.y, self.x).cmp(&(other.y, other.x))
    }
}

impl Point {
    pub fn to_f64(&self) -> (f64, f64) {
        (self.x as f64, self.y as f64)
    }

    /**
     * Returns the slope between this point and the argument point.
     * Formally, if the two points are (x0, y0) and (x1, y1), then the slope
     * is (y1 - y0) / (x1 - x0). For completeness, the slope is defined to be
     * 0.0 if the line segment connecting the two points is horizontal;
     * INFINITY if the line segment is vertical;
     * and NEG_INFINITY if (x0, y0) and (x1, y1) are equal.
     *
     * @param that the other point
     * @return the slope between this point and the argument point
     */
    pub fn slope_to(&self, other: &Point) -> f64 {
        if other.x == self.x && other.y == self.y {
            f64::NEG_INFINITY
        } else if other.x == self.x {
            f64::INFINITY
        } else if other.y == self.y {
            0.0
        } else {
            (other.y - self.y) as f64 / (other.x - self.x) as f64
        }
    }

    /**
     * Compares two points by the slope they make with this point.
     * The slope is defined as in the slope_to() method.
     *
     * @return the Comparator that defines this ordering on points
     */
    pub fn slope_order(&'_ self) -> SlopeOrder<'_> {
        slope_order::SlopeOrder::new(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn total_order() {
        let this = Point::new(2, 2);
        let test1 = Point::new(2, 1);
        let test2 = Point::new(1, 2);
        let test3 = Point::new(2, 2);
        assert_eq!(this > test1, true);
        assert_eq!(test1 < this, true);
        assert_eq!(this > test2, true);
        assert_eq!(test2 < this, true);
        assert_eq!(this == test3, true);
        assert_eq!(this, test3);
        assert_ne!(this, test1);
        assert_ne!(test2, this);
    }

    #[test]
    fn sorting() {
        let y1 = Point::new(0, 5);
        let y2 = Point::new(0, 3);
        let x1 = Point::new(-1, 3);
        let x2 = Point::new(1, 5);
        let x3 = Point::new(1, 3);
        let sorted = [&x1, &y2, &x3, &y1, &x2];
        let mut test = [&x2, &y1, &x1, &y2, &x3];
        test.sort();
        assert_eq!(sorted[0], test[0]);
        assert_eq!(sorted[1], test[1]);
        assert_eq!(sorted[2], test[2]);
        assert_eq!(sorted[3], test[3]);
        assert_eq!(sorted[4], test[4]);
    }
}
