use aoc::{aoc, aoc_input};
use aoc_core::counter::Counter;
use itertools::Itertools;
use rustc_hash::FxHashMap;

fn grow(polymer: &str, rules: &FxHashMap<(u8, u8), u8>, steps: usize) -> Counter<(u8, u8)> {
    (0..steps).fold(to_pairs(polymer), |pairs, _| insert_pairs(&pairs, rules))
}

fn to_pairs(polymer: &str) -> Counter<(u8, u8)> {
    polymer
        .as_bytes()
        .iter()
        .tuple_windows()
        .map(|(a, b)| (*a, *b))
        .collect()
}

fn insert_pairs(pairs: &Counter<(u8, u8)>, rules: &FxHashMap<(u8, u8), u8>) -> Counter<(u8, u8)> {
    let mut res = Counter::default();

    for (pair, count) in pairs {
        let new_char = rules.get(pair).unwrap();
        res.add((pair.0, *new_char), *count);
        res.add((*new_char, pair.1), *count);
    }

    res
}

fn score(pairs: &Counter<(u8, u8)>) -> usize {
    let mut counter: Counter<u8> = Counter::default();

    for (pair, count) in pairs {
        counter.add(pair.0, *count);
        counter.add(pair.1, *count);
    }

    match counter.values().minmax() {
        itertools::MinMaxResult::MinMax(a, b) => (b + 1) / 2 - (a + 1) / 2,
        _ => unreachable!(),
    }
}

fn parse(data: &str) -> (&str, FxHashMap<(u8, u8), u8>) {
    let mut lines = data.trim().lines();
    let template = lines.next().unwrap().trim();
    let rules = lines
        .flat_map(|l| l.split_once(" -> "))
        .map(|(x, y)| match (x.as_bytes(), y.as_bytes()) {
            ([a, b], [c]) => ((*a, *b), *c),
            _ => unreachable!(),
        })
        .collect();

    (template, rules)
}

#[aoc(2021, 14)]
pub fn main() {
    let data = aoc_input!(2021, 14).unwrap();
    let (template, rules) = parse(&data);

    // Part I
    println!("{}", score(&grow(template, &rules, 10)));

    // Part I
    println!("{}", score(&grow(template, &rules, 40)));
}
