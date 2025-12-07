use crate::num::Unsigned;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::EdgeType;
use std::str::FromStr;

/// Parse a string into a 2D grid graph
pub fn parse_graph<N, F, Ty>(text: &str, parse: F) -> Graph<N, (), Ty>
where
    N: Unsigned + FromStr + Copy,
    F: Fn(&str) -> N,
    Ty: EdgeType,
{
    let text = text.trim();
    let lines = text.lines().filter(|l| !l.is_empty());
    let n_rows = text.matches('\n').count();
    let n_cols = text.find('\n').unwrap();
    let get_idx = |row, col| row * n_cols + col;
    let mut g = Graph::with_capacity(n_cols * n_cols, 4 * n_rows * n_cols);

    for (row, line) in lines.enumerate() {
        for (col, word) in line.split("").filter(|c| !c.is_empty()).enumerate() {
            let current_node = parse(word);
            let current_index = g.add_node(current_node);

            if col > 0 {
                let previous_index = NodeIndex::new(get_idx(row, col - 1));
                g.add_edge(previous_index, current_index, ());
                if g.is_directed() {
                    g.add_edge(current_index, previous_index, ());
                }
            }
            if row > 0 {
                let previous_index = NodeIndex::new(get_idx(row - 1, col));
                g.add_edge(previous_index, current_index, ());
                if g.is_directed() {
                    g.add_edge(current_index, previous_index, ());
                }
            }
        }
    }
    g.shrink_to_fit();
    g
}
