use thiserror::Error;

use super::{DiGraph, Graph, GraphError};
use crate::data_types::{In, InputError};

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum InError {
    #[error("Graph error: {0}")]
    Graph(#[from] GraphError), // Wraps your existing enum
    #[error("Input error: {0}")]
    In(#[from] InputError), // Wraps your existing enum
    #[error("No data: {0}")]
    Input(String),
    #[error("source vertex given as arg ({0}) is outside of the graph range (0..{1})")]
    RangeError(i32, i32),
    #[error("Vertex index cannot be negative")]
    NegativeInt,
    #[error("Graph compilation aborted: The input data file contains an infinite cycle: {0:?}")]
    CyclicDeadlock(crate::data_containers::Stack<usize>),
}

pub fn build_graph(config: &Config) -> Result<Graph, InError> {
    let ints = In::build(&config.file_path)?;
    let mut iter = ints.into_iter();

    let vertex = iter.next().ok_or(InError::Input(
        ("Missing total number of verticies").to_string(),
    ))?;

    let edge = iter
        .next()
        .ok_or(InError::Input(("Missing expected edges").to_string()))?;

    let source = config.source_vertex;
    if source >= vertex {
        return Err(InError::RangeError(source, vertex));
    };

    let mut graph = Graph::new(vertex as usize);
    graph.build(edge as usize, iter)?;
    Ok(graph)
}

pub fn build_digraph(config: &Config) -> Result<DiGraph, InError> {
    let ints = In::build(&config.file_path)?;
    let mut iter = ints.into_iter();

    let vertex = iter.next().ok_or(InError::Input(
        ("Missing total number of verticies").to_string(),
    ))?;

    let edge = iter
        .next()
        .ok_or(InError::Input(("Missing expected edges").to_string()))?;

    let source = config.source_vertex;
    if source >= vertex {
        return Err(InError::RangeError(source, vertex));
    };

    let mut graph = DiGraph::new(vertex as usize);
    graph.build(edge as usize, iter)?;

    // 🚀 AUTOMATED SAFETY GATE: Enforce that the graph must be acyclic!
    // If it's broken, extract the cycle path to give user diagnostic feedback
    if let Some(deadlock_path) = graph.cycle() {
        return Err(InError::CyclicDeadlock(deadlock_path));
    }

    Ok(graph)
}

pub struct Config {
    file_path: String,
    pub source_vertex: i32,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let file_path = args.next().ok_or("Missing file path")?;
        let source_vertex = args
            .next()
            .ok_or("Missing source_vertex argument")?
            .parse()
            .map_err(|_| "The source_vertex arg must be an integer")?;

        Ok(Self {
            file_path,
            source_vertex,
        })
    }
}
