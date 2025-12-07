use aoc::{aoc, aoc_input};
use itertools::{Itertools, MultiProduct};
use rustc_hash::FxHashMap;
use std::{num::ParseIntError, str::FromStr};

pub trait ProductRepeat: Iterator + Clone
where
    Self::Item: Clone,
{
    fn product_repeat(self, repeat: usize) -> MultiProduct<Self> {
        std::iter::repeat(self)
            .take(repeat)
            .multi_cartesian_product()
    }
}

impl<T: Iterator + Clone> ProductRepeat for T where T::Item: Clone {}

#[derive(Debug)]
struct Program {
    value: u64,
    address: u64,
}

#[derive(Debug, Default)]
struct BitMask {
    ones: u64,
    zeros: u64,
}

impl FromStr for BitMask {
    type Err = ParseIntError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let zeros = value.replace("X", "0");
        let zeros = u64::from_str_radix(&zeros, 2)?;
        let ones = value.replace("X", "1");
        let ones = u64::from_str_radix(&ones, 2)?;

        Ok(Self { ones, zeros })
    }
}

impl BitMask {
    #[inline(always)]
    fn apply(&self, other: u64) -> u64 {
        (other | self.zeros) & self.ones
    }
}

#[derive(Debug)]
enum InputLine {
    MASK(BitMask),
    PROG(Program),
}

impl FromStr for InputLine {
    type Err = ParseIntError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        if line.starts_with("mask") {
            let (_, bit_mask) = line.split_once(" = ").unwrap();
            Ok(InputLine::MASK(bit_mask.parse().unwrap()))
        } else {
            let (addr, val) = line.split_once(" = ").unwrap();
            let addr: u64 = addr
                .trim_start_matches("mem[")
                .trim_end_matches("]")
                .parse()?;
            let val = val.parse()?;
            Ok(InputLine::PROG(Program {
                value: val,
                address: addr,
            }))
        }
    }
}

#[aoc(2020, 14)]
pub fn main() {
    let data = aoc_input!(2020, 14).unwrap();
    let instructions = data
        .trim()
        .lines()
        .flat_map(InputLine::from_str)
        .collect::<Vec<_>>();

    // Part I
    let mut mask = &BitMask::default();
    let mut memory = FxHashMap::default();

    for instruction in &instructions {
        match instruction {
            InputLine::MASK(m) => mask = m,
            InputLine::PROG(prog) => {
                let val = mask.apply(prog.value);
                memory.insert(prog.address, val);
            }
        }
    }

    println!("{}", memory.values().sum::<u64>());

    // Part II
    let mut mask = &BitMask::default();
    let mut memory = FxHashMap::default();
    let mut x_mask = 0;

    for instruction in &instructions {
        match instruction {
            InputLine::MASK(m) => {
                mask = m;
                x_mask = m.ones ^ m.zeros;
            }
            InputLine::PROG(prog) => {
                let mut addr0 = prog.address | mask.zeros;
                let mut x_mask = x_mask;
                let mut addresses = vec![];

                for i in 0..36 {
                    let a = addr0 % 2;
                    let x = x_mask % 2;

                    addresses = if x == 1 {
                        if addresses.is_empty() {
                            vec![0, 1]
                        } else {
                            addresses
                                .iter()
                                .flat_map(|addr| vec![addr + (1 << i), *addr])
                                .collect()
                        }
                    } else if addresses.is_empty() {
                        vec![a]
                    } else {
                        addresses.iter().map(|addr| addr + a * (1 << i)).collect()
                    };

                    addr0 >>= 1;
                    x_mask >>= 1;
                }

                for addr in addresses {
                    memory.insert(addr, prog.value);
                }
            }
        }
    }

    println!("{}", memory.values().sum::<u64>());
}
