use aoc::aoc_input;
use itertools::Itertools;
use std::borrow::Borrow;
use std::collections::HashSet;
use std::hash::Hash;

fn intersect<I, T>(sets: I) -> HashSet<T>
where
    I: IntoIterator,
    I::Item: Borrow<HashSet<T>>,
    T: Eq + Copy + Hash,
{
    let mut sets = sets.into_iter();
    let mut result: HashSet<T> = sets
        .next()
        .map(|s| s.borrow().iter().copied().collect())
        .unwrap_or_default();

    for set in sets {
        result.retain(|elem| set.borrow().contains(elem));
    }

    result
}

fn main() {
    let data = aoc_input!(2020, 6).unwrap();
    let answers: Vec<Vec<HashSet<char>>> = data
        .trim_end()
        .split("\n\n")
        .map(|lines| {
            lines
                .lines()
                .map(|line| line.chars().collect::<HashSet<char>>())
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
