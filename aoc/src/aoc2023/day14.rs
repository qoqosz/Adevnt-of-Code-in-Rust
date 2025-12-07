use aoc::{aoc, aoc_input};
use rustc_hash::FxHashMap;
use std::{fmt::Display, hash::Hash, str::FromStr};

#[derive(Clone, Eq, PartialEq)]
struct Platform {
    platform: FxHashMap<(u32, u32), char>,
    n: u32,
}

impl FromStr for Platform {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let platform = s
            .lines()
            .enumerate()
            .flat_map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(j, ch)| ((i as u32, j as u32), ch))
            })
            .collect::<FxHashMap<_, _>>();
        let n = platform.keys().map(|(r, _)| *r).max().unwrap() + 1;

        Ok(Self { platform, n })
    }
}

impl Platform {
    fn tilt(&mut self) {
        for row in 1..self.n {
            for col in 0..self.n {
                if let Some(&'O') = self.platform.get(&(row, col)) {
                    // find new pos where it will roll
                    // iterate col..0 and find first non '.'
                    let new_row = (0..row)
                        .rev()
                        .find(|&r| *self.platform.get(&(r, col)).unwrap() != '.')
                        .map_or(0, |x| x + 1);

                    if new_row != row {
                        *self.platform.get_mut(&(row, col)).unwrap() = '.';
                        *self.platform.get_mut(&(new_row, col)).unwrap() = 'O';
                    }
                }
            }
        }
    }

    fn total_load(&self) -> u32 {
        self.platform
            .iter()
            .filter(|(_, &v)| v == 'O')
            .map(|(k, _)| self.n - k.0)
            .sum::<u32>()
    }

    fn rotate(&mut self) {
        let rotated: FxHashMap<(u32, u32), char> = self
            .platform
            .clone()
            .into_iter()
            .map(|(k, v)| ((k.1, self.n - 1 - k.0), v))
            .collect();
        self.platform = rotated;
    }

    fn cycle(&mut self) {
        for _ in 0..4 {
            self.tilt();
            self.rotate();
        }
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut repr = vec![];

        for i in 0..self.n {
            repr.extend((0..self.n).map(move |j| *self.platform.get(&(i, j)).unwrap()));
            repr.push('\n');
        }

        write!(f, "{}", repr.iter().collect::<String>())
    }
}

impl Hash for Platform {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(self.to_string().as_bytes());
        let _ = state.finish();
    }
}

fn solve_cycle(platform: &mut Platform, n: u32) -> Option<u32> {
    let mut cache: FxHashMap<Platform, u32> = FxHashMap::default();

    for i in 1..n {
        platform.cycle();

        if let Some(v) = cache.insert(platform.clone(), i) {
            let y = (n - v) % (i - v) + v;
            return cache
                .iter()
                .find(|(_, &x)| x == y)
                .map(|(p, _)| p.total_load());
        }
    }
    None
}

#[aoc(2023, 14)]
pub fn main() {
    let data = aoc_input!(2023, 14).unwrap();
    let parsed: Platform = data.parse().unwrap();

    // Part I
    let mut platform = parsed.clone();
    platform.tilt();
    println!("{}", platform.total_load());

    // Part II
    let n = solve_cycle(&mut platform, 1_000_000_000).unwrap();
    println!("{n}");
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test_part1() {
        let mut platform = Platform::from_str(EXAMPLE).unwrap();
        platform.tilt();
        assert_eq!(platform.total_load(), 136);
    }

    #[test]
    fn test_part2() {
        let mut platform = Platform::from_str(EXAMPLE).unwrap();
        assert_eq!(solve_cycle(&mut platform, 1_000_000_000), Some(64));
    }
}
