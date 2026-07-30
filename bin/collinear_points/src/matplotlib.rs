use crate::fast_collinear::LineSegment;
use anyhow::Result;
use bodium_core::data_types::Point;
use plotters::prelude::*;

type Floats = (f64, f64);

pub fn handle_visualization(points: &[Point], segments: &[LineSegment]) -> anyhow::Result<()> {
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
    // let dynamic_image = image::open("collinear_output.png")?;

    // // 2. Open your desktop rendering context window
    // let window = show_image::create_window("Collinear Analysis Output Graph", Default::default())?;

    // // 🚀 FIX: Borrow a clean, temporary image view matching the expected trait bound!
    // window.set_image("graph_view", dynamic_image)?;

    // // 🚀 CRITICAL FIX: Block the main thread here so the window stays open!
    // window.wait_until_destroyed()?;

    Ok(())
}

/// Renders a list of geometric coordinates and lines onto a static unit square PNG image
fn render_geometry_plot(
    output_path: &str,
    points: &[Floats],             // Your Point dataset casted to floats
    segments: &[(Floats, Floats)], // Your LineSegment bounds
) -> Result<()> {
    // 1. Initialize the drawing canvas (Resolution: 800 x 800 pixels)
    let root = BitMapBackend::new(output_path, (800, 800)).into_drawing_area();

    // 🚀 1. Dial down the white glare by 20% using a clean matte-grey RGB mix
    let soft_background = RGBColor(225, 225, 225); // Slightly muted from (255, 255, 255)
    root.fill(&soft_background.mix(0.9))?;

    // 2. Establish a Chart Context mapped strictly to a 0.0 -> 1.0 Unit Square
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Geometric Collinear Analysis",
            ("sans-serif", 30).into_font(),
        )
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0.0..1.0, 0.0..1.0)?; // 👈 Limits bounds to Unit Square

    // 3. Configure the Grid, Axes, and Labels
    chart
        .configure_mesh()
        .x_desc("X (Normalized)")
        .y_desc("Y (Normalized)")
        .axis_desc_style(("sans-serif", 15))
        // 🚀 CRITICAL UPDATE: Turn off the background gridlines!
        .disable_x_mesh() // Disables vertical gridlines
        .disable_y_mesh() // Disables horizontal gridlines
        .draw()?;

    // 4. Draw the Line Segments (Rendered in Black)
    chart
        .draw_series(segments.iter().map(|&(start, end)| {
            let line_style = BLACK.mix(0.8).stroke_width(2);
            PathElement::new(vec![start, end], line_style)
        }))?
        .label("Collinear Segments")
        .legend(|(x, y)| PathElement::new(vec![(x, y - 5), (x + 20, y - 5)], BLACK));

    // 5. Draw the Discrete Scatter Points (Rendered as Blue Circles)
    chart
        .draw_series(
            points
                .iter()
                .map(|&coord| Circle::new(coord, 5, BLUE.filled())),
        )?
        .label("Input Points")
        .legend(|(x, y)| Circle::new((x + 10, y), 4, BLUE.filled()));

    // 6. Render the Legend Box in the upper right quadrant
    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperRight) // 👈 Moves from center-right to top-right
        .background_style(soft_background.mix(0.8)) // Matches our new soft background
        .border_style(BLACK)
        .draw()?;

    // Flush mutations safely to disk
    root.present()?;
    println!("Graph successfully exported to {output_path}");
    Ok(())
}
