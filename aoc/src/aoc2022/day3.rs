use aoc::{aoc, aoc_input};
use itertools::Itertools;
use rustc_hash::FxHashSet;

#[inline(always)]
fn priority(ch: &char) -> u16 {
    match ch {
        'a'..='z' => *ch as u16 - 96,
        'A'..='Z' => *ch as u16 - 38,
        _ => unreachable!(),
    }
}

#[aoc(2022, 3)]
pub fn main() {
    let data = aoc_input!(2022, 3).unwrap();

    // Part I
    let sum_priorities: u16 = data
        .trim()
        .lines()
        .map(|line| {
            let n = line.len();
            let (first, second) = line.split_at(n / 2);
            let (first, second) = (
                FxHashSet::from_iter(first.chars()),
                FxHashSet::from_iter(second.chars()),
            );
            let common = first.intersection(&second);
            common.map(priority).sum::<u16>()
        })
        .sum();
    println!("{sum_priorities}");

    // Part II
    let sum_priorities: u16 = data
        .trim()
        .split('\n')
        .chunks(3)
        .into_iter()
        .map(|chunk| chunk.map(|line| FxHashSet::from_iter(line.chars())))
        .map(|mut sets| {
            sets.next()
                .map(|set| {
                    sets.fold(set, |set1, set2| {
                        set1.intersection(&set2).copied().collect()
                    })
                })
                .unwrap()
        })
        .map(|x| x.iter().map(priority).sum::<u16>())
        .sum();
    println!("{sum_priorities}");
}
