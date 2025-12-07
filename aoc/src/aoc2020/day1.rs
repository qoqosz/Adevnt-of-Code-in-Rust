use aoc::aoc_input;
use itertools::Itertools;

fn search(nums: &[i32], total: i32, n: usize) -> Option<i32> {
    nums.iter()
        .combinations(n)
        .find(|v| v.iter().map(|x| **x).sum::<i32>() == total)
        .map(|v| v.iter().map(|x| **x).product())
}

pub fn main() {
    let data = aoc_input!(2020, 1).unwrap();
    let nums = data.lines().flat_map(|x| x.parse::<i32>()).collect_vec();

    // Part I
    println!("{}", search(&nums, 2020, 2).unwrap());

    // Part II
    println!("{}", search(&nums, 2020, 3).unwrap());
}
