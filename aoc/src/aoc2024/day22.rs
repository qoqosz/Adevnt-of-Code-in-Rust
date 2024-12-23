use aoc::{aoc, aoc_input};
use rustc_hash::FxHashMap;
use std::iter::successors;

#[inline(always)]
fn mix(secret: usize, other: usize) -> usize {
    secret ^ other
}

#[inline(always)]
fn prune(secret: usize) -> usize {
    secret % 16777216
}

fn next_secret(secret: usize) -> usize {
    // Step 1
    let mut secret = prune(mix(secret, secret * 64));
    // Step 2
    secret = prune(mix(secret, secret / 32));
    // Step 3
    prune(mix(secret, secret * 2048))
}

#[aoc(2024, 22)]
pub fn main() {
    let data = aoc_input!(2024, 22).unwrap();
    let secrets: Vec<usize> = data.lines().filter_map(|line| line.parse().ok()).collect();

    // Part I
    let buyers_secrets = secrets
        .iter()
        .map(|&secret| {
            successors(Some(secret), |x| Some(next_secret(*x)))
                .take(2001)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let sum_secrets = buyers_secrets
        .iter()
        .filter_map(|bs| bs.last())
        .sum::<usize>();
    println!("{sum_secrets}");

    // Part II
    let bs_maps = buyers_secrets
        .iter()
        .map(|bs| bs.iter().map(|x| (x % 10) as i64).collect::<Vec<_>>())
        .map(|bs| {
            let mut price_map = FxHashMap::default();

            for win in bs.windows(5) {
                let diff = (
                    win[1] - win[0],
                    win[2] - win[1],
                    win[3] - win[2],
                    win[4] - win[3],
                );
                price_map.entry(diff).or_insert((win[4] % 10) as i64);
            }

            price_map
        })
        .collect::<Vec<_>>();

    let mut res_map = FxHashMap::default();

    for bs_map in bs_maps {
        for (diff, price) in bs_map {
            *res_map.entry(diff).or_insert(0) += price;
        }
    }

    println!("{}", res_map.values().max().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mix() {
        assert_eq!(37, mix(42, 15));
    }

    #[test]
    fn test_prune() {
        assert_eq!(16113920, prune(100000000));
    }

    #[test]
    fn test_next_secret() {
        assert_eq!(15887950, next_secret(123));
    }
}
