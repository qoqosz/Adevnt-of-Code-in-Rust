use aoc::{aoc, aoc_input};
use glam::IVec2 as Point;
use rustc_hash::{FxHashMap, FxHashSet};
use std::ops::Add;

macro_rules! point {
    ($x:expr, $y:expr) => {
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
        self + Point::from(other) // Into::<Self>::into(other)
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

fn eval_round(elves: &FxHashSet<Point>, n: usize) -> FxHashSet<Point> {
    // first half
    let mut elves_to_stay = FxHashSet::default();
    let mut elves_to_move = FxHashSet::default();

    for elf in elves {
        if elves.is_free(elves.neighbors(elf)) {
            elves_to_stay.insert(*elf);
        } else {
            elves_to_move.insert(*elf);
        }
    }

    // let mut moves = FxHashMap::default();

    for elf in elves_to_move {
        'inner: for (checks, mov) in CHECKS.iter().cycle().skip(n).take(4) {}
    }

    elves.clone()
}

#[aoc(2022, 23)]
pub fn main() {
    let data = aoc_input!(2022, 23).unwrap();

    // Part I
    let scan = parse(&data);
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
