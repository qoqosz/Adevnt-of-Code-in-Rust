use aoc::{aoc, aoc_input};
use itertools::Itertools;

fn optimize<F>(positions: &[i32], cost: F) -> Option<i32>
where
    F: Fn(i32) -> i32,
{
    let (start, end) = match positions.iter().minmax() {
        itertools::MinMaxResult::MinMax(a, b) => (*a, *b),
        _ => unreachable!(),
    };
    (start..=end)
        .map(|target| {
            positions
                .iter()
                .map(|pos| cost((pos - target).abs()))
                .sum::<i32>()
        })
        .min()
}

#[aoc(2021, 7)]
pub fn main() {
    let data = aoc_input!(2021, 7).unwrap();
    let positions = data
        .trim()
        .split(',')
        .flat_map(|x| x.parse::<i32>())
        .collect::<Vec<_>>();

    // Part I
    let cost1 = optimize(&positions, |p| p).unwrap();
    println!("{cost1}");

    // Part II
    let cost2 = optimize(&positions, |p| p * (p + 1) / 2).unwrap();
    println!("{cost2}");
}
