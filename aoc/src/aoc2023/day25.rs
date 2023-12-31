use aoc::{aoc, aoc_input};
use rustworkx_core::connectivity::stoer_wagner_min_cut;
use rustworkx_core::petgraph::graphmap::UnGraphMap;

fn parse(data: &str) -> UnGraphMap<&str, ()> {
    let edges = data.trim().lines().flat_map(|line| {
        let (key, vals) = line.split_once(": ").unwrap();
        vals.split(' ').map(move |val| (key, val))
    });
    UnGraphMap::<&str, ()>::from_edges(edges)
}

fn solve(graph: &UnGraphMap<&str, ()>) -> Option<usize> {
    let cut: rustworkx_core::Result<Option<(usize, Vec<_>)>> =
        stoer_wagner_min_cut(&graph, |_| Ok(1));
    let (_, partition) = cut.ok()??;
    let (n, m) = (graph.node_count(), partition.len());

    Some(m * (n - m))
}

#[aoc(2023, 25)]
pub fn main() {
    let data = aoc_input!(2023, 25).unwrap();
    let graph = parse(&data);

    // Part I
    println!("{}", solve(&graph).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

    #[test]
    fn test_part1() {
        let graph = parse(EXAMPLE);
        assert_eq!(solve(&graph), Some(54));
    }
}
