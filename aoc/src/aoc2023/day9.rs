use aoc::{aoc, aoc_input};

fn parse(data: &str) -> Vec<Vec<i64>> {
    data.lines()
        .filter(|x| !x.is_empty())
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

fn extrapolate(seq: &Vec<i64>) -> i64 {
    let mut diffs = seq.to_owned();
    let mut res = *diffs.last().unwrap();

    // Calc diffs
    while diffs.iter().any(|x| *x != 0) {
        diffs = diffs.windows(2).map(|w| w[1] - w[0]).collect();
        res += diffs.last().unwrap();
    }

    res
}

#[aoc(2023, 9)]
pub fn main() {
    let data = aoc_input!(2023, 9).unwrap();
    let seqs = parse(&data);

    // Part I
    let res: i64 = seqs.iter().map(extrapolate).sum();
    println!("{res}");

    // Part II
    let res: i64 = seqs
        .iter()
        .map(|seq| extrapolate(&seq.iter().rev().copied().collect()))
        .sum();
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_part1() {
        let seqs = parse(EXAMPLE);
        let res: i64 = seqs.iter().map(extrapolate).sum();
        assert_eq!(res, 114);
    }

    #[test]
    fn test_part2() {
        let seqs = parse(EXAMPLE);
        let res: i64 = seqs
            .iter()
            .map(|seq| extrapolate(&seq.iter().rev().copied().collect()))
            .sum();
        assert_eq!(res, 2);
    }
}
