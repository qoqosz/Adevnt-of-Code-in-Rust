use aoc::{aoc, aoc_input};
use itertools::Itertools;

fn correct_error(grid: &[u8], key_fn: impl Fn(usize) -> usize) -> String {
    let width = *grid.iter().find(|ch| ch.is_ascii_whitespace()).unwrap() as usize - 2;

    (0..width)
        .filter_map(|i| {
            grid.iter()
                .skip(i)
                .filter(|ch| !ch.is_ascii_whitespace())
                .step_by(width)
                .map(|ch| *ch as char)
                .counts()
                .iter()
                .max_by_key(|(_, v)| key_fn(**v))
                .map(|(k, _)| *k)
        })
        .collect()
}

#[aoc(2016, 6)]
pub fn main() {
    let data = aoc_input!(2016, 6).unwrap();
    let grid = data.as_bytes();

    // Part I
    let msg = correct_error(grid, |x| x);
    println!("{msg}");

    // Part II
    let msg = correct_error(grid, |x| usize::MAX - x);
    println!("{msg}");
}
