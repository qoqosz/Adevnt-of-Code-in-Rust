use aoc::aoc_input;
use regex::Regex;
use rustc_hash::FxHashMap;

type Bag<'a> = FxHashMap<&'a str, Vec<(usize, &'a str)>>;

fn contains(bag: &Bag, color: &str, target_color: &str) -> bool {
    if color.contains(target_color) {
        return true;
    }
    match bag.get(color) {
        Some(inside) => inside.iter().any(|(_, c)| contains(bag, c, target_color)),
        _ => false,
    }
}

fn capacity(bag: &Bag, color: &str) -> usize {
    match bag.get(color) {
        Some(inside) => inside.iter().map(|(n, c)| n + n * capacity(bag, c)).sum(),
        _ => 0,
    }
}

fn main() {
    let data = aoc_input!(2020, 7).unwrap();
    let target_color = "shiny gold";
    let re: Regex = Regex::new(r"(\d+) (.*?) bag").unwrap();
    let bag: Bag = data
        .lines()
        .filter_map(|line| {
            let (out_color, in_color) = line.trim().split_once(" bags contain ").unwrap();
            if in_color.starts_with("no other") {
                None
            } else {
                let colors = re
                    .captures_iter(line)
                    .map(|d| {
                        (
                            d.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                            d.get(2).unwrap().as_str(),
                        )
                    })
                    .collect::<Vec<_>>();
                Some((out_color, colors))
            }
        })
        .collect();

    // Part I
    let n = bag
        .keys()
        .filter(|c| contains(&bag, c, target_color))
        .count();
    println!("{}", n - 1);

    // Part II
    println!("{}", capacity(&bag, target_color));
}
