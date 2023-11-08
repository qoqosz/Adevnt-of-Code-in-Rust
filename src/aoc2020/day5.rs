use aoc::aoc_input;
use itertools::Itertools;

fn seat_id(code: &str) -> usize {
    let n = code.len();
    let row = &code[..7].replace('F', "0").replace('B', "1");
    let col = &code[n - 3..].replace('R', "1").replace('L', "0");

    usize::from_str_radix(row, 2).unwrap() * 8 + usize::from_str_radix(col, 2).unwrap()
}

fn main() {
    let data = aoc_input!(2020, 5).unwrap();
    let ids: Vec<usize> = data
        .lines()
        .filter(|x| !x.is_empty())
        .map(seat_id)
        .sorted()
        .collect();

    // Part I
    println!("{}", ids.last().unwrap());

    // Part II
    println!(
        "{}",
        ids.windows(2)
            .find(|w| w[0] + 1 != w[1])
            .map(|w| w[0] + 1)
            .unwrap()
    );
}
