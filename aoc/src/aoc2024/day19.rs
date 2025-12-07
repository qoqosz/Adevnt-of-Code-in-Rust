use aoc::{aoc, aoc_input};
use itertools::Itertools;
use rustc_hash::FxHashMap;

fn parse(data: &str) -> (Vec<&str>, Vec<&str>) {
    let (towels, patterns) = data.trim().split_once("\n\n").unwrap();

    (
        towels
            .split(", ")
            .sorted_by_key(|x| x.len())
            .rev()
            .collect(),
        patterns.lines().collect(),
    )
}

fn check_design<'a>(
    pattern: &'a str,
    towels: &[&str],
    cache: &mut FxHashMap<&'a str, usize>,
) -> usize {
    if let Some(ans) = cache.get(pattern) {
        return *ans;
    }

    let n = pattern.len();

    if n == 0 {
        return 1;
    }

    let val = towels
        .iter()
        .filter(|towel| pattern.starts_with(**towel))
        .map(|towel| &pattern[towel.len()..])
        .filter(|p| p.len() < n)
        .map(|p| check_design(p, towels, cache))
        .sum();

    cache.insert(pattern, val).unwrap_or(val)
}

#[aoc(2024, 19)]
pub fn main() {
    let data = aoc_input!(2024, 19).unwrap();
    let (towels, patterns) = parse(&data);
    let mut cache = FxHashMap::default();
    let designs: Vec<_> = patterns
        .iter()
        .map(|p| check_design(p, &towels, &mut cache))
        .collect();

    // Part I
    let n_designs: usize = designs.iter().filter(|n| **n > 0).count();
    println!("{n_designs}");

    // Part II
    let n_ways: usize = designs.iter().sum();
    println!("{n_ways}");
}
