use aoc::{aoc, aoc_input};
use rustc_hash::{FxHashMap, FxHashSet};

type Plan<'a> = Vec<(char, i64)>;

fn parse(data: &str) -> Plan {
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

fn parse_hex(data: &str) -> Plan {
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

fn dig(plan: &Plan) -> Vec<(i64, i64)> {
    let mut hole = vec![];
    let (mut x, mut y) = (0, 0);
    hole.push((x, y));

    for (dir, step) in plan {
        let (dx, dy) = match dir {
            'R' => (1, 0),
            'L' => (-1, 0),
            'D' => (0, 1),
            'U' => (0, -1),
            _ => unreachable!(),
        };

        (x, y) = (x + step * dx, y + step * dy);
        hole.push((x, y));
    }

    hole
}

fn shoelace(hole: &Vec<(i64, i64)>) -> i64 {
    let mut points = Vec::from_iter(hole.iter());
    let n = points.len();
    let mut area = 0;

    for i in 1..n - 1 {
        area += points[i].1 * (points[i - 1].0 - points[i + 1].0);
    }

    area += points[0].1 * (points[n - 1].0 - points[1].0);
    area += points[n - 1].1 * (points[n - 2].0 - points[0].0);

    points.push(points[0]);
    let adj = points
        .windows(2)
        .map(|w| w[0].0.abs_diff(w[1].0) + w[0].1.abs_diff(w[1].1))
        .map(|x| x as i64)
        .sum::<i64>();

    (area + adj) / 2 + 1
}

#[aoc(2023, 18)]
pub fn main() {
    let data = aoc_input!(2023, 18).unwrap();

    // Part I
    let plan = parse(&data);
    let hole = dig(&plan);
    let n = shoelace(&hole);
    println!("{n}");

    // Part II
    let plan = parse_hex(&data);
    let hole = dig(&plan);
    let n = shoelace(&hole);
    println!("{n}");
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
