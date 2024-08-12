use aoc::{aoc, aoc_input};
use rustc_hash::FxHashSet;

#[aoc(2018, 1)]
pub fn main() {
    let data = aoc_input!(2018, 1).unwrap();
    let shifts = data
        .lines()
        .flat_map(|x| x.parse::<i32>())
        .collect::<Vec<_>>();

    // Part I
    let freq = shifts.iter().sum::<i32>();
    println!("{freq}");

    // Part II
    let mut seen = FxHashSet::<i32>::default();
    let (mut i, mut freq, n) = (0, 0, shifts.len());

    loop {
        freq += shifts[i];

        if !seen.insert(freq) {
            println!("{freq}");
            break;
        }

        i = (i + 1) % n;
    }
}
