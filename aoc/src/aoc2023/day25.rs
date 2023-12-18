use aoc::{aoc, aoc_input};

fn parse(data: &str) {
    _ = data.trim().lines();
}

//#[aoc(2023, 25)]
pub fn main() {
    let data = aoc_input!(2023, 25).unwrap();
    parse(&data);

    // Part I
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
}
