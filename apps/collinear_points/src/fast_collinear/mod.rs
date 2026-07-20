use anyhow::{Result, bail};
use compare::Compare;
use std::cmp::Ordering;

use bodium_core::data_types::Point;
pub mod line_segment;
use line_segment::LineSegment;

#[derive(Debug)]
pub struct FastCollinear {
    points: Vec<Point>,
    num: usize,
    segment_count: u32,
    segments: Vec<LineSegment>,
}

impl FastCollinear {
    pub fn build(points: &Vec<Point>) -> Result<Self> {
        if points.len() < 4 {
            bail!("Minimum Four Points Needed (found {})", points.len());
        }

        // 1. Create a quick, lightweight clone of the references/values just for checking
        let mut check_pts = points.clone();
        check_pts.sort_unstable();

        // 2. Use windows(2) to look at pairs side-by-side.
        // If ANY pair matches, we found a duplicate!
        let has_duplicates = check_pts.windows(2).any(|w| w[0] == w[1]);

        if has_duplicates {
            bail!("No Duplicate Points Allowed");
        }

        Ok(Self {
            num: points.len(),
            points: points.to_vec(),
            segment_count: 0,
            segments: Vec::new(),
        })
    }
}

impl FastCollinear {
    // find the collinear points that have the origin ordered as lowest
    fn check_for_line(&mut self, origin: Point, low_idx: usize, high_idx: usize, slope: &[Point]) {
        // Elements in slope[low_idx..high_idx] are already sorted by slope.
        // We only need to find the absolute minimum and maximum to check the segment.
        let segment_slice = &slope[low_idx..high_idx];

        // Find min and max using standard iterator tools (no allocations!)
        #[allow(clippy::collapsible_if)]
        if let (Some(min_p), Some(max_p)) = (segment_slice.iter().min(), segment_slice.iter().max())
        {
            if origin < *min_p {
                self.segments.push(LineSegment::new(origin, *max_p));
                self.segment_count += 1;
            }
        }
    }

    pub fn fast_collinear(&mut self) {
        let mut slope = self.points.clone();
        // for origin in self.points.clone() {
        for i in 0..self.num {
            let origin = self.points[i];
            let slope_ordered = origin.slope_order();
            slope.sort_by(|l, r| slope_ordered.compare(l, r));
            let mut low = 1;
            for j in 1..(self.num - 1) {
                match slope_ordered.compare(&slope[j], &slope[j + 1]) {
                    Ordering::Equal => {
                        if j == self.num - 2 && j + 1 - low >= 2 {
                            // self.check_for_line(origin, &slope[low..]);
                            self.check_for_line(origin, low, self.num, &slope);
                        }
                    }
                    _ => {
                        if j - low >= 2 {
                            // self.check_for_line(origin, &slope[low..(j + 1)]);
                            self.check_for_line(origin, low, j + 1, &slope);
                        }
                        low = j + 1;
                    }
                }
            }
        }
    }

    pub fn number_of_line_segments(&self) -> u32 {
        self.segment_count
    }

    pub fn line_segments(self) -> Vec<LineSegment> {
        self.segments
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test 1: Ensure it builds correctly with valid, unique points
    #[test]
    fn test_build_success() {
        let unique_points = vec![
            Point::new(0, 0),
            Point::new(1, 1),
            Point::new(2, 2),
            Point::new(3, 3),
        ];

        let result = FastCollinear::build(&unique_points);
        assert!(
            result.is_ok(),
            "Expected successful build with unique points"
        );

        let cv = result.unwrap();
        assert_eq!(cv.num, 4);
    }

    // Test 2: Ensure it errors out if there are fewer than 4 points
    #[test]
    fn test_insufficient_points() {
        let too_few = vec![Point::new(0, 0), Point::new(1, 1)];
        let result = FastCollinear::build(&too_few);

        assert!(result.is_err());
        let err_message = result.unwrap_err().to_string();
        assert!(
            err_message.contains("Minimum Four Points Needed"),
            "Unexpected error message: {err_message}"
        );
    }

    // Test 3: Ensure it catches duplicates and returns the precise error message
    #[test]
    fn test_duplicate_points_trigger_error() {
        let duplicate_points = vec![
            Point::new(1, 1),
            Point::new(2, 2),
            Point::new(1, 1), // Duplicate point!
            Point::new(3, 3),
        ];

        let result = FastCollinear::build(&duplicate_points);

        // Assert that an error actually happened
        assert!(
            result.is_err(),
            "Expected an error due to duplicate points, but got Ok"
        );

        // Extract the anyhow error message as a string
        let error_message = result.unwrap_err().to_string();

        // Match against your exact string expectation
        assert_eq!(error_message, "No Duplicate Points Allowed");
    }
}
