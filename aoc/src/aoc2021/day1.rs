use aoc::{aoc, aoc_input};

#[aoc(2021, 1)]
pub fn main() {
    let data = aoc_input!(2021, 1).unwrap();
    let measurements = data
        .lines()
        .flat_map(|x| x.parse::<u32>())
        .collect::<Vec<_>>();

    // Part I
    let p1 = measurements.windows(2).filter(|w| w[0] < w[1]).count();
    println!("{p1}");

    // Part II
    let p2 = measurements
        .windows(4)
        .filter(|w| w[0..3].iter().sum::<u32>() < w[1..].iter().sum())
        .count();
    println!("{p2}");
}
