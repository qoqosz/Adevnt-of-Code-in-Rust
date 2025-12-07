use aoc::{aoc, aoc_input};
use rustc_hash::FxHashMap;

/// Add `value` to an existing one in a `map` at `key` or insert it.
fn increment(map: &mut FxHashMap<usize, usize>, key: usize, value: usize) {
    map.entry(key).and_modify(|v| *v += value).or_insert(value);
}

/// Run both part I and part II simulations.
fn simulate(manifold: &Vec<Vec<u8>>) -> (usize, usize) {
    let start = manifold[0].iter().position(|x| *x == b'S').unwrap();
    let mut beams = FxHashMap::from_iter([(start, 1)]);
    let mut n_splits = 0;

    for row in manifold.iter().step_by(2) {
        let mut new_beams = FxHashMap::default();

        for (beam, count) in beams {
            match row[beam] {
                b'^' => {
                    increment(&mut new_beams, beam - 1, count);
                    increment(&mut new_beams, beam + 1, count);
                    n_splits += 1;
                }
                _ => {
                    increment(&mut new_beams, beam, count);
                }
            }
        }

        beams = new_beams;
    }

    (n_splits, beams.values().sum())
}

fn parse(data: &str) -> Vec<Vec<u8>> {
    data.trim()
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect()
}

#[aoc(2025, 7)]
pub fn main() {
    let data = aoc_input!(2025, 7).unwrap();
    let manifold = parse(&data);
    let (n_splits, n_paths) = simulate(&manifold);

    // Part I
    println!("{n_splits}");

    // Part II
    println!("{n_paths}");
}
