use crate::fast_collinear::LineSegment;
use bodium_core::data_types::Point;

use anyhow::{Result, bail};

#[derive(Debug)]
pub struct BruteCollinear {
    segments: Vec<LineSegment>,
}

impl BruteCollinear {
    pub fn build(mut points: Vec<Point>) -> Result<Self> {
        if points.len() < 4 {
            bail!("Minimum Four Points Needed (found {})", points.len());
        }

        // Check for duplicates
        let mut check_pts = points.clone();
        check_pts.sort_unstable();
        if check_pts.windows(2).any(|w| w[0] == w[1]) {
            bail!("No Duplicate Points Allowed");
        }

        // Brute force relies on points being sorted by position
        // to easily identify the segment endpoints (min/max)
        points.sort_unstable();

        let mut segments = Vec::new();
        let n = points.len();

        // 4-nested loops to check all combinations of 4 points
        for i in 0..n {
            for j in (i + 1)..n {
                for k in (j + 1)..n {
                    for m in (k + 1)..n {
                        let p = points[i];
                        let q = points[j];
                        let r = points[k];
                        let s = points[m];

                        // Calculate slopes relative to point p
                        let slope_pq = p.slope_to(&q);
                        let slope_pr = p.slope_to(&r);
                        let slope_ps = p.slope_to(&s);

                        // If all 4 points are collinear
                        if slope_pq == slope_pr && slope_pq == slope_ps {
                            // Because points is sorted, p is min and s is max
                            segments.push(LineSegment::new(p, s));
                        }
                    }
                }
            }
        }

        Ok(Self { segments })
    }

    pub fn number_of_line_segments(&self) -> u32 {
        self.segments.len() as u32
    }

    pub fn line_segments(self) -> Vec<LineSegment> {
        self.segments
    }
}
