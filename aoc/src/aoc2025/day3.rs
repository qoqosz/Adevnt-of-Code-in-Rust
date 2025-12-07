use aoc::{aoc, aoc_input};

fn max_joltage(bank: &str, sz: usize) -> usize {
    fn inner(bank: &str, sz: usize) -> (u8, &str) {
        let n = bank.len();
        let digit = bank[..n - sz + 1].as_bytes().iter().max().unwrap();
        let idx = bank.as_bytes().iter().position(|x| x == digit).unwrap();

        (*digit, &bank[idx + 1..])
    }

    let mut out = Vec::with_capacity(sz);
    let mut bank = bank;

    for i in 0..sz {
        let ch: u8;
        (ch, bank) = inner(bank, sz - i);
        out.push(ch);
    }

    str::from_utf8(&out).unwrap().parse().unwrap()
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
