use anyhow::Result;
use plotters::prelude::*;

type Floats = (f64, f64);

/// Renders a list of geometric coordinates and lines onto a static unit square PNG image
pub fn render_geometry_plot(
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
