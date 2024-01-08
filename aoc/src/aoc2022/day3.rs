use aoc::{aoc, aoc_input};
use itertools::Itertools;
use std::borrow::Borrow;

#[inline(always)]
fn priority<C: Borrow<char>>(ch: C) -> u16 {
    let ch = *ch.borrow();
    match ch {
        'a'..='z' => ch as u16 - 96,
        'A'..='Z' => ch as u16 - 38,
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
        .flat_map(|line| {
            let n = line.len();
            let (first, second) = line.split_at(n / 2);
            first.chars().find(|ch| second.contains(*ch)).map(priority)
        })
        .sum();
    println!("{sum_priorities}");

    // Part II
    let sum_priorities: u16 = data
        .trim()
        .lines()
        .chunks(3)
        .into_iter()
        .flat_map(|mut lines| {
            let mut first = lines.next().unwrap().chars();
            // let mut first = lines.next().unwrap().chars().collect::<Vec<_>>();
            first.find(|ch| lines.all(|l| l.contains(*ch)));
            first.map(priority)
        })
        .sum();
    println!("{sum_priorities}");
}
