use aoc::{aoc, aoc_input, utils::transpose};

struct ReflectionPattern<'a> {
    pattern: &'a str,
}

impl<'a> ReflectionPattern<'a> {
    fn new(pattern: &'a str) -> Self {
        Self { pattern }
    }

    fn rows(&self) -> Vec<u32> {
        self.pattern.lines().flat_map(line_to_u32).collect()
    }

    fn cols(&self) -> Vec<u32> {
        let chars = self
            .pattern
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        transpose(chars)
            .iter()
            .flat_map(|line| line_to_u32(&line.iter().collect::<String>()))
            .collect()
    }
}

fn line_to_u32(line: &str) -> Result<u32, std::num::ParseIntError> {
    u32::from_str_radix(&line.replace('.', "0").replace('#', "1"), 2)
}

fn reflect(data: &[u32], diff: u32) -> Option<usize> {
    (1..data.len()).find(|&i| {
        let left = data[..i].iter().rev();
        let right = data[i..].iter();

        left.zip(right)
            .map(|(x, y)| (*x ^ *y).count_ones())
            .sum::<u32>()
            == diff
    })
}

fn solve(pattern: &ReflectionPattern, diff: u32) -> usize {
    let mut res = 0;

    if let Some(n) = reflect(&pattern.rows(), diff) {
        res += 100 * n;
    }
    if let Some(n) = reflect(&pattern.cols(), diff) {
        res += n;
    }

    res
}

fn parse(data: &str) -> Vec<ReflectionPattern<'_>> {
    data.split("\n\n")
        .map(ReflectionPattern::new)
        .collect::<Vec<_>>()
}

#[aoc(2023, 13)]
pub fn main() {
    let data = aoc_input!(2023, 13).unwrap();
    let patterns = parse(&data);

    // Part I
    let summary = patterns.iter().map(|p| solve(p, 0)).sum::<usize>();
    println!("{summary}");

    // Part II
    let summary = patterns.iter().map(|p| solve(p, 1)).sum::<usize>();
    println!("{summary}");
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE1: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_binary_rows() {
        let patterns = parse(EXAMPLE1);
        let pattern = &patterns[0];
        let rows = [358, 90];
        assert_eq!(pattern.rows()[..2], rows);
    }

    #[test]
    fn test_binary_cols() {
        let patterns = parse(EXAMPLE1);
        let pattern = &patterns[0];
        let cols = [89, 24];
        assert_eq!(pattern.cols()[..2], cols);
    }

    #[test]
    fn test_case1() {
        let patterns = parse(EXAMPLE1);
        let pattern = &patterns[0];
        assert_eq!(reflect(&pattern.cols(), 0), Some(5));
    }

    #[test]
    fn test_case2() {
        let patterns = parse(EXAMPLE1);
        let pattern = &patterns[1];
        assert_eq!(reflect(&pattern.rows(), 0), Some(4));
    }

    #[test]
    fn test_part1() {
        let patterns = parse(EXAMPLE1);
        let ans = patterns.iter().map(|p| solve(p, 0)).sum::<usize>();
        assert_eq!(ans, 405);
    }

    #[test]
    fn test_part2() {
        let patterns = parse(EXAMPLE1);
        let ans = patterns.iter().map(|p| solve(p, 1)).sum::<usize>();
        assert_eq!(ans, 400);
    }
}
