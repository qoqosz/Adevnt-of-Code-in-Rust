use aoc::{aoc, aoc_input};
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use num::Integer;
use rustc_hash::FxHashMap;

type Network<'a> = FxHashMap<&'a str, (&'a str, &'a str)>;

fn parse(data: &str) -> (&str, Network<'_>) {
    let (instruction, mapping) = data.split_once("\n\n").unwrap();
    let network = mapping
        .lines()
        .map(|line| {
            let (key, val) = line.split_once(" = ").unwrap();
            (
                key,
                val.trim_start_matches('(')
                    .trim_end_matches(')')
                    .split_once(", ")
                    .unwrap(),
            )
        })
        .collect::<FxHashMap<_, _>>();

    (instruction, network)
}

fn travel(
    instruction: &str,
    network: &Network,
    start: &str,
    is_goal: impl Fn(&str) -> bool,
) -> usize {
    instruction
        .chars()
        .cycle()
        .fold_while((0, start), |(i, node), dir| {
            let next = match dir {
                'L' => network.get(&node).unwrap().0,
                'R' => network.get(&node).unwrap().1,
                _ => unreachable!(),
            };

            match is_goal(next) {
                true => Done((i + 1, next)),
                false => Continue((i + 1, next)),
            }
        })
        .into_inner()
        .0
}

fn travel_ghost(instruction: &str, network: &Network) -> Option<usize> {
    network
        .keys()
        .filter(|node| node.ends_with('A'))
        .map(|node| travel(instruction, network, node, |n| n.ends_with('Z')))
        .reduce(|a, b| a.lcm(&b))
}

#[aoc(2023, 8)]
pub fn main() {
    let data = aoc_input!(2023, 8).unwrap();
    let (instruction, network) = parse(&data);

    // Part I
    println!("{}", travel(instruction, &network, "AAA", |n| n == "ZZZ"));

    // Part II
    println!("{}", travel_ghost(instruction, &network).unwrap());
}
