use anyhow::Result;
use std::{env, process};

use bodium_core::{
    algos::{BreadthFirstPaths, DepthFirstPaths},
    data_containers::{Config, build_graph},
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
    let graph = build_graph(&config)?;

    let mut dfs = DepthFirstPaths::new(&graph, source_v);
    dfs.build(&graph);
    let mut bfs = BreadthFirstPaths::new(&graph, source_v);
    bfs.build(&graph);

    let (count, groups) = graph.get_cc();

    print!("{}", graph);
    connected(&dfs);
    println!("A Graph contains {count} independent components");
    for (id, list) in groups.iter().enumerate() {
        println!("Component {id}: {list:?}");
    }
    println!();
    println!("Depth first search\n{}", dfs);
    println!("Breadth first search\n{}", bfs);
    Ok(())
}

/*
    A graph is connected if there is a path from every vertex to every other vertex
    in the graph. A graph that is not connected consists of a set of connected components,
    which are maximal connected subgraphs.
*/
fn connected(p: &DepthFirstPaths) {
    if p.is_connected() {
        println!("A Graph is connected\n");
    } else {
        println!("A Graph is not connected\n")
    }
}
