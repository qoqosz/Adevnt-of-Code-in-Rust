use aoc::aoc_input;
use itertools::Itertools;
use rustc_hash::FxHashSet;
use std::borrow::Borrow;

fn intersect<I>(sets: I) -> FxHashSet<char>
where
    I: IntoIterator + Clone,
    I::Item: Borrow<FxHashSet<char>> + Clone,
{
    let first = match sets.clone().into_iter().next() {
        Some(val) => val.clone(),
        None => return FxHashSet::default(),
    };

    first
        .borrow()
        .iter()
        .filter(move |elem| {
            sets.clone()
                .into_iter()
                .all(|set| set.borrow().contains(elem))
        })
        .cloned()
        .collect()
}

fn main() {
    let data = aoc_input!(2020, 6).unwrap();
    let answers: Vec<Vec<FxHashSet<char>>> = data
        .trim_end()
        .split("\n\n")
        .map(|lines| {
            lines
                .lines()
                .map(|line| line.chars().collect::<FxHashSet<char>>())
                .collect()
        })
        .collect();

    // Part I
    println!(
        "{}",
        answers
            .iter()
            .map(|group| group.iter().flatten().unique().count())
            .sum::<usize>()
    );

    // Part II
    println!(
        "{}",
        answers
            .iter()
            .map(|group| intersect(group).len())
            .sum::<usize>()
    );
}
