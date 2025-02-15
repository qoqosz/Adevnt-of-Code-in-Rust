use aoc::{aoc, aoc_input};
use itertools::Itertools;
use num::Integer;
use rustc_hash::FxHashMap;
use std::sync::LazyLock;

static DIGITS: LazyLock<FxHashMap<char, i64>> =
    LazyLock::new(|| FxHashMap::from_iter([('0', 0), ('1', 1), ('2', 2), ('-', -1), ('=', -2)]));
static INV_DIGITS: LazyLock<FxHashMap<i64, char>> =
    LazyLock::new(|| DIGITS.iter().map(|(k, v)| (*v, *k)).collect());

fn snafu2num(s: &str) -> i64 {
    s.chars()
        .rev()
        .enumerate()
        .map(|(i, ch)| DIGITS[&ch] * 5_i64.pow(i as u32))
        .sum()
}

fn num2snafu(n: i64) -> String {
    let mut out = vec![];
    let mut x = n;

    while x != 0 {
        let (n, rem) = x.div_rem(&5);
        x = n;

        if rem <= 2 {
            out.push(rem);
        } else {
            out.push(rem - 5);
            x += 1;
        }
    }

    out.iter().rev().map(|x| INV_DIGITS[x]).join("")
}

#[aoc(2022, 25)]
pub fn main() {
    let data = aoc_input!(2022, 25).unwrap();

    // Part I
    let n = data.lines().map(snafu2num).sum::<i64>();
    println!("{}", num2snafu(n));
}
