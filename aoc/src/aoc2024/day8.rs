use aoc::{aoc, aoc_input};
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::str::FromStr;

#[derive(Debug)]
struct Map {
    width: i32,
    height: i32,
    antenas: FxHashMap<(i32, i32), char>,
}

impl Map {
    fn freqs(&self) -> Vec<char> {
        self.antenas.values().copied().unique().collect()
    }

    fn get_freq(&self, freq: char) -> Vec<(i32, i32)> {
        self.antenas
            .iter()
            .filter_map(|(pos, f)| if *f == freq { Some(*pos) } else { None })
            .collect()
    }

    fn is_within(&self, pos: &(i32, i32)) -> bool {
        pos.0 >= 0 && pos.0 < self.width && pos.1 >= 0 && pos.1 < self.height
    }

    fn antinode(&self, x: &(i32, i32), y: &(i32, i32)) -> Option<(i32, i32)> {
        let dw = y.0 - x.0;
        let dh = y.1 - x.1;
        let antinode = (x.0 - dw, x.1 - dh);

        self.is_within(&antinode).then(|| antinode)
    }

    // Part I
    fn count_antinodes(&self) -> usize {
        let mut count = 0;

        for freq in self.freqs() {
            let antenas = self.get_freq(freq);

            for pair in antenas.iter().combinations(2) {
                let (x, y) = (pair[0], pair[1]);

                if let Some(antinode) = self.antinode(x, y) {
                    if !self.antenas.contains_key(&antinode) {
                        count += 1;
                    }
                }
                if let Some(antinode) = self.antinode(y, x) {
                    if !self.antenas.contains_key(&antinode) {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    // Part II
    fn count_updated_antinodes(&self) -> usize {
        let mut antinodes = FxHashSet::default();

        for freq in self.freqs() {
            let antenas = self.get_freq(freq);

            for pair in antenas.iter().combinations(2) {
                let (x, y) = (pair[0], pair[1]);
                let dw = y.0 - x.0;
                let dh = y.1 - x.1;

                for i in -50..=50 {
                    let antinode = (x.0 + i * dw, x.1 + i * dh);

                    if self.is_within(&antinode) {
                        antinodes.insert(antinode);
                    }
                }

                antinodes.insert(*x);
                antinodes.insert(*y);
            }
        }

        antinodes.len()
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.find('\n').unwrap() as i32;
        let height = s.trim().match_indices('\n').count() as i32 + 1;
        let antenas = s
            .trim()
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, ch)| ((x as i32, y as i32), ch))
            })
            .filter(|(_, ch)| *ch != '.')
            .collect();

        Ok(Self {
            width,
            height,
            antenas,
        })
    }
}

#[aoc(2024, 8)]
pub fn main() {
    let data = aoc_input!(2024, 8).unwrap();
    let map = Map::from_str(&data).unwrap();

    // Part I
    println!("{}", map.count_antinodes());

    // Part II
    println!("{}", map.count_updated_antinodes());
}
