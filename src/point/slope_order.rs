use crate::point::Point;
use compare::Compare;
use std::cmp::Ordering;

/*
* Compares two points by the slope they make with self.origin.
* The slope is defined as the Point.slope_to() method.
*/
pub struct SlopeOrder<'a> {
    origin: &'a Point,
}

impl<'a> SlopeOrder<'a> {
    pub fn new(origin: &'a Point) -> Self {
        Self { origin }
    }
}

/**
 * the Comparator that defines this ordering on points
 */
impl<'a> Compare<Point> for SlopeOrder<'a> {
    fn compare(&self, p: &Point, q: &Point) -> Ordering {
        if self.origin.slope_to(p) < self.origin.slope_to(q) {
            return Ordering::Less;
        }
        if self.origin.slope_to(p) > self.origin.slope_to(q) {
            return Ordering::Greater;
        }
        Ordering::Equal
    }
}
