mod brute_force;
mod fast_collinear;
mod matplotlib;

use anyhow::{Context, Result};
use std::{env, process};

use crate::fast_collinear::{FastCollinear, LineSegment};
use bodium_core::data_types::{In, Point};
use matplotlib::render_geometry_plot;

#[show_image::main]
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(err) = run(config) {
        eprintln!("Application error: {err}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<()> {
    let ints = In::build(&config.file_path)?;
    let mut iter = ints.into_iter();
    let n = iter
        .next()
        .context("Missing total number of points at start of file")? as usize;
    let mut points: Vec<Point> = Vec::with_capacity(n);
    for i in 0..n {
        let x = iter
            .next()
            .with_context(|| format!("Missing X coordinate for point {}", i + 1))?;
        let y = iter
            .next()
            .with_context(|| format!("Missing Y coordinate for point {}", i + 1))?;
        points.push(Point::new(x, y));
    }

    let mut fast = FastCollinear::build(&points)?;

    fast.fast_collinear();
    println!(
        "number of line segments: {}",
        fast.number_of_line_segments()
    );
    let segments = fast.line_segments();
    for seg in &segments {
        println!("{seg}");
    }

    handle_visualization(&points, &segments)?;

    Ok(())
}

struct Config {
    file_path: String,
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let file_path = args.next().ok_or("Missing file path")?;

        Ok(Config { file_path })
    }
}

fn handle_visualization(points: &[Point], segments: &[LineSegment]) -> anyhow::Result<()> {
    // 1. Transform raw integer structures into flat float arrays instantly
    let raw_points: Vec<(f64, f64)> = points.iter().map(|p| p.to_f64()).collect();
    let raw_segments: Vec<((f64, f64), (f64, f64))> = segments.iter().map(|s| s.to_f64()).collect();

    // 2. Discover the absolute maximum upper boundary value (e.g., 20000.0)
    // We fall back to 1.0 if the graph is empty to prevent dividing by zero
    let max_x = raw_points.iter().map(|p| p.0).fold(0.0, f64::max);
    let max_y = raw_points.iter().map(|p| p.1).fold(0.0, f64::max);

    let scale_x = if max_x > 0.0 { max_x } else { 1.0 };
    let scale_y = if max_y > 0.0 { max_y } else { 1.0 };

    // 3. Scale everything down to fit perfectly inside a 0.0 to 1.0 unit square
    let normalized_points: Vec<(f64, f64)> = raw_points
        .iter()
        .map(|&(x, y)| (x / scale_x, y / scale_y))
        .collect();

    let normalized_segments: Vec<((f64, f64), (f64, f64))> = raw_segments
        .iter()
        .map(|&(start, end)| {
            (
                (start.0 / scale_x, start.1 / scale_y),
                (end.0 / scale_x, end.1 / scale_y),
            )
        })
        .collect();

    // 4. Pass the scaled datasets to your unit square plotter utility
    render_geometry_plot(
        "collinear_output.png",
        &normalized_points,
        &normalized_segments,
    )?;

    // 1. Load the dynamic image array buffer from disk
    let dynamic_image = image::open("collinear_output.png")?;

    // 2. Open your desktop rendering context window
    let window = show_image::create_window("Collinear Analysis Output Graph", Default::default())?;

    // 🚀 FIX: Borrow a clean, temporary image view matching the expected trait bound!
    window.set_image("graph_view", dynamic_image)?;

    // 🚀 CRITICAL FIX: Block the main thread here so the window stays open!
    window.wait_until_destroyed()?;

    Ok(())
}
