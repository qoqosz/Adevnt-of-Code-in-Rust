use aoc::{aoc, aoc_input};
use itertools::Itertools;

type Stack = Vec<char>;
type Step = (usize, usize, usize);

fn parse(data: &str) -> (Vec<Stack>, Vec<Step>) {
    let (stacks, procedure) = data.trim_end().split_once("\n\n").unwrap();
    // assume that there are only 9 stacks
    let mut crates = (0..9).map(|_| Vec::with_capacity(100)).collect::<Vec<_>>();

    for line in stacks.lines().rev().skip(1) {
        for (i, ch) in line.chars().enumerate() {
            if i.saturating_sub(1) % 4 == 0 && ch.is_ascii_uppercase() {
                crates[i.saturating_sub(1) / 4].push(ch)
            }
        }
    }

    let instructions = procedure
        .lines()
        .flat_map(|line| {
            line.split(' ')
                .enumerate()
                .filter(|(i, _)| i % 2 == 1)
                .flat_map(|(_, w)| w.parse::<usize>())
                .collect_tuple::<(_, _, _)>()
        })
        .map(|(x, y, z)| (x, y.saturating_sub(1), z.saturating_sub(1)))
        .collect();

    (crates, instructions)
}

fn rearrange(crates: &mut [Stack], instruction: &Step) {
    let (qty, src, dest) = *instruction;

    for _ in 0..qty {
        let item = crates[src].pop().unwrap();
        crates[dest].push(item);
    }
}

fn rearrange_multiple(crates: &mut [Stack], instruction: &Step) {
    let (qty, src, dest) = *instruction;
    let n = crates[src].len();
    let removed = crates[src].drain((n - qty)..).collect::<Vec<_>>();
    crates[dest].extend(removed);
}

#[aoc(2022, 5)]
pub fn main() {
    let data = aoc_input!(2022, 5).unwrap();
    let (mut crates, instructions) = parse(&data);

    // Part I
    let mut crates1 = crates.clone();
    for instruction in &instructions {
        rearrange(&mut crates1, instruction);
    }
    println!(
        "{}",
        crates1
            .iter()
            .map(|c| c.last().unwrap())
            .collect::<String>()
    );

    // Part II
    for instruction in &instructions {
        rearrange_multiple(&mut crates, instruction);
    }
    println!(
        "{}",
        crates.iter().map(|c| c.last().unwrap()).collect::<String>()
    );
}
