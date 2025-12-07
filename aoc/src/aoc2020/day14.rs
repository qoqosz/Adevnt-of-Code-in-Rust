use aoc::{aoc, aoc_input};
use std::str::FromStr;

struct BitMask {
    ones: u64,
    zeros: u64,
}

impl FromStr for BitMask {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let ones: u64 = value
            .chars()
            .rev()
            .enumerate()
            .filter(|(_, ch)| *ch == '1')
            .map(|(i, _)| 1 << i)
            .sum();
        let zeros: u64 = value
            .chars()
            .rev()
            .enumerate()
            .filter(|(_, ch)| *ch == '0')
            .map(|(i, _)| 1 << i)
            .sum();

        println!("{ones:#036b}, {zeros:#036b}");

        Ok(Self { ones, zeros })
    }
}

impl BitMask {
    fn apply(&self, other: u64) -> u64 {
        let tmp = other | self.ones;
        !(tmp ^ !self.zeros)
    }
}

#[aoc(2020, 14)]
pub fn main() {
    let data = aoc_input!(2020, 14).unwrap();
    let mask = BitMask::from_str("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").unwrap();
    println!("{} {:#036b}", mask.apply(11), mask.apply(11));
    println!("{} {:#036b} {:#036b}", 11, 11, (!11));
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "939
7,13,x,x,59,x,31,19";

    #[test]
    fn test_part1() {}
}
