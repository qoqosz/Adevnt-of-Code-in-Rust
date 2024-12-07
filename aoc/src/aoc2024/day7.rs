use aoc::{aoc, aoc_input};
use std::str::FromStr;

struct Equation {
    value: u64,
    numbers: Vec<u64>,
}

impl FromStr for Equation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (value, numbers) = s.split_once(": ").unwrap();
        let value = value.parse::<u64>().unwrap();
        let numbers = numbers
            .split_whitespace()
            .flat_map(|x| x.parse::<u64>())
            .collect::<Vec<_>>();
        Ok(Equation { value, numbers })
    }
}

impl Equation {
    fn is_true1(&self) -> bool {
        check1(self.value, self.numbers[0], &self.numbers[1..])
    }

    fn is_true2(&self) -> bool {
        check2(self.value, self.numbers[0], &self.numbers[1..])
    }
}
fn check1(value: u64, left: u64, rem: &[u64]) -> bool {
    let sum = left + rem[0];
    let prod = left * rem[0];

    match rem.len() {
        1 => sum == value || prod == value,
        _ => check1(value, sum, &rem[1..]) || check1(value, prod, &rem[1..]),
    }
}

fn check2(value: u64, left: u64, rem: &[u64]) -> bool {
    let sum = left + rem[0];
    let prod = left * rem[0];
    let concat = concat(left, rem[0]);

    match rem.len() {
        1 => sum == value || prod == value || concat == value,
        _ => {
            check2(value, sum, &rem[1..])
                || check2(value, prod, &rem[1..])
                || check2(value, concat, &rem[1..])
        }
    }
}

fn concat(a: u64, b: u64) -> u64 {
    a * 10u64.pow(b.ilog10() + 1) + b
}

fn parse(data: &str) -> Vec<Equation> {
    data.trim()
        .lines()
        .flat_map(|line| Equation::from_str(line))
        .collect()
}

#[aoc(2024, 7)]
pub fn main() {
    let data = aoc_input!(2024, 7).unwrap();
    let equations = parse(&data);

    // Part I
    let total_calibration = equations
        .iter()
        .filter(|eq| eq.is_true1())
        .map(|eq| eq.value)
        .sum::<u64>();
    println!("{total_calibration}");

    // Part II
    let total_calibration = equations
        .iter()
        .filter(|eq| eq.is_true2())
        .map(|eq| eq.value)
        .sum::<u64>();
    println!("{total_calibration}");
}
