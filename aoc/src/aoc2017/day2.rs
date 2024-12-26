use aoc::{aoc, aoc_input};
use itertools::{Itertools, MinMaxResult};

#[aoc(2017, 2)]
pub fn main() {
    let data = aoc_input!(2017, 2).unwrap();
    let spreadsheet: Vec<Vec<i32>> = data
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .flat_map(|x| x.parse::<i32>())
                .collect()
        })
        .collect();

    // Part I
    let checksum: i32 = spreadsheet
        .iter()
        .map(|row| match row.iter().minmax() {
            MinMaxResult::OneElement(_) => 0,
            MinMaxResult::MinMax(x, y) => y - x,
            _ => unreachable!(),
        })
        .sum();
    println!("{checksum}");

    // Part II
    let mut result = 0;

    for row in spreadsheet {
        for pair in row.iter().permutations(2) {
            let n = pair[0] / pair[1];
            if *pair[0] == n * pair[1] {
                result += n;
            }
        }
    }

    println!("{result}");
}
