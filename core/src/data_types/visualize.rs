use crate::data_containers::BinarySearchTree;
use crate::data_containers::binary_tree::Node;
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
    let total_nodes = nodes_list.len();

    if nodes_list.is_empty() {
        return Err(KvError::NoData);
    }

    let mut visual_nodes: Vec<VisualNode> = Vec::new();
    let mut visual_lines: Vec<VisualLine> = Vec::new();
    let mut max_depth = 0.0;

    // Run the layout calculation engine starting at depth 0 without a parent coordinate
    compute_layout(
        bst.root_node(),
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
        .build_cartesian_2d(-0.5..(total_nodes as f64 - 0.5), (max_depth + 0.5)..-0.5)
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
                12,
                BLUE.filled(),
            )))
            .map_err(|e| KvError::Plot(format!("Failed to draw node circle: {:?}", e)))?;

        // Center the text layout regardless of digit count
        use plotters::style::text_anchor::{HPos, Pos, VPos};
        let anchor_pos = Pos::new(HPos::Center, VPos::Center);

        let text_style = ("sans-serif", 15)
            .into_font()
            .color(&WHITE)
            // Use TextAlignment to handle layout dynamically
            .pos(anchor_pos);

        // Draw text label centered inside the blue circle
        chart
            .draw_series(std::iter::once(Text::new(
                node.label.clone(),
                (node.x, node.y),
                text_style,
            )))
            .map_err(|e| KvError::Plot(format!("Failed to render text labels: {:?}", e)))?;
    }

    root.present()
        .map_err(|e| KvError::Plot(format!("Failed to write PNG file to storage: {:?}", e)))?;

    Ok(())
}

// 🚀 True structural layout engine:
// Recursively traverses the actual BST structure while using the global sorted list for X coordinates.
fn compute_layout<K: Ord + std::fmt::Display, V>(
    current_node: Option<&Node<K, V>>,
    global_list: &[&Node<K, V>],
    depth: f64,
    parent_coord: Option<(f64, f64)>,
    visual_nodes: &mut Vec<VisualNode>,
    visual_lines: &mut Vec<VisualLine>,
    max_depth: &mut f64,
) {
    // 1. Base case: if the pointer is empty, stop traversing
    let Some(node) = current_node else {
        return;
    };

    // 2. Find where this node physically sits in the global sorted list to get its stable X coordinate
    let global_x = global_list
        .iter()
        .position(|&n| std::ptr::eq(n, node)) // Using ptr::eq prevents key collision issues
        .unwrap() as f64;

    let current_coord = (global_x, depth);

    if depth > *max_depth {
        *max_depth = depth;
    }

    // 3. Save the node for drawing
    visual_nodes.push(VisualNode {
        x: global_x,
        y: depth,
        label: format!("{}", node.key),
    });

    // 4. Add branch connection line back up to the parent node
    if let Some(parent) = parent_coord {
        visual_lines.push(VisualLine {
            start: parent,
            end: current_coord,
        });
    }

    // 5. Recursively follow the ACTUAL tree pointers, not array slices
    compute_layout(
        node.left.0.as_deref(), // Assumes left is Option<Box<Node<K, V>>> or similar
        global_list,
        depth + 1.0,
        Some(current_coord),
        visual_nodes,
        visual_lines,
        max_depth,
    );

    compute_layout(
        node.right.0.as_deref(),
        global_list,
        depth + 1.0,
        Some(current_coord),
        visual_nodes,
        visual_lines,
        max_depth,
    );
}
