use aoc::{aoc, aoc_input};
use itertools::Itertools;
use rustc_hash::FxHashSet;

trait Neighbors {
    fn adj(&self) -> impl Iterator<Item = Self>;
}

impl Neighbors for (i32, i32) {
    fn adj(&self) -> impl Iterator<Item = Self> {
        (-1..=1)
            .cartesian_product(-1..=1)
            .filter(|(i, j)| !(*i == 0 && *j == 0))
            .map(|(i, j)| (self.0 + i, self.1 + j))
    }
}

fn parse(data: &str) -> FxHashSet<(i32, i32)> {
    data.trim()
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .filter(|(_, ch)| **ch == b'@')
                .map(move |(j, _)| (i as i32, j as i32))
        })
        .collect()
}

#[aoc(2025, 4)]
pub fn main() {
    let data = aoc_input!(2025, 4).unwrap();
    let mut diagram = parse(&data);

    // Part I
    let n_rolls = diagram
        .iter()
        .filter(|k| k.adj().filter(|v| diagram.contains(v)).count() < 4)
        .count();
    println!("{n_rolls}");

    // Part II
    let total = diagram.len();

    loop {
        let to_remove = diagram
            .iter()
            .filter(|k| k.adj().filter(|v| diagram.contains(v)).count() < 4)
            .copied()
            .collect::<FxHashSet<_>>();

        if to_remove.is_empty() {
            break;
        }

        for elem in &to_remove {
            diagram.remove(elem);
        }
    }

    println!("{}", total - diagram.len());
}
