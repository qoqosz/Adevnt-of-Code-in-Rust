use aoc::{aoc, aoc_input};
use itertools::Itertools;

fn is_triangle(a: u32, b: u32, c: u32) -> bool {
    (a + b) > c && (a + c) > b && (b + c) > a
}

fn parse(data: &str) -> Vec<(u32, u32, u32)> {
    data.lines()
        .filter(|x| !x.is_empty())
        .map(|l| {
            l.split_whitespace()
                .filter_map(|x| x.parse::<u32>().ok())
                .collect_tuple::<(u32, u32, u32)>()
                .unwrap()
        })
        .collect::<_>()
}

#[aoc(2016, 3)]
pub fn main() {
    let data = aoc_input!(2016, 3).unwrap();
    let nums = parse(&data);

    // Part I
    let n = nums.iter().filter(|t| is_triangle(t.0, t.1, t.2)).count();
    println!("{n}");

    // Part II
    let n = nums
        .chunks(3)
        .map(|w| {
            is_triangle(w[0].0, w[1].0, w[2].0) as usize
                + is_triangle(w[0].1, w[1].1, w[2].1) as usize
                + is_triangle(w[0].2, w[1].2, w[2].2) as usize
        })
        .sum::<usize>();
    println!("{n}");
}
