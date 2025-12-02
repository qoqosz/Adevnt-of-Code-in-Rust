use aoc::{aoc, aoc_input};
use itertools::Itertools;
use std::num::ParseIntError;

#[derive(Debug)]
struct Range {
    first: u64,
    last: u64,
}

impl IntoIterator for Range {
    type Item = u64;
    type IntoIter = std::ops::RangeInclusive<u64>;

    fn into_iter(self) -> Self::IntoIter {
        self.first..=self.last
    }
}

impl TryFrom<&str> for Range {
    type Error = ParseIntError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (x, y) = value.split_once('-').unwrap();
        Ok(Self {
            first: x.parse()?,
            last: y.parse()?,
        })
    }
}

// Part I
fn is_invalid(num: u64) -> bool {
    let num = num.to_string();
    let n = num.len();

    if n % 2 != 0 {
        return false;
    }

    &num[..n / 2] == &num[n / 2..]
}

// Part II
fn is_invalid2(num: u64) -> bool {
    let num = num.to_string();
    let n = num.len();

    for i in 1..=n / 2 {
        if n % i != 0 {
            continue;
        }
        if num.as_bytes().chunks(i).all_equal() {
            return true;
        }
    }

    false
}

#[aoc(2025, 2)]
pub fn main() {
    let data = aoc_input!(2025, 2).unwrap();
    let ranges = data
        .split(',')
        .flat_map(Range::try_from)
        .collect::<Vec<_>>();

    let (mut n1, mut n2) = (0, 0);

    for r in ranges {
        for i in r {
            if is_invalid(i) {
                n1 += i;
            }
            if is_invalid2(i) {
                n2 += i;
            }
        }
    }

    // Part I
    println!("{n1}");

    // Part II
    println!("{n2}");
}
