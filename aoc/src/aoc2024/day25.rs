use aoc::{aoc, aoc_input};
use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Schematics {
    Lock([u8; 5]),
    Key([u8; 5]),
}

impl FromStr for Schematics {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line = s.lines().next().clone().unwrap();
        let mut schema = [0, 0, 0, 0, 0];
        let is_lock = line.chars().all(|ch| ch == '#');

        for line in s.lines() {
            for (i, c) in line.chars().enumerate() {
                if c == '#' {
                    schema[i] += 1;
                }
            }
        }

        if is_lock {
            Ok(Self::Lock(schema))
        } else {
            Ok(Self::Key(schema))
        }
    }
}

impl Schematics {
    fn inner(&self) -> &[u8; 5] {
        match self {
            Schematics::Lock(val) => val,
            Schematics::Key(val) => val,
        }
    }

    fn is_fit(&self, other: &Self) -> bool {
        self.inner()
            .iter()
            .zip(other.inner().iter())
            .all(|(x, y)| *x + *y <= 7)
    }
}

fn parse(data: &str) -> (Vec<Schematics>, Vec<Schematics>) {
    let mut keys = vec![];
    let mut locks: Vec<_> = data
        .trim()
        .split("\n\n")
        .flat_map(|line| Schematics::from_str(line))
        .collect();

    locks.retain(|x| match x {
        Schematics::Lock(_) => true,
        Schematics::Key(_) => {
            keys.push(x.clone());
            false
        }
    });

    (locks, keys)
}

#[aoc(2024, 25)]
pub fn main() {
    let data = aoc_input!(2024, 25).unwrap();
    let (locks, keys) = parse(&data);

    let n_pairs = locks
        .iter()
        .cartesian_product(keys.iter())
        .filter(|(l, k)| l.is_fit(k))
        .count();
    println!("{n_pairs}");
}
