use aoc::{aoc, aoc_input};
use itertools::Itertools;
use std::ops::RangeInclusive;

fn is_fresh(id: &u64, ranges: &[RangeInclusive<u64>]) -> bool {
    ranges.iter().any(|r| r.contains(id))
}

fn parse(data: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let (ranges, ids) = data.split_once("\n\n").unwrap();
    let ranges = ranges
        .trim()
        .lines()
        .map(|line| {
            let (a, b) = line.split_once('-').unwrap();
            let a = a.parse().unwrap();
            let b = b.parse().unwrap();
            a..=b
        })
        .sorted_unstable_by_key(|r| *r.start())
        .collect();
    let ids = ids.lines().flat_map(|line| line.parse()).collect();

    (ranges, ids)
}

#[aoc(2025, 5)]
pub fn main() {
    let data = aoc_input!(2025, 5).unwrap();
    let (ranges, ids) = parse(&data);

    // Part I
    let n_fresh = ids.iter().filter(|id| is_fresh(id, &ranges)).count();
    println!("{n_fresh}");

    // Part II
    let merged = ranges
        .into_iter()
        .fold(vec![], |mut v: Vec<RangeInclusive<u64>>, r| {
            match v.last_mut() {
                Some(last) if r.start() <= last.end() => {
                    *last = *last.start()..=*(last.end().max(r.end()))
                }
                _ => v.push(r),
            }
            v
        });
    let n = merged.iter().map(|r| r.end() - r.start() + 1).sum::<u64>();
    println!("{n}");
}
