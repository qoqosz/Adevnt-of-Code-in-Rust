use std::collections::VecDeque;

use aoc::{aoc, aoc_input};
use rustc_hash::{FxHashMap, FxHashSet};

type Garden = FxHashMap<(i32, i32), char>;

fn parse(data: &str) -> Garden {
    data.trim()
        .lines()
        .filter(|x| !x.is_empty())
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, ch)| ((x as i32, y as i32), ch))
        })
        .collect()
}

fn search(garden: &Garden, n_steps: usize) -> usize {
    let n = 131;
    let mut queue = VecDeque::new();
    let mut visited = FxHashSet::default();
    let mut count: FxHashMap<usize, usize> = FxHashMap::default();

    let start = *garden.iter().find(|x| *x.1 == 'S').unwrap().0;
    queue.push_back((0, start));

    while let Some((i, pos)) = queue.pop_front() {
        if !visited.insert((i, pos)) {
            continue;
        }

        if i > n_steps {
            continue;
        }

        *count.entry(i).or_insert(0) += 1;

        for next in [
            (pos.0, pos.1 - 1),
            (pos.0, pos.1 + 1),
            (pos.0 - 1, pos.1),
            (pos.0 + 1, pos.1),
        ] {
            let adj = ((next.0).rem_euclid(n), (next.1).rem_euclid(n));
            if let Some(ch) = garden.get(&adj) {
                if *ch != '#' {
                    queue.push_back((i + 1, next));
                }
            }
        }
    }

    *count.get(&n_steps).unwrap()
}

#[aoc(2023, 21)]
pub fn main() {
    let data = aoc_input!(2023, 21).unwrap();
    let garden = parse(&data);
    let n: usize = 131;

    // Part I
    println!("{}", search(&garden, 64));

    // Part II
    // Let f(n) be the number of spaces you can reach after n steps. Let X be the length of
    // your input grid. f(n), f(n+X), f(n+2X), ...., is a quadratic, so you can find it by
    // finding the first 3 values, then use that to interpolate the final answer.
    let n_steps: usize = 26501365;
    let (x, y) = (n_steps / n, n_steps % n);

    let points = (
        search(&garden, y),         // f(0) = a * 0 + b * 0 + c
        search(&garden, y + n),     // f(1) = a + b + c
        search(&garden, y + 2 * n), // f(2) = 4a + 2b + c
    );

    // Find (a, b, c)
    let (a, b, c) = (
        (points.2 - 2 * points.1 + points.0) / 2,
        2 * points.1 - (points.2 + 3 * points.0) / 2,
        points.0,
    );
    println!("{}", a * x * x + b * x + c);
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn test_part1() {
        let garden = parse(EXAMPLE);
        let res = search(&garden, 6);
        println!("{}", res);
    }
}
