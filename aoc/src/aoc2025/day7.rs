use aoc::{aoc, aoc_input};
use rustc_hash::FxHashMap;

fn simulate(manifold: &Vec<Vec<u8>>) -> (usize, usize) {
    let start = manifold[0].iter().position(|x| *x == b'S').unwrap();
    let mut beams = FxHashMap::from_iter([(start, 1)]);
    let mut n_splits = 0;

    for row in manifold.iter().step_by(2) {
        let mut new_beams = FxHashMap::default();

        for (beam, count) in beams {
            match row[beam] {
                b'^' => {
                    new_beams
                        .entry(beam - 1)
                        .and_modify(|v| *v += count)
                        .or_insert(count);
                    new_beams
                        .entry(beam + 1)
                        .and_modify(|v| *v += count)
                        .or_insert(count);
                    n_splits += 1;
                }
                _ => {
                    new_beams
                        .entry(beam)
                        .and_modify(|v| *v += count)
                        .or_insert(count);
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
