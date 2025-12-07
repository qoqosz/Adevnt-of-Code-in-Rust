use aoc::{aoc, aoc_input};
use itertools::Itertools;

#[aoc(2018, 2)]
pub fn main() {
    let data = aoc_input!(2018, 2).unwrap();

    // Part I
    let check = data
        .trim()
        .lines()
        .map(|id| {
            let cnt = id.chars().counts();
            (
                cnt.values().contains(&2) as i32,
                cnt.values().contains(&3) as i32,
            )
        })
        .reduce(|acc, x| (acc.0 + x.0, acc.1 + x.1))
        .unwrap_or_default();
    println!("{}", check.0 * check.1);

    // Part II
    let pair = data
        .trim()
        .lines()
        .sorted()
        .tuple_windows()
        .find(|(id1, id2)| {
            id1.chars()
                .zip(id2.chars())
                .filter(|(c1, c2)| c1 != c2)
                .count()
                == 1
        })
        .unwrap();
    let common = pair
        .0
        .chars()
        .zip(pair.1.chars())
        .filter(|(c1, c2)| c1 == c2)
        .map(|(c1, _)| c1)
        .collect::<String>();
    println!("{common}");
}
