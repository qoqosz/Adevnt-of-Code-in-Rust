use aoc::{aoc, aoc_input};
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use itertools::MinMaxResult::MinMax;

fn find_invalid(nums: &[i64], preamble: usize) -> Option<i64> {
    nums.windows(preamble + 1).find_map(|win| {
        let (prevs, cur) = win.split_at(preamble);
        if prevs
            .to_vec()
            .iter()
            .combinations(2)
            .any(|x| x[0] + x[1] == cur[0])
        {
            None
        } else {
            Some(cur[0])
        }
    })
}

fn find_subset(nums: &[i64], target: i64) -> Option<&[i64]> {
    for i in 0..nums.len() {
        let (j, sum) = nums[i..]
            .iter()
            .enumerate()
            .fold_while((0, 0), |(_, acc), (j, x)| {
                if acc < target {
                    Continue((j, acc + x))
                } else {
                    Done((j, acc))
                }
            })
            .into_inner();

        if sum == target {
            return Some(&nums[i..i + j]);
        }
    }
    None
}

#[aoc(2020, 9)]
pub fn main() {
    let data = aoc_input!(2020, 9).unwrap();
    let nums = data
        .lines()
        .filter_map(|x| x.parse::<i64>().ok())
        .collect::<Vec<_>>();

    // Part I
    let n = find_invalid(&nums, 25).unwrap();
    println!("{}", n);

    // Part II
    if let MinMax(a, b) = find_subset(&nums, n).unwrap().iter().minmax() {
        println!("{}", a + b);
    }
}
