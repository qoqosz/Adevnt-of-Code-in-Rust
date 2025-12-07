use aoc::aoc_input;
use aoc::graph::parse_graph;
use petgraph::algo::{astar, Measure};
use petgraph::graph::DiGraph;
use petgraph::visit::EdgeRef;
use std::cmp::max;
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

fn increment(data: &str) -> String {
    data.to_owned()
        .chars()
        .map(|c| match c {
            ref x if x.is_ascii_digit() => max(49, (*x as u8 + 1) % 58) as char,
            x => x,
        })
        .collect()
}

fn extend(data: &str) -> String {
    let mut out: String = String::new();

    for line in data.lines() {
        let mut inc: String = line.to_string();
        out += &inc;

        for _ in 0..4 {
            inc = increment(&inc);
            out += &inc;
        }

        out += "\n";
    }

    let mut lines = out.lines().map(|x| x.to_string()).collect::<Vec<_>>();
    let n = lines.len();

    for i in 0..4 {
        for j in 0..n {
            let inc = increment(&lines[i * n + j]);
            lines.push(inc);
        }
    }

    lines.join("\n")
}

pub fn main() {
    let data = aoc_input!(2021, 15).unwrap();

    // Part I
    let graph: DiGraph<u32, ()> = parse_graph(&data, |c| c.parse::<u32>().unwrap());
    let dist = shortest_path(&graph).unwrap();
    println!("{}", dist);

    // Part II
    let data2 = extend(&data);
    let graph = parse_graph(&data2, |c| c.parse::<u32>().unwrap());
    let dist = shortest_path(&graph).unwrap();
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
