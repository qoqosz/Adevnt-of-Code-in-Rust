use aoc::{aoc, aoc_input};
use itertools::Itertools;

fn parse(data: &str) -> Vec<Vec<i64>> {
    data.lines()
        .filter(|x| !x.is_empty())
        .map(|line| {
            line.split(' ')
                .flat_map(|x| x.parse::<i64>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn is_safe(report: &[i64]) -> bool {
    is_monotonic(report) && is_clamped(report, 1, 3)
}

fn differences(report: &[i64]) -> Vec<i64> {
    report
        .iter()
        .tuple_windows()
        .map(|(x, y)| *y - *x)
        .collect()
}

fn is_monotonic(report: &[i64]) -> bool {
    let diffs: Vec<_> = differences(report).iter().map(|x| x.cmp(&0)).collect();
    let first = diffs[0];
    diffs.iter().all(|&elem| elem == first)
}

fn is_clamped(report: &[i64], min: i64, max: i64) -> bool {
    differences(report)
        .iter()
        .map(|x| x.abs())
        .all(|elem| elem >= min && elem <= max)
}

fn is_safe_tol(report: &Vec<i64>) -> bool {
    if is_safe(report) {
        return true;
    }

    let n = report.len();

    for i in 0..n {
        let mut copy = report.clone();
        copy.remove(i);

        if is_safe(&copy) {
            return true;
        }
    }

    false
}

#[aoc(2024, 2)]
pub fn main() {
    let data = aoc_input!(2024, 2).unwrap();
    let reports = parse(&data);

    // Part I
    let n = reports.iter().filter(|report| is_safe(report)).count();
    println!("{n}");

    // Part II
    let n = reports.iter().filter(|report| is_safe_tol(report)).count();
    println!("{n}");
}
