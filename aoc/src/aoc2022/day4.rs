use aoc::{aoc, aoc_input};

type Interval = aoc::interval::Interval<u8>;

#[inline(always)]
fn parse_line(line: &str) -> (Interval, Interval) {
    let (sec1, sec2) = line.split_once(',').unwrap();
    let int1 = sec1.split_once('-').unwrap();
    let int2 = sec2.split_once('-').unwrap();
    (
        Interval::new(int1.0.parse::<u8>().unwrap(), int1.1.parse::<u8>().unwrap()),
        Interval::new(int2.0.parse::<u8>().unwrap(), int2.1.parse::<u8>().unwrap()),
    )
}

#[aoc(2022, 4)]
pub fn main() {
    let data = aoc_input!(2022, 4).unwrap();
    let (mut n1, mut n2) = (0, 0);

    for line in data.trim().lines() {
        let (int1, int2) = parse_line(line);

        if int1.contains(&int2) || int2.contains(&int1) {
            n1 += 1;
        }
        if int1.intersects(&int2) {
            n2 += 1;
        }
    }
    // Part I
    println!("{n1}");

    // Part II
    println!("{n2}");
}
