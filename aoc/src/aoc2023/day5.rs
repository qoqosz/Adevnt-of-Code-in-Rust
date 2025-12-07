use aoc::{aoc, aoc_input};

fn part1(lines: &[&str]) -> String {
    println!("{lines:?}");
    "".to_string()
}

fn part2(lines: &[&str]) -> String {
    println!("{lines:?}");
    "".to_string()
}

fn parse(data: &str) -> Vec<&str> {
    data.lines().filter(|x| !x.is_empty()).collect()
}

#[aoc(2023, 5)]
pub fn main() {
    // let data = aoc_input!(2023, 5).unwrap();
    let data = "hello\nworld\n".to_owned();
    let lines = parse(&data);

    // Part I
    println!("{}", part1(&lines));

    // Part II
    println!("{}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE1: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    static EXAMPLE2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_part1() {
        let lines = parse(EXAMPLE1);
        assert_eq!(part1(&lines), "");
    }

    #[test]
    fn test_part2() {
        let lines = parse(EXAMPLE2);
        assert_eq!(part2(&lines), "");
    }
}
