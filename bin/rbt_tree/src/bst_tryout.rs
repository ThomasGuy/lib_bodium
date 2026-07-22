use bodium_core::data_containers::BinarySearchTree;
use bodium_core::data_types::{Point, Seeder, visualize_bst};
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

            let mut bst = BinarySearchTree::new();
            for (key, val) in data {
                bst.put(key, val);
            }

            println!(
                "Text search tree constructed! Total elements: {}",
                bst.size()
            );

            for (key, value) in &bst {
                println!("ID: {:<5} -> Coordinates: {}", key, value);
            }

            visualize_bst::draw_tree_graph("tree_view.png", &bst)?;
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

            let mut bst = BinarySearchTree::new();
            println!("data key order");
            for (key, val) in data {
                print!("{}, ", key);
                bst.put(key, val);
            }

            println!(
                "\nGeometric search tree constructed! Total elements: {}",
                bst.size()
            );

            visualize_bst::draw_tree_graph("tree_view.png", &bst)?;
        }
    }

    Ok(())
}
