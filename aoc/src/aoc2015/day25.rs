use aoc::aoc;

fn get_term(x0: u64, n: u64) -> u64 {
    let a: u64 = 252533;
    let b: u64 = 33554393;
    let c = (1..n).fold(1, |acc, _| acc * a % b);
    x0 * c % b
}

fn iloc(row: u64, col: u64) -> u64 {
    let n = row + col - 1;
    col + n * (n - 1) / 2
}

#[aoc(2015, 25)]
pub fn main() {
    let (row, col) = (3010, 3019);
    let x0 = 20151125;
    let i = iloc(row, col);
    println!("{}", get_term(x0, i));
}
