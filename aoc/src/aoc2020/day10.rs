use aoc::{aoc, aoc_input};
use itertools::Itertools;

fn fib(x: usize) -> usize {
    match x {
        0 | 1 => x + 1,
        _ => fib(x - 1) + fib(x - 2),
    }
}

#[aoc(2020, 10)]
pub fn main() {
    let data = aoc_input!(2020, 10).unwrap();
    let mut adapters = data
        .lines()
        .flat_map(|x| x.parse::<i32>())
        .collect::<Vec<_>>();
    adapters.push(0);
    adapters.push(adapters.iter().max().unwrap() + 3);
    adapters.sort();

    // Part I
    let diffs = adapters.windows(2).map(|x| x[1] - x[0]).collect::<Vec<_>>();
    let diff_counts = &diffs.iter().counts();
    println!("{}", diff_counts[&1] * diff_counts[&3]);

    // Part 2
    // Elements where diff in jolts is < 3 can be rearranged.
    // Number of this rearrangement is based only on a total jolts diff.
    // E.g.
    //   - diff 2 jolts yields: (1, 1), (2) - 2 arrangements
    //   - diff 3 jolts yields: (1, 1, 1), (1, 2), (2, 1), (3) - 4 arrangements, etc.
    let n = diffs.len();
    let shuffle_groups = &diffs[..n - 1]
        .iter()
        .chunk_by(|x| **x == 3)
        .into_iter()
        .filter_map(|(key, group)| if !key { Some(group.sum::<i32>()) } else { None })
        .collect::<Vec<_>>();

    println!(
        "{}",
        shuffle_groups
            .iter()
            .map(|x| fib(*x as usize) - 1)
            .product::<usize>()
    );
}
