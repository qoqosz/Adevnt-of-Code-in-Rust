use aoc::{aoc, aoc_input};
use petgraph::{algo::all_simple_paths, prelude::*};
use rustc_hash::FxHashMap;
use std::collections::VecDeque;

type Map = FxHashMap<(i32, i32), char>;

fn parse(data: &str) -> Map {
    data.trim()
        .lines()
        .filter(|x| !x.is_empty())
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, ch)| ((x as i32, y as i32), ch))
        })
        .collect::<Map>()
}

fn build_graph(data: &str) {
    //}-> DiGraph<char, i32> {
    let map = parse(data);
    let mut idx = FxHashMap::default();

    let mut graph = Graph::new();

    // create nodes
    for (node, ch) in map {
        if ch == '#' {
            continue;
        }
        let i = graph.add_node(node);
        idx.insert(node, i);
    }
    // add edges
    for (node, node_idx) in &idx {
        for adj in [
            (node.0 + 1, node.1),
            (node.0 - 1, node.1),
            (node.0, node.1 + 1),
            (node.0, node.1 - 1),
        ] {
            if let Some(neighbor) = idx.get(&adj) {
                graph.add_edge(*node_idx, *neighbor, 1);
            }
        }
    }

    // simplify edges
    for _ in 0..100 {
        for node in graph.node_indices() {
            let adj_edges = graph.edges(node).collect::<Vec<_>>();
            let adj_nodes = graph.neighbors(node).collect::<Vec<_>>();

            if adj_edges.len() == 2 {
                let w = adj_edges[0].weight() + adj_edges[1].weight();
                let (n1, n2) = (adj_nodes[0], adj_nodes[1]);

                graph.add_edge(n1, n2, w);
                graph.add_edge(n2, n1, w);
                graph.remove_node(node);
            }
        }
    }

    // find start and end
    let mut start = NodeIndex::default();
    let mut end = NodeIndex::default();

    for node in graph.node_indices() {
        if graph[node] == (1, 0) {
            start = node;
        }
        if graph[node] == (139, 140) {
            end = node;
        }
    }

    let paths = all_simple_paths::<Vec<_>, _>(&graph, start, end, 0, None).collect::<Vec<_>>();

    let path_lens = paths.iter().map(|path| {
        path.windows(2)
            .map(|w| {
                let edge = graph.find_edge(w[0], w[1]).unwrap();
                *graph.edge_weight(edge).unwrap()
            })
            .sum::<i32>()
    });

    println!("{}", path_lens.max().unwrap());
}

fn longest_path(map: &Map, end: &(i32, i32)) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut paths = vec![];

    queue.push_back(vec![(1, 0)]);

    while let Some(path) = queue.pop_front() {
        let (x, y) = *path.last().unwrap();
        if (x, y) == *end {
            paths.push(path);
            continue;
        }

        let ch = match map.get(&(x, y)) {
            Some(val) => *val,
            None => {
                continue;
            }
        };

        if ch == '#' {
            continue;
        }
        if (ch == '.' || ch == '>') && !path.contains(&(x + 1, y)) {
            let mut new_path = path.clone();
            new_path.push((x + 1, y));
            queue.push_back(new_path);
        }
        if (ch == '.' || ch == '<') && !path.contains(&(x - 1, y)) {
            let mut new_path = path.clone();
            new_path.push((x - 1, y));
            queue.push_back(new_path);
        }
        if (ch == '.' || ch == 'v') && !path.contains(&(x, y + 1)) {
            let mut new_path = path.clone();
            new_path.push((x, y + 1));
            queue.push_back(new_path);
        }
        if (ch == '.' || ch == '^') && !path.contains(&(x, y - 1)) {
            let mut new_path = path.clone();
            new_path.push((x, y - 1));
            queue.push_back(new_path);
        }
    }

    paths.iter().map(|x| x.len() - 1).max()
}

#[aoc(2023, 23)]
pub fn main() {
    let data = aoc_input!(2023, 23).unwrap();
    let mut map = parse(&data);

    // Part I
    println!("{}", longest_path(&map, &(139, 140)).unwrap());

    // Part I bis
    build_graph(&data);
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    fn test_part1() {
        let map = parse(EXAMPLE);
        let n = longest_path(&map, &(21, 22)).unwrap();
        assert_eq!(n, 94);
    }

    #[test]
    fn test_part2() {
        let mut map = parse(EXAMPLE);
        for (_, v) in map.iter_mut() {
            if *v != '#' {
                *v = '.'
            }
        }
        let n = longest_path(&map, &(21, 22)).unwrap();
        assert_eq!(n, 154);
    }

    #[test]
    fn test_part1b() {
        build_graph(EXAMPLE);
    }
}
