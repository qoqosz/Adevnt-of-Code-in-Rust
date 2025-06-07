use aoc::{aoc, aoc_input};
use std::{num::ParseIntError, str::FromStr};

#[derive(Clone)]
struct BinaryNumber {
    size: u8,
    data: u16,
}

impl FromStr for BinaryNumber {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        u16::from_str_radix(s, 2).map(|num| Self {
            size: s.len() as u8,
            data: num,
        })
    }
}

impl BinaryNumber {
    fn nth_bit(&self, n: u8) -> Option<bool> {
        if 1 <= n && n <= self.size {
            Some(self.data & (1 << (self.size - n)) != 0)
        } else {
            None
        }
    }
}

#[derive(Clone)]
struct Report(Vec<BinaryNumber>);

impl Report {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn size(&self) -> Option<u8> {
        self.0.get(0).map(|x| x.size)
    }
}

impl FromStr for Report {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .filter_map(|line| BinaryNumber::from_str(line).ok())
                .collect::<Vec<_>>(),
        ))
    }
}

fn most_common_bit(report: &Report, i: u8) -> bool {
    let n = report.len();
    let n_true = report
        .0
        .iter()
        .filter(|row| row.nth_bit(i).unwrap_or(false))
        .count();

    if 2 * n_true == n {
        true
    } else {
        2 * n_true > n
    }
}

fn gamma_rate(report: &Report) -> usize {
    let size = report.size().unwrap().into();
    let bits = (1..=size)
        .map(|i| {
            if most_common_bit(report, i as u8) {
                '1'
            } else {
                '0'
            }
        })
        .collect::<String>();
    BinaryNumber::from_str(&bits)
        .map(|num| num.data as usize)
        .unwrap()
}

fn power_consumption(report: &Report) -> usize {
    let len: usize = report.size().unwrap().into();
    let gamma = gamma_rate(report);
    let epsilon = (!gamma) % (1 << len);

    gamma * epsilon
}

fn filter(report: &Report, i: u8, by: bool) -> Report {
    Report(
        report
            .0
            .iter()
            .filter(|row| row.nth_bit(i).unwrap() == by)
            .cloned()
            .collect(),
    )
}

fn rating(report: &Report, cond: impl Fn(&Report, u8) -> bool) -> Option<usize> {
    let mut report = report.clone();
    let n = report.size().unwrap();

    for i in 1..=n {
        let by = cond(&report, i);
        report = filter(&report, i, by);

        if report.len() == 1 {
            break;
        }
    }

    match report.0.get(0) {
        Some(num) => Some(num.data.into()),
        _ => None,
    }
}

fn oxygen_generator_rating(report: &Report) -> Option<usize> {
    rating(report, |report, i| most_common_bit(report, i))
}

fn co2_scrubber_rating(report: &Report) -> Option<usize> {
    rating(report, |report, i| !most_common_bit(report, i))
}

#[aoc(2021, 3)]
pub fn main() {
    let data = aoc_input!(2021, 3).unwrap();
    let report = Report::from_str(&data).unwrap();

    // Part I
    println!("{}", power_consumption(&report));

    // Part II
    let o2 = oxygen_generator_rating(&report).unwrap();
    let co2 = co2_scrubber_rating(&report).unwrap();
    println!("{}", o2 * co2);
}
