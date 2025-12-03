use aoc::{aoc, aoc_input};

fn max_joltage(bank: &str, sz: usize) -> usize {
    let n = bank.len();
    let mut out = Vec::with_capacity(sz);
    let mut idx = 0;

    for i in 1..=sz {
        let (jdx, ch) =
            bank[idx..n - sz + i]
                .as_bytes()
                .iter()
                .enumerate()
                .fold(
                    (0, 0),
                    |prev, (j, &d)| if d > prev.1 { (j, d) } else { prev },
                );
        idx += jdx + 1;
        out.push(ch);
    }

    out.iter()
        .fold(0_usize, |acc, x| 10 * acc + (*x - b'0') as usize)
}

#[aoc(2025, 3)]
pub fn main() {
    let data = aoc_input!(2025, 3).unwrap();
    let solve = |n| {
        data.trim()
            .lines()
            .map(|l| max_joltage(l, n))
            .sum::<usize>()
    };

    // Part I
    println!("{}", solve(2));

    // Part II
    println!("{}", solve(12));
}
