use aoc::{aoc, aoc_input};
use itertools::Itertools;

fn parse(data: &str) -> Vec<(char, i64)> {
    data.trim()
        .lines()
        .map(|line| {
            let (n, _) = line[2..].split_once(' ').unwrap();
            let n = n.parse::<i64>().unwrap();
            let dir = line.chars().next().unwrap();
            (dir, n)
        })
        .collect()
}

fn parse_hex(data: &str) -> Vec<(char, i64)> {
    data.trim()
        .lines()
        .map(|line| {
            let (_, color) = line[2..].split_once(' ').unwrap();
            let color = color.trim_start_matches("(#").trim_end_matches(')');
            let n = i64::from_str_radix(&color[..5], 16).unwrap();
            let dir = match color.chars().last().unwrap() {
                '0' => 'R',
                '1' => 'D',
                '2' => 'L',
                '3' => 'U',
                _ => unreachable!(),
            };
            (dir, n)
        })
        .collect()
}

fn dig(plan: &Vec<(char, i64)>) -> Vec<(i64, i64)> {
    let (mut x, mut y) = (0, 0);
    let mut edges = vec![(x, y)];

    for (dir, step) in plan {
        let (dx, dy) = match dir {
            'R' => (1, 0),
            'L' => (-1, 0),
            'D' => (0, 1),
            'U' => (0, -1),
            _ => unreachable!(),
        };

        (x, y) = (x + step * dx, y + step * dy);
        edges.push((x, y));
    }

    edges
}

fn shoelace(edges: &[(i64, i64)]) -> u64 {
    let area: i64 = edges
        .iter()
        .circular_tuple_windows::<(_, _, _)>()
        .map(|(w0, w1, w2)| w1.1 * (w0.0 - w2.0))
        .sum();

    let border: u64 = edges
        .iter()
        .circular_tuple_windows()
        .map(|(w0, w1)| w0.0.abs_diff(w1.0) + w0.1.abs_diff(w1.1))
        .sum();

    (area as u64 + border) / 2 + 1
}

#[aoc(2023, 18)]
pub fn main() {
    let data = aoc_input!(2023, 18).unwrap();

    // Part I
    let plan = parse(&data);
    let edges = dig(&plan);
    println!("{}", shoelace(&edges));

    // Part II
    let plan = parse_hex(&data);
    let edges = dig(&plan);
    println!("{}", shoelace(&edges));
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn test_part1() {
        let plan = parse(EXAMPLE);
        let hole = dig(&plan);
        let n = shoelace(&hole);
        assert_eq!(n, 62);
    }

    #[test]
    fn test_part2() {
        let plan = parse_hex(EXAMPLE);
        let hole = dig(&plan);
        let n = shoelace(&hole);
        assert_eq!(n, 952408144115);
    }
}
