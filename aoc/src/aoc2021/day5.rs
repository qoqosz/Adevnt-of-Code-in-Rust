use aoc::{aoc, aoc_input};
use rustc_hash::FxHashMap;
use std::num::ParseIntError;

struct Line {
    a: (i32, i32),
    b: (i32, i32),
}

impl TryFrom<&str> for Line {
    type Error = ParseIntError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (lhs, rhs) = value.split_once(" -> ").unwrap();
        let parse = |x: &str| -> Result<(i32, i32), ParseIntError> {
            let (a, b) = x.split_once(',').unwrap();
            Ok((a.parse::<i32>()?, b.parse::<i32>()?))
        };

        Ok(Self {
            a: parse(lhs)?,
            b: parse(rhs)?,
        })
    }
}

impl Line {
    fn dx(&self) -> i32 {
        (self.b.0 - self.a.0).signum()
    }

    fn dy(&self) -> i32 {
        (self.b.1 - self.a.1).signum()
    }

    fn is_diagonal(&self) -> bool {
        self.dx().abs() == self.dy().abs()
    }

    fn points(&self) -> impl Iterator<Item = (i32, i32)> {
        let (px, py) = self.a;
        let (dx, dy) = (self.dx(), self.dy());
        let t = std::cmp::max((self.a.0 - self.b.0).abs(), (self.a.1 - self.b.1).abs());

        (0..=t).map(move |i| (px + i * dx, py + i * dy))
    }
}

fn count_overlap<'a>(lines: impl IntoIterator<Item = &'a Line>) -> usize {
    let mut seen = FxHashMap::default();

    for line in lines {
        for p in line.points() {
            seen.entry(p).and_modify(|v| *v += 1).or_insert(1);
        }
    }

    seen.values().filter(|v| **v > 1).count()
}

#[aoc(2021, 5)]
pub fn main() {
    let data = aoc_input!(2021, 5).unwrap();
    let lines = data
        .trim()
        .lines()
        .flat_map(Line::try_from)
        .collect::<Vec<_>>();

    // Part I
    let cnt1 = count_overlap(lines.iter().filter(|line| !line.is_diagonal()));
    println!("{cnt1}");

    // Part II
    let cnt2 = count_overlap(&lines);
    println!("{cnt2}");
}
