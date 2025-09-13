use aoc::{aoc, aoc_input};
use itertools::Itertools;
use rustc_hash::FxHashSet;
use std::hash::Hash;

fn is_valid<'a, T, F>(line: &'a str, f: F) -> bool
where
    F: Fn(&'a str) -> T + 'a,
    T: Eq + Hash,
{
    let words: Vec<T> = line.split_whitespace().map(f).collect();
    let unique = FxHashSet::from_iter(&words);

    words.len() == unique.len()
}

fn char_counts(word: &str) -> Vec<(char, usize)> {
    word.chars()
        .counts()
        .into_iter()
        .sorted_by_key(|x| x.0)
        .collect()
}

#[aoc(2017, 4)]
pub fn main() {
    let data = aoc_input!(2017, 4).unwrap();
    let lines = data.trim().lines();

    // Part I
    let n = lines
        .clone()
        .filter(|passphrase| is_valid(passphrase, |w| w))
        .count();
    println!("{n}");

    // Part II
    let n = lines
        .filter(|passphrase| is_valid(passphrase, char_counts))
        .count();
    println!("{n}");
}
