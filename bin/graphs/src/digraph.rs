use anyhow::Result;
use std::{env, process};

use bodium_core::{
    algos::{DirectedBFS, DirectedDFS},
    data_containers::{Config, build_digraph},
};

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
    let source_v = config.source_vertex as usize;
    let graph = build_digraph(&config)?;

    let mut dfs = DirectedDFS::new(&graph, source_v);
    dfs.build(&graph);
    let mut bfs = DirectedBFS::new(&graph, source_v);
    bfs.build(&graph);

    print!("{}", graph);
    // for (id, list) in groups.iter().enumerate() {
    //     println!("Component {id}: {list:?}");
    // }
    println!();
    println!("Depth first search\n{}", dfs);
    println!("Breadth first search\n{}", bfs);

    Ok(())
}
