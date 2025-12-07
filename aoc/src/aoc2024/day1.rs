use aoc::{aoc, aoc_input};
use itertools::Itertools;

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
    let mut similarity_score = 0;

    for x in left_sorted.iter() {
        let n = right_sorted.iter().filter(|y| *y == x).count();
        similarity_score += **x * (n as u64);
    }

    println!("{similarity_score}");
}
