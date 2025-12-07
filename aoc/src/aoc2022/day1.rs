use aoc::{aoc, aoc_input};
use itertools::Itertools;

#[aoc(2022, 1)]
pub fn main() {
    let data = aoc_input!(2022, 1).unwrap();
    let mut calories = data
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| -line.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .k_smallest(3);

    // Part I
    let total_max = calories.next().unwrap();
    println!("{}", -total_max);

    // Part II
    println!("{}", -calories.fold(total_max, |acc, x| acc + x));
}
