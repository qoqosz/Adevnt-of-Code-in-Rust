use aoc::{aoc, aoc_input};

#[aoc(2022, 2)]
pub fn main() {
    let data = aoc_input!(2022, 2).unwrap();
    let (mut score1, mut score2) = (0, 0);

    for line in data.trim().lines() {
        let mut chs = line.bytes();
        let a = chs.next().unwrap();
        let b = chs.nth(1).unwrap();
        score1 += 3 * ((b - a + 2) as u32 % 3) + 1 + (b + 2) as u32 % 3;
        score2 += (b + a + 2) as u32 % 3 + 1 + 3 * ((b + 2) as u32 % 3);
    }

    // Part I
    println!("{score1}");

    // Part II
    println!("{score2}");
}
