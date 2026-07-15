use crate::data_containers::BinarySearchTree;
use crate::data_types::KvError;
use plotters::prelude::*;

/// Represents a node's calculated coordinates and its label for drawing
struct VisualNode {
    x: f64,
    y: f64,
    label: String,
}

/// Represents a line connection between a parent node and a child node
struct VisualLine {
    start: (f64, f64),
    end: (f64, f64),
}

pub fn draw_tree_graph<K, V>(output_path: &str, bst: &BinarySearchTree<K, V>) -> Result<(), KvError>
where
    K: Ord + Clone + std::fmt::Display,
    V: Clone + std::fmt::Display,
{
    let root = BitMapBackend::new(output_path, (1000, 600)).into_drawing_area();
    let soft_background = RGBColor(225, 225, 225);
    root.fill(&soft_background)
        .map_err(|e| KvError::Plot(format!("Failed to fill canvas: {:?}", e)))?;

    // 1. Fetch the in-order sorted list of nodes
    let nodes_list = bst.nodes();
    let total_elements = nodes_list.len();

    if nodes_list.is_empty() {
        return Err(KvError::NoData);
    }

    let mut visual_nodes: Vec<VisualNode> = Vec::new();
    let mut visual_lines: Vec<VisualLine> = Vec::new();
    let mut max_depth = 0.0;

    // 2. 🚀 Divide-and-Conquer Layout Engine:
    // Recursively simulates the tree structure to find exact X, Y coordinates and lines.
    fn compute_layout<K: Ord + std::fmt::Display, V>(
        nodes: &[&crate::data_containers::binary_tree::node::Node<K, V>],
        global_list: &[&crate::data_containers::binary_tree::node::Node<K, V>],
        depth: f64,
        parent_coord: Option<(f64, f64)>,
        visual_nodes: &mut Vec<VisualNode>,
        visual_lines: &mut Vec<VisualLine>,
        max_depth: &mut f64,
    ) {
        if nodes.is_empty() {
            return;
        }

        // The middle element of the sorted list is ALWAYS the root of this subtree
        let mid_idx = nodes.len() / 2;
        let current_node = nodes[mid_idx];

        // Find where this node physically sits in the global sorted list to get its stable X coordinate
        let global_x = global_list
            .iter()
            .position(|&n| n.key == current_node.key)
            .unwrap() as f64;

        let current_coord = (global_x, depth);
        if depth > *max_depth {
            *max_depth = depth;
        }

        // Save the node for drawing
        visual_nodes.push(VisualNode {
            x: global_x,
            y: depth,
            label: format!("{}", current_node.key),
        });

        // 🚀 Add branch connection line back up to the parent node if it exists
        if let Some(parent) = parent_coord {
            visual_lines.push(VisualLine {
                start: parent,
                end: current_coord,
            });
        }

        // Recursively calculate the left and right subtrees
        compute_layout(
            &nodes[..mid_idx],
            global_list,
            depth + 1.0,
            Some(current_coord),
            visual_nodes,
            visual_lines,
            max_depth,
        );
        compute_layout(
            &nodes[mid_idx + 1..],
            global_list,
            depth + 1.0,
            Some(current_coord),
            visual_nodes,
            visual_lines,
            max_depth,
        );
    }

    // Run the layout calculation engine starting at depth 0 without a parent coordinate
    compute_layout(
        &nodes_list,
        &nodes_list,
        0.0,
        None,
        &mut visual_nodes,
        &mut visual_lines,
        &mut max_depth,
    );

    // 3. Setup the chart canvas window (Y-axis runs top-to-bottom for cascading trees)
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Binary Search Tree Structural Visualization",
            ("sans-serif", 30).into_font(),
        )
        .margin(40)
        // Reverse Y-axis (max_depth + 0.5 down to -0.5) so root is at the absolute top
        .build_cartesian_2d(-0.5..(total_elements as f64 - 0.5), (max_depth + 0.5)..-0.5)
        .map_err(|e| KvError::Plot(format!("Failed to build chart context: {:?}", e)))?;

    // Turn off grid paper background
    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()
        .map_err(|e| KvError::Plot(format!("Failed to configure grid lines: {:?}", e)))?;

    // 4. 🚀 Draw the branch connection lines first (so they sit behind the circles)
    chart
        .draw_series(visual_lines.iter().map(|line| {
            PathElement::new(vec![line.start, line.end], BLACK.mix(0.6).stroke_width(2))
        }))
        .map_err(|e| KvError::Plot(format!("Failed to draw branch paths: {:?}", e)))?;

    // 5. Draw the node circles and white text labels
    for node in &visual_nodes {
        // Draw blue node circle
        chart
            .draw_series(std::iter::once(Circle::new(
                (node.x, node.y),
                18,
                BLUE.filled(),
            )))
            .map_err(|e| KvError::Plot(format!("Failed to draw node circle: {:?}", e)))?;

        // Draw text label centered inside the blue circle
        chart
            .draw_series(std::iter::once(Text::new(
                node.label.clone(),
                (node.x - 0.15, node.y - 0.05),
                ("sans-serif", 15).into_font().color(&WHITE),
            )))
            .map_err(|e| KvError::Plot(format!("Failed to render text labels: {:?}", e)))?;
    }

    root.present()
        .map_err(|e| KvError::Plot(format!("Failed to write PNG file to storage: {:?}", e)))?;
    Ok(())
}
