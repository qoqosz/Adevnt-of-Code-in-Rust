use aoc::aoc_input;
use itertools::Itertools;
use petgraph::prelude::*;
use std::collections::HashMap;

fn shortest_path(graph: &Graph<&str, i32, Undirected>) -> Option<i32> {
    let mut min_distance = i32::MAX;
    let n_nodes = graph.node_count();

    for route in (0..n_nodes).permutations(n_nodes) {
        let mut dist: i32 = 0;

        for win in route.windows(2) {
            let e = graph
                .find_edge(NodeIndex::new(win[0]), NodeIndex::new(win[1]))
                .unwrap();
            dist += graph.edge_weight(e).unwrap();
        }
        if dist < min_distance {
            min_distance = dist
        }
    }
    match min_distance {
        i32::MAX => None,
        _ => Some(min_distance),
    }
}

fn main() {
    let data = aoc_input!(2015, 9).unwrap();
    let mut graph = Graph::<&str, i32, Undirected>::new_undirected();
    let mut map: HashMap<&str, NodeIndex> = HashMap::new();

    for line in data.split('\n').filter(|x| !x.is_empty()) {
        let words: Vec<_> = line.split(' ').collect();
        let from = words[0];
        let dest = words[2];
        let dist: i32 = words.last().unwrap().parse().unwrap();
        let node1 = map
            .entry(from)
            .or_insert_with(|| graph.add_node(from))
            .clone();
        let node2 = map
            .entry(dest)
            .or_insert_with(|| graph.add_node(dest))
            .clone();
        graph.add_edge(node1, node2, dist);
    }

    // Part I
    println!("{}", shortest_path(&graph).unwrap());

    // Part II
    for edge in graph.edge_weights_mut() {
        *edge *= -1;
    }
    println!("{}", -shortest_path(&graph).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let mut graph = Graph::<&str, usize, Undirected>::new_undirected();
        let d = graph.add_node("Dublin");
        let l = graph.add_node("London");
        let b = graph.add_node("Belfast");
        let n = graph.node_count();
        graph.extend_with_edges(&[(d, l, 464), (d, b, 141), (l, b, 518)]);

        assert_eq!(605, shortest_path(&graph).unwrap());
    }
}
