use aoc::aoc_input;
use aoc::graph::parse_graph;
use petgraph::algo::{astar, Measure};
use petgraph::graph::DiGraph;
use petgraph::visit::EdgeRef;
use std::fmt::Debug;

fn shortest_path<T>(graph: &DiGraph<T, ()>) -> Option<T>
where
    T: Copy + Debug + Default + Measure,
{
    let start = graph.node_indices().next().unwrap();
    let goal = graph.node_indices().last().unwrap();
    let path = astar(
        &graph,
        start,
        |f| f == goal,
        |e| graph[e.target()],
        |_| T::default(),
    );
    path.unzip().0
}

fn main() {
    let data = aoc_input!(2021, 15).unwrap();
    let graph: DiGraph<u32, ()> = parse_graph(&data, |c| c.parse::<u32>().unwrap());
    let dist = shortest_path(&graph).unwrap();

    // Part I
    println!("{}", dist);
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_CASE: &str = "1163751742\n\
                              1381373672\n\
                              2136511328\n\
                              3694931569\n\
                              7463417111\n\
                              1319128137\n\
                              1359912421\n\
                              3125421639\n\
                              1293138521\n\
                              2311944581";

    #[test]
    fn test_part_1() {
        let graph: DiGraph<u8, ()> = parse_graph(TEST_CASE, |c| c.parse::<u8>().unwrap());
        let dist = shortest_path(&graph).unwrap();
        assert_eq!(dist, 40);
    }
}
