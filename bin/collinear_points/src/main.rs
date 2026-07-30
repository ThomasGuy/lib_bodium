mod fast_collinear;
mod matplotlib;

use anyhow::{Context, Result};
use std::{env, process};

use crate::fast_collinear::FastCollinear;
use bodium_core::data_types::{In, Point};
use matplotlib::handle_visualization;

// #[show_image::main]
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
