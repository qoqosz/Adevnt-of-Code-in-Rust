use aoc::{aoc, aoc_input};
use glam::IVec2 as Point;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::ops::Add;

macro_rules! point {
    ($x:expr_2021, $y:expr_2021) => {
        Point::new($x, $y)
    };
}

#[derive(Debug, Copy, Clone)]
enum Dir {
    N,
    S,
    W,
    E,
    NW,
    NE,
    SW,
    SE,
}

impl Dir {
    const VALUES: [Self; 8] = [
        Self::N,
        Self::S,
        Self::W,
        Self::E,
        Self::NW,
        Self::NE,
        Self::SW,
        Self::SE,
    ];
}

static CHECKS: [([Dir; 3], Dir); 4] = [
    ([Dir::N, Dir::NE, Dir::NW], Dir::N),
    ([Dir::S, Dir::SE, Dir::SW], Dir::S),
    ([Dir::W, Dir::NW, Dir::SW], Dir::W),
    ([Dir::E, Dir::NE, Dir::SE], Dir::E),
];

impl From<Dir> for Point {
    fn from(item: Dir) -> Point {
        match item {
            Dir::N => point!(0, -1),
            Dir::S => point!(0, 1),
            Dir::W => point!(-1, 0),
            Dir::E => point!(1, 0),
            Dir::NW => Point::from(Dir::N) + Point::from(Dir::W),
            Dir::NE => Point::from(Dir::N) + Point::from(Dir::E),
            Dir::SW => Point::from(Dir::S) + Point::from(Dir::W),
            Dir::SE => Point::from(Dir::S) + Point::from(Dir::E),
        }
    }
}

impl Add<Dir> for Point {
    type Output = Self;

    fn add(self, other: Dir) -> Self::Output {
        self + Point::from(other)
    }
}

trait ElvesScan {
    type Item;

    fn neighbors(&self, item: &Self::Item) -> impl Iterator<Item = Self::Item>;
    fn is_free(&self, adjs: impl Iterator<Item = Self::Item>) -> bool;
}

impl ElvesScan for FxHashSet<Point> {
    type Item = Point;

    fn neighbors(&self, item: &Self::Item) -> impl Iterator<Item = Self::Item> {
        Dir::VALUES.iter().map(move |dir| *item + *dir)
    }

    fn is_free(&self, mut adjs: impl Iterator<Item = Self::Item>) -> bool {
        !adjs.any(|adj| self.contains(&adj))
    }
}

enum ElvesResult {
    NoMoves,
    Moves(FxHashSet<Point>),
}

fn parse(data: &str) -> FxHashSet<Point> {
    data.trim()
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars().enumerate().filter_map(move |(j, ch)| {
                if ch == '#' {
                    Some(point!(j as i32, i as i32))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn eval_round(elves: &FxHashSet<Point>, n: usize) -> ElvesResult {
    // first half
    let mut elves_to_move = FxHashSet::default();

    for elf in elves {
        if !elves.is_free(elves.neighbors(elf)) {
            elves_to_move.insert(*elf);
        }
    }

    // Look around
    let mut proposed_moves = FxHashMap::default();

    for elf in elves_to_move {
        for (checks, mov) in CHECKS.iter().cycle().skip(n).take(4) {
            let is_free = checks.iter().all(|dir| {
                let adj = elf + *dir;
                !elves.contains(&adj)
            });

            if is_free {
                proposed_moves.insert(elf, elf + *mov);
                break;
            }
        }
    }

    // 2nd half
    let values = proposed_moves.values().copied().collect::<Vec<_>>();
    proposed_moves.retain(|_, new_elf| values.iter().filter(|v| **v == *new_elf).count() == 1);

    if proposed_moves.len() == 0 {
        ElvesResult::NoMoves
    } else {
        let mut output = elves.clone();

        output.retain(|elf| !proposed_moves.contains_key(elf));
        output.extend(proposed_moves.values());

        ElvesResult::Moves(output)
    }
}

fn empty_ground(elves: &FxHashSet<Point>) -> usize {
    if let itertools::MinMaxResult::MinMax(x_min, x_max) = elves.iter().map(|p| p.x).minmax() {
        if let itertools::MinMaxResult::MinMax(y_min, y_max) = elves.iter().map(|p| p.y).minmax() {
            let w = x_max - x_min + 1;
            let h = y_max - y_min + 1;

            return (w as usize) * (h as usize) - elves.len();
        } else {
            unreachable!();
        }
    } else {
        unreachable!();
    }
}

#[aoc(2022, 23)]
pub fn main() {
    let data = aoc_input!(2022, 23).unwrap();

    // Part I
    let mut scan = parse(&data);

    for i in 0..10 {
        if let ElvesResult::Moves(new_scan) = eval_round(&scan, i) {
            scan = new_scan;
        };
    }

    println!("{}", empty_ground(&scan));

    // Part II
    let mut scan = parse(&data);
    let mut i = 1;

    loop {
        match eval_round(&scan, i - 1) {
            ElvesResult::NoMoves => break,
            ElvesResult::Moves(new_scan) => {
                scan = new_scan;
                i += 1;
            }
        }
    }

    println!("{}", i);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_8dirs() {
        assert_eq!(8, Dir::VALUES.len());
    }

    #[test]
    fn test_dir_to_point() {
        assert_eq!(Point::from(Dir::N), point!(0, -1));
        assert_eq!(Point::from(Dir::S), point!(0, 1));
        assert_eq!(Point::from(Dir::W), point!(-1, 0));
        assert_eq!(Point::from(Dir::E), point!(1, 0));
        assert_eq!(Point::from(Dir::NW), point!(-1, -1));
        assert_eq!(Point::from(Dir::NE), point!(1, -1));
        assert_eq!(Point::from(Dir::SW), point!(-1, 1));
        assert_eq!(Point::from(Dir::SE), point!(1, 1));
    }
}
