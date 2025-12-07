use aoc::{aoc, aoc_input};
use itertools::Itertools;
use rustc_hash::FxHashMap;

fn parse(data: &str) -> (Vec<u64>, Vec<u64>) {
    let (mut a, mut b) = (vec![], vec![]);

    for line in data.lines().filter(|x| !x.is_empty()) {
        let (x, y) = line.split_once("   ").unwrap();
        a.push(x.parse::<u64>().unwrap());
        b.push(y.parse::<u64>().unwrap());
    }

    (a, b)
}

#[aoc(2024, 1)]
pub fn main() {
    let data = aoc_input!(2024, 1).unwrap();
    let (left, right) = parse(&data);

    // Part I
    let left_sorted: Vec<_> = left.iter().sorted_unstable().collect();
    let right_sorted: Vec<_> = right.iter().sorted_unstable().collect();

    let total_distance: u64 = left_sorted
        .iter()
        .zip(right_sorted.iter())
        .map(|(&&x, &&y)| x.abs_diff(y))
        .sum();
    println!("{total_distance}");

    // Part II
    let mut right_count: FxHashMap<u64, usize> = FxHashMap::default();

    for y in right_sorted {
        *right_count.entry(*y).or_default() += 1;
    }

    let similarity_score: u64 = left_sorted
        .iter()
        .map(|x| **x * (right_count.get(*x).copied().unwrap_or_default() as u64))
        .sum();
    println!("{similarity_score}");
}
