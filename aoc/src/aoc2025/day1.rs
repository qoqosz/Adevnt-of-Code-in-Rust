use std::num::ParseIntError;

use aoc::{aoc, aoc_input};

#[derive(Debug)]
enum Rotation {
    Left(i16),
    Right(i16),
}

impl TryFrom<&str> for Rotation {
    type Error = ParseIntError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim().split_at(1) {
            ("L", dist) => Ok(Self::Left(dist.parse()?)),
            ("R", dist) => Ok(Self::Right(dist.parse()?)),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Dial {
    // position of the dial
    point: i16,
}

impl Dial {
    fn new() -> Self {
        Self { point: 50 }
    }

    /// Rotate the dial.
    ///
    /// # Returns
    ///
    /// - `i16` new dial's position.
    /// - `i16` number of times the dial points at `0`.
    fn rotate(&mut self, rotation: &Rotation) -> (i16, i16) {
        let zeros = match rotation {
            Rotation::Left(left) => {
                let val = (100 - self.point).rem_euclid(100) + left;
                self.point = (100 - val).rem_euclid(100);
                val / 100
            }
            Rotation::Right(right) => {
                let val = self.point + right;
                self.point = val.rem_euclid(100);
                val / 100
            }
        };

        (self.point, zeros)
    }
}

#[aoc(2025, 1)]
pub fn main() {
    let data = aoc_input!(2025, 1).unwrap();
    let rotations = data
        .lines()
        .flat_map(Rotation::try_from)
        .collect::<Vec<_>>();

    // Part I
    let mut dial = Dial::new();
    let password = rotations
        .iter()
        .map(|r| dial.rotate(r).0)
        .filter(|p| *p == 0)
        .count();
    println!("{password}");

    // Part II
    let mut dial = Dial::new();
    let zeros = rotations.iter().map(|r| dial.rotate(r).1).sum::<i16>();
    println!("{zeros}");
}
