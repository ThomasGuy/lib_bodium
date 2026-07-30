use thiserror::Error;

use super::{Config, DiGraph, Graph, GraphError};
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
    #[error("Graph compilation aborted: The input data file contains an infinite cycle: \n{0:?}")]
    CyclicDeadlock(crate::data_containers::Stack<usize>),
}

pub fn build_graph(config: &Config) -> Result<Graph, InError> {
    let (vertex, edge, iter) = graph_input(config)?;
    let mut graph = Graph::new(vertex);
    graph.build(edge, iter)?;
    Ok(graph)
}

pub fn build_digraph(config: &Config) -> Result<DiGraph, InError> {
    let (vertex, edge, iter) = graph_input(config)?;
    let mut digraph = DiGraph::new(vertex);
    digraph.build(edge, iter)?;

    // 🚀 AUTOMATED SAFETY GATE: Enforce that the digraph must be acyclic!
    // If it's broken, extract the cycle path to give user diagnostic feedback
    if let Some(deadlock_path) = digraph.cycle() {
        return Err(InError::CyclicDeadlock(deadlock_path));
    }
    Ok(digraph)
}

fn graph_input(config: &Config) -> Result<(usize, usize, std::vec::IntoIter<i32>), InError> {
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

    Ok((vertex as usize, edge as usize, iter))
}
