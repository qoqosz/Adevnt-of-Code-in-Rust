use aoc::{aoc, aoc_input};
use petgraph::graphmap::GraphMap;

#[aoc(2022, 12)]
pub fn main() {
    let data = aoc_input!(2022, 12).unwrap();
    let grid = data
        .trim_end()
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // Part I
    // let mut graph: GraphMap<(usize, usize), i32> = GraphMap::new();

    // todo:
    // 1. add all nodes (node type, index + char)
    // 2. then iterate and add edges

    // Part II
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "";

    #[test]
    fn test_part1() {}

    #[test]
    fn test_part2() {}
}
