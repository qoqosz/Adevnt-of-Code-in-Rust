use aoc::graph::parse_graph;
use aoc::load_input;
use aoc_cache::get;
use petgraph::algo::{astar, Measure};
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use std::fmt::Debug;

#[allow(dead_code)]
static TEST_CASE: &'static str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

fn shortest_path<T>(graph: &DiGraph<T, ()>) -> Option<T>
where
    T: Copy + Debug + Default + Measure,
{
    let start = graph.node_indices().nth(0).unwrap();
    let goal = graph.node_indices().last().unwrap();
    let path = astar(
        &graph,
        start,
        |f| f == goal,
        |e| {
            let idx: NodeIndex = e.target().into();
            graph[idx]
        },
        |_| T::default(),
    );
    path.unzip().0
}

fn main() {
    const MY_COOKIE: &str = include_str!("session.cookie");
    let data =
        load_input!("/Users/qoqosz/Documents/Coding/Rust/Advent of Code/data/2021/day15.txt");
    let graph: DiGraph<u32, ()> = parse_graph(&data, |c| c.parse::<u32>().unwrap());
    let dist = shortest_path(&graph).expect("Could not find the shortest path");

    println!("{:?}", dist);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let graph: DiGraph<u8, ()> = parse_graph(TEST_CASE, |c| c.parse::<u8>().unwrap());
        let dist = shortest_path(&graph).unwrap();
        assert_eq!(dist, 40);
    }
}
