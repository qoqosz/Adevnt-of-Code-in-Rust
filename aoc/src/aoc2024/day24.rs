use aoc::{aoc, aoc_input};
use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::collections::VecDeque;

struct Device {
    x: u64,
    y: u64,
    z: u64,
}

fn parse(data: &str) -> (FxHashMap<&str, bool>, Vec<(&str, &str, &str, &str)>) {
    let (inits, instructions) = data.trim().split_once("\n\n").unwrap();

    let inits = inits
        .lines()
        .map(|line| {
            let (var, val) = line.split_once(": ").unwrap();
            (var, val.parse::<i64>().unwrap() != 0)
        })
        .collect();

    let instructions = instructions
        .lines()
        .map(|line| {
            let (op, out) = line.split_once(" -> ").unwrap();
            let op = op.split_whitespace().collect::<Vec<_>>();
            (op[0], op[1], op[2], out)
        })
        .collect();

    (inits, instructions)
}

fn read(memory: &FxHashMap<&str, bool>, ch: char) -> usize {
    let output = memory
        .iter()
        .filter(|(k, _)| k.starts_with(ch))
        .sorted_unstable()
        .rev()
        .map(|(_, v)| if *v { '1' } else { '0' })
        .collect::<String>();
    usize::from_str_radix(&output, 2).unwrap()
}

#[aoc(2024, 24)]
pub fn main() {
    let data = aoc_input!(2024, 24).unwrap();
    let (inits, instructions) = parse(&data);

    // Part I
    let mut memory = inits.clone();
    let mut queue = VecDeque::from(instructions.clone());

    while let Some((x, op, y, out)) = queue.pop_front() {
        if let (Some(&left), Some(&right)) = (memory.get(x), memory.get(y)) {
            let res = match op {
                "AND" => left & right,
                "OR" => left | right,
                "XOR" => left ^ right,
                _ => unreachable!(),
            };
            memory.insert(out, res);
        } else {
            queue.push_back((x, op, y, out));
            continue;
        }
    }

    println!("{}", read(&memory, 'z'));

    // Part II
    let x = 1;
    println!("{x:044b}");
    println!("x{x:02}");
}
