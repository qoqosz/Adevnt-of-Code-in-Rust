use aoc::aoc_input;
use itertools::Itertools;

fn min_qe(packages: &Vec<i64>, n: i64) -> Option<i64> {
    let weight: i64 = packages.iter().sum::<i64>() / n;

    for i in 1..(packages.len() - 2) {
        let groups = packages
            .iter()
            .combinations(i)
            .filter(|comb| comb.iter().copied().sum::<i64>() == weight)
            .map(|comb| comb.into_iter().product())
            .collect::<Vec<i64>>();
        if !groups.is_empty() {
            return groups.into_iter().min();
        }
    }

    None
}

pub fn main() {
    let data = aoc_input!(2015, 24).unwrap();
    let packages = data
        .lines()
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    // Part I
    println!("{}", min_qe(&packages, 3).unwrap());

    // Part II
    println!("{}", min_qe(&packages, 4).unwrap());
}
