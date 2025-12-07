use aoc::{aoc, aoc_input, heap::MinHeap};
use rustc_hash::{FxHashMap, FxHashSet};

type Graph = FxHashMap<(i16, i16), char>;

fn build_graph(data: &str) -> Graph {
    data.trim_end()
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.as_bytes()
                .into_iter()
                .enumerate()
                .map(move |(j, ch)| ((i as i16, j as i16), *ch as char))
        })
        .collect()
}

fn neighbors(p: &(i16, i16)) -> impl IntoIterator<Item = (i16, i16)> {
    vec![
        (p.0 - 1, p.1),
        (p.0 + 1, p.1),
        (p.0, p.1 - 1),
        (p.0, p.1 + 1),
    ]
}

#[inline]
fn is_transition(start: char, end: char) -> bool {
    match (start, end) {
        ('E', 'y' | 'z') | ('a', 'S') => true,
        ('E', _) | (_, 'S') => false,
        (src @ _, dest @ _) => (dest as i8) + 1 >= (src as i8),
    }
}

fn shortest_path_len(graph: &Graph, start: (i16, i16), target: char) -> Option<usize> {
    let mut queue = MinHeap::from([(0, start)]);
    let mut visited = FxHashSet::default();

    while let Some((dist, pos)) = queue.pop() {
        if let Some(&ch) = graph.get(&pos) {
            // already visited
            if !visited.insert(pos) {
                continue;
            }
            // end condition
            if ch == target {
                return Some(dist);
            }
            // visit neighbors
            for n in neighbors(&pos) {
                if let Some(&next) = graph.get(&n) {
                    if is_transition(ch, next) {
                        queue.push(dist + 1, n);
                    }
                }
            }
        }
    }

    None
}

#[allow(non_snake_case)]
#[aoc(2022, 12)]
pub fn main() {
    let data = aoc_input!(2022, 12).unwrap();
    let graph = build_graph(&data);
    let E = graph
        .iter()
        .find(|(_, v)| **v == 'E')
        .map(|(k, _)| *k)
        .unwrap();

    // Part I
    let n_steps1 = shortest_path_len(&graph, E, 'S').unwrap();
    println!("{n_steps1}");

    // Part II
    let n_steps2 = shortest_path_len(&graph, E, 'a').unwrap();
    println!("{n_steps2}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    static EXAMPLE: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[fixture]
    pub fn graph() -> Graph {
        build_graph(EXAMPLE)
    }

    #[rstest]
    fn test_end(#[by_ref] graph: &Graph) {
        assert_eq!(graph.get(&(2, 5)), Some(&'E'));
    }

    #[rstest]
    fn test_part1(#[by_ref] graph: &Graph) {
        let res = shortest_path_len(graph, (2, 5), 'S');
        assert_eq!(res, Some(31));
    }

    #[rstest]
    fn test_part2(#[by_ref] graph: &Graph) {
        let res = shortest_path_len(graph, (2, 5), 'a');
        assert_eq!(res, Some(29));
    }
}
