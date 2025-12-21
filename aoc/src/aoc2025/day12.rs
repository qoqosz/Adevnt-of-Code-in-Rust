use aoc::{aoc, aoc_input};

fn parse(data: &str) -> (u32, u32, Vec<u32>) {
    let w = data[..2].parse::<u32>().unwrap();
    let h = data[3..5].parse::<u32>().unwrap();
    let boxes = data[7..]
        .split_whitespace()
        .flat_map(|x| x.parse())
        .collect::<Vec<_>>();
    (w, h, boxes)
}

#[aoc(2025, 12)]
pub fn main() {
    let data = aoc_input!(2025, 12).unwrap();
    let input = data.trim().lines().skip(30);

    // Part I
    let n = input
        .map(parse)
        .filter(|(w, h, boxes)| (w / 3) * (h / 3) >= boxes.iter().sum())
        .count();
    println!("{n}");
}
