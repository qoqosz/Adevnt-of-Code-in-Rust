use aoc::{aoc, aoc_input};
use petgraph::algo::dijkstra;
use petgraph::graphmap::{DiGraphMap, UnGraphMap};

fn parse(data: &str) -> DiGraphMap<&str, ()> {
    DiGraphMap::<_, ()>::from_edges(
        data.lines()
            .filter(|x| !x.is_empty())
            .filter_map(|l| l.split_once(')')),
    )
}

fn count_orbits(graph: &DiGraphMap<&str, ()>) -> usize {
    dijkstra(graph, "COM", None, |_| 1).values().sum()
}

fn minimum_transfers<'a>(graph: &'a DiGraphMap<&'a str, ()>) -> Option<usize> {
    let ungraph: UnGraphMap<&str, ()> = UnGraphMap::from_edges(graph.all_edges());
    let res = dijkstra(&ungraph, "YOU", Some("SAN"), |_| 1);
    res.get("SAN").map(|x: &usize| x.saturating_sub(2))
}

#[aoc(2019, 6)]
pub fn main() {
    let data = aoc_input!(2019, 6).unwrap();
    let graph = parse(&data);

    // Part I
    let n_orbits = count_orbits(&graph);
    println!("{n_orbits}");

    // Part II
    let n_transfers = minimum_transfers(&graph).unwrap();
    println!("{n_transfers}");
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE1: &str = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";

    static EXAMPLE2: &str = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";

    #[test]
    fn test_parse() {
        let graph = parse(&EXAMPLE1);
        assert_eq!(graph.node_count(), 12);
    }

    #[test]
    fn test_part1() {
        let graph = parse(&EXAMPLE1);
        let n = count_orbits(&graph);
        assert_eq!(n, 42);
    }

    #[test]
    fn test_part2() {
        let graph = parse(&EXAMPLE2);
        let n = minimum_transfers(&graph);
        assert_eq!(n, Some(4));
    }
}
