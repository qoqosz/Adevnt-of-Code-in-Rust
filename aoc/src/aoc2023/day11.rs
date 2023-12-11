use aoc::{aoc, aoc_input};
use itertools::Itertools;
use rustc_hash::FxHashSet;

fn parse(data: &str) -> FxHashSet<(i64, i64)> {
    data.lines()
        .filter(|x| !x.is_empty())
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(move |(j, ch)| ((i as i64, j as i64), ch))
        })
        .filter(|(_, c)| *c == '#')
        .map(|(p, _)| p)
        .collect()
}

fn expand(sky: &FxHashSet<(i64, i64)>, inc: i64) -> FxHashSet<(i64, i64)> {
    let n_rows = sky.iter().map(|(r, _)| *r).max().unwrap();
    let n_cols = sky.iter().map(|(_, c)| *c).max().unwrap();

    let empty_rows = (0..n_rows)
        .filter(|i| sky.iter().filter(|(r, _)| r == i).count() == 0)
        .collect::<Vec<_>>();
    let empty_cols = (0..n_cols)
        .filter(|j| sky.iter().filter(|(_, c)| c == j).count() == 0)
        .collect::<Vec<_>>();

    sky.iter()
        .map(|(row, col)| {
            (
                row + empty_rows.iter().filter(|r| *r < row).count() as i64 * inc,
                col + empty_cols.iter().filter(|c| *c < col).count() as i64 * inc,
            )
        })
        .collect()
}

fn shortest_paths(sky: &FxHashSet<(i64, i64)>) -> u64 {
    sky.iter()
        .tuple_combinations()
        .map(|(a, b)| a.0.abs_diff(b.0) + a.1.abs_diff(b.1))
        .sum()
}

#[aoc(2023, 11)]
pub fn main() {
    let data = aoc_input!(2023, 11).unwrap();
    let sky = parse(&data);

    // Part I
    let sky1 = expand(&sky, 1);
    println!("{:?}", shortest_paths(&sky1));

    // Part II
    let sky2 = expand(&sky, 1_000_000 - 1);
    println!("{:?}", shortest_paths(&sky2));
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_part1() {
        let mut sky = parse(EXAMPLE);
        sky = expand(&sky, 1);
        assert_eq!(shortest_paths(&sky), 374);
    }

    #[test]
    fn test_part2() {
        let mut sky = parse(EXAMPLE);
        sky = expand(&sky, 10 - 1);
        assert_eq!(shortest_paths(&sky), 1030);
    }
}
