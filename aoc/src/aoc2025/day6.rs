use aoc::utils::transpose;
use aoc::{aoc, aoc_input};

/// Sum or multiply numbers in a group.
fn calculate<'a, N, S>(group: N, op: S) -> u64
where
    N: IntoIterator<Item = &'a u64>,
    S: AsRef<str>,
{
    match op.as_ref() {
        "+" => group.into_iter().sum::<u64>(),
        "*" => group.into_iter().product::<u64>(),
        _ => unreachable!(),
    }
}

/// Calculate the grand total, i.e. solution to the problem.
fn grand_total(numbers: &[Vec<u64>], ops: &[&str]) -> u64 {
    numbers
        .iter()
        .zip(ops.iter())
        .map(|(group, op)| calculate(group, op))
        .sum()
}

/// Parse the input for part I.
fn parse(data: &str) -> (Vec<Vec<u64>>, Vec<&str>) {
    let mut lines = data.trim().lines().collect::<Vec<_>>();
    let ops = lines.pop().unwrap().split_whitespace().collect::<Vec<_>>();
    let numbers = lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .flat_map(|n| n.parse::<u64>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (transpose(numbers), ops)
}

/// Parse the input for part II.
fn parse2(data: &str) -> (Vec<Vec<u64>>, Vec<&str>) {
    let mut lines = data.trim().lines().collect::<Vec<_>>();
    let ops = lines.pop().unwrap().split_whitespace().collect::<Vec<_>>();
    let (mut numbers, mut tmp, mut i) = (vec![], vec![], 0);

    loop {
        let col = lines
            .iter()
            .flat_map(|line| line.chars().nth(i))
            .collect::<String>();

        if col.is_empty() {
            break;
        }

        match col.trim().parse::<u64>() {
            Ok(num) => tmp.push(num),
            _ => {
                numbers.push(tmp.clone());
                tmp.clear();
            }
        }

        i += 1;
    }
    numbers.push(tmp);

    (numbers, ops)
}

#[aoc(2025, 6)]
pub fn main() {
    let data = aoc_input!(2025, 6).unwrap();

    // Part I
    let (numbers, ops) = parse(&data);
    println!("{}", grand_total(&numbers, &ops));

    // Part II
    let (numbers, ops) = parse2(&data);
    println!("{}", grand_total(&numbers, &ops));
}
