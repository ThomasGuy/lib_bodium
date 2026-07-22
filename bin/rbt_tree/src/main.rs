use bodium_core::data_containers::RedBlackTree;
use bodium_core::data_types::{Point, Seeder, visualize_rbt};
use std::{env, process};
pub mod config;
use crate::config::{Config, DataFormat};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(err) = run_pipeline(config) {
        eprintln!("Application error: {err}");
        process::exit(1);
    }
}

pub fn run_pipeline(config: Config) -> anyhow::Result<()> {
    match config.format {
        // --- BRANCH A: TEXT FORMAT PROCESSING TRACK ---
        DataFormat::TextPairs => {
            println!(
                "Initializing Text Dictionary Engine for: {:?}",
                config.file_path
            );

            // Explicitly tell the compiler to build a u32 -> String seeder
            let mut data: Seeder<u32, String> = Seeder::from_file(&config.file_path)?;
            data.shuffle(); // Prevent tree degradation bottlenecks

            let mut rbt = RedBlackTree::new();
            for (key, val) in data {
                rbt.insert(key, val);
            }

            println!(
                "Text search tree constructed! Total elements: {}",
                rbt.size()
            );

            for (key, value) in &rbt {
                println!("ID: {:<5} -> Coordinates: {}", key, value);
            }

            visualize_rbt::draw_tree("brt_tree.png", &rbt, None)?;
        }

        // --- BRANCH B: GEOMETRIC FORMAT PROCESSING TRACK ---
        DataFormat::Geometry => {
            println!(
                "Initializing Geometric Grid Engine for: {:?}",
                config.file_path
            );

            // Explicitly tell the compiler to build a u32 -> Point seeder
            let mut data: Seeder<u32, Point> = Seeder::from_file(&config.file_path)?;
            data.shuffle();

            let mut rbt = RedBlackTree::new();
            println!("data key order");
            for (key, val) in data.clone() {
                print!("{}, ", key);
                rbt.insert(key, val);
            }

            println!(
                "\nGeometric search tree constructed! Total elements: {}",
                rbt.size()
            );

            visualize_rbt::draw_tree("brt_tree.png", &rbt, None)?;
        }
    }

    Ok(())
}

#[test]
fn stress_test_red_black_tree() {
    use rand::seq::SliceRandom; // Assumes rand crate is in dependencies

    let mut rng = rand::rng();
    let original_dataset: Vec<i32> = (1..=100).collect();
    let mut failure_triggered = false;

    println!("🚀 Starting 10,000 iteration stress test...");

    for iteration in 1..=10000 {
        // 1. Generate a completely fresh shuffle every iteration
        let mut shuffled_dataset = original_dataset.clone();
        shuffled_dataset.shuffle(&mut rng);

        // 2. Build the tree
        let mut rbt = RedBlackTree::new();
        for &key in &shuffled_dataset {
            rbt.insert(key, format!("val_{}", key));
        }

        // 3. Verify absolute black balance stability
        if rbt.verify_black_balance().is_none() {
            println!("\n💥 Invariant Broken on iteration {}!", iteration);
            println!("Exact dataset sequence that caused the split:");
            println!("{:?}", shuffled_dataset);

            // 4. Save the broken state layout snapshot
            if let Err(e) = visualize_rbt::draw_tree("broken_tree_view.png", &rbt, None) {
                println!("Failed to output visual graph: {:?}", e);
            } else {
                println!("📸 Saved structural snapshot to 'broken_tree_view.png'");
            }

            failure_triggered = true;
            break;
        }

        // Optional: print progress every 1,000 iterations to watch it work
        if iteration % 1000 == 0 {
            println!("✓ Passed {} iterations...", iteration);
        }
    }

    // Explicitly fail the cargo test run if the invariant tripped
    assert!(
        !failure_triggered,
        "Tree lost balance during the stress test loop!"
    );
    println!("🎉 Success! Tree remained perfectly balanced across all 10,000 random shuffles.");
}
