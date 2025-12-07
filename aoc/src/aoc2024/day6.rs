use aoc::{aoc, aoc_input};
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::str::FromStr;

#[derive(PartialEq, Eq)]
enum GuardState {
    Loop,
    Finished,
    Continue,
}

#[derive(Clone)]
struct GuardMap {
    grid: FxHashMap<(i32, i32), char>,
    pos: (i32, i32),
    dir: (i32, i32),
    visited: FxHashSet<((i32, i32), (i32, i32))>,
}

impl FromStr for GuardMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: FxHashMap<(i32, i32), char> = s
            .trim()
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, ch)| ((x as i32, y as i32), ch))
            })
            .collect();
        let pos = grid
            .iter()
            .find_map(|(key, &val)| if val == '^' { Some(*key) } else { None })
            .unwrap();
        let dir = (0, -1);
        let visited = FxHashSet::from_iter([(pos, dir)]);

        Ok(Self {
            grid,
            pos,
            dir,
            visited,
        })
    }
}

impl GuardMap {
    fn next(&self) -> (i32, i32) {
        (self.pos.0 + self.dir.0, self.pos.1 + self.dir.1)
    }

    fn rotate_right(&mut self) {
        self.dir = (-self.dir.1, self.dir.0)
    }

    fn advance(&mut self) -> GuardState {
        let next_pos = self.next();

        if let Some(next_ch) = self.grid.get(&next_pos) {
            match next_ch {
                '#' => self.rotate_right(),
                '.' | '^' => {
                    self.pos = next_pos;
                    if !self.visited.insert((self.pos, self.dir)) {
                        return GuardState::Loop;
                    };
                }
                _ => unreachable!(),
            }
            return GuardState::Continue;
        }
        return GuardState::Finished;
    }

    fn pos_count(&mut self) -> usize {
        while self.advance() == GuardState::Continue {}
        self.visited.iter().map(|(k, _)| *k).unique().count()
    }

    fn is_loop(&mut self) -> bool {
        loop {
            match self.advance() {
                GuardState::Loop => return true,
                GuardState::Finished => return false,
                _ => continue,
            }
        }
    }
}

// Brute force
fn find_loops(map: &GuardMap) -> usize {
    map.grid
        .iter()
        .filter(|(_, ch)| **ch != '#')
        .filter_map(|(pos, _)| {
            let mut new_map = map.clone();
            new_map.grid.insert(*pos, '#');
            new_map.is_loop().then(|| ())
        })
        .count()
}

#[aoc(2024, 6)]
pub fn main() {
    let data = aoc_input!(2024, 6).unwrap();

    // Part I
    let mut map = GuardMap::from_str(&data).unwrap();
    println!("{}", map.pos_count());

    // Part II
    let map = GuardMap::from_str(&data).unwrap();
    println!("{}", find_loops(&map));
}
