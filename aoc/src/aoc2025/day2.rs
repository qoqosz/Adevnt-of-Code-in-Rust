use aoc::{aoc, aoc_input};
use itertools::Itertools;

fn parse(value: &str) -> std::ops::RangeInclusive<u64> {
    let (x, y) = value.split_once('-').unwrap();
    let first = x.parse().unwrap();
    let last = y.parse().unwrap();

    first..=last
}

// Part I
fn is_invalid(num: u64) -> bool {
    let num = num.to_string();
    let n = num.len();

    match n % 2 {
        0 => &num[..n / 2] == &num[n / 2..],
        _ => false,
    }
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
    let ranges = data.trim().split(',').map(parse);

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
