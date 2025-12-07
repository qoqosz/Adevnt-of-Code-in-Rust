use aoc::{aoc, aoc_input};

fn parse(data: &str) {
    _ = data.trim().lines();
}

#[aoc(2023, 18)]
pub fn main() {
    let data = aoc_input!(2023, 18).unwrap();
    parse(&data);

    // Part I

    // Part II
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";

    #[test]
    fn test_part1() {
        parse(EXAMPLE);
    }

    #[test]
    fn test_part2() {
        //        assert_eq!(part2(&lines), 1);
    }
}
