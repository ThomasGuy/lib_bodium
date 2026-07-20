use bodium_core::data_types::Point;
use collinear_points::fast_collinear::FastCollinear;

fn main() {
    let points: Vec<Point> = vec![
        Point::new(0, -10),
        Point::new(20, -10),
        Point::new(20, 40),
        Point::new(20, 30),
        Point::new(10, 0),
        Point::new(0, 0),
        Point::new(0, 30),
        Point::new(40, 30),
        Point::new(30, 20),
        Point::new(20, 10),
        Point::new(20, 20),
        // Point::new(20, 20),
    ];

    match FastCollinear::build(&points) {
        Ok(mut fast) => {
            fast.fast_collinear();
            println!(
                "number of line segments: {}",
                fast.number_of_line_segments()
            );
            for seg in fast.line_segments() {
                println!("{seg}");
            }
        }
        Err(error) => {
            eprintln!("Error: {:?}", error)
        }
    };
}
