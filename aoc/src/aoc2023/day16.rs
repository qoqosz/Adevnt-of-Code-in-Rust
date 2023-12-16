use aoc::{aoc, aoc_input};
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;

type State = (i32, i32, i32, i32);

fn parse(data: &str) -> FxHashMap<(i32, i32), char> {
    data.trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, ch)| ((x as i32, y as i32), ch))
        })
        .collect()
}

/// `init` lays outside the layout
fn solve(contraption: &FxHashMap<(i32, i32), char>, init: &State) -> usize {
    let mut visited: FxHashSet<State> = FxHashSet::default();
    let mut queue: VecDeque<State> = VecDeque::new();
    queue.push_back(*init);

    while let Some(state) = queue.pop_front() {
        if !visited.insert(state) {
            continue;
        }

        let (x, y, dx, dy) = state;

        if let Some(&ch) = contraption.get(&(x + dx, y + dy)) {
            match (dx, dy, ch) {
                // continue in the same direction}
                (_, _, '.') => {
                    queue.push_back((x + dx, y + dy, dx, dy));
                }
                // passes through the splitter as if the splitter were empty space
                (_, 1, '|') | (_, -1, '|') => {
                    queue.push_back((x + dx, y + dy, dx, dy));
                }
                // the beam is split into two beams going in each of the two directions
                (1, _, '|') | (-1, _, '|') => {
                    queue.push_back((x + dx, y + dy, 0, 1));
                    queue.push_back((x + dx, y + dy, 0, -1));
                }
                // passes through the splitter as if the splitter were empty space
                (1, _, '-') | (-1, _, '-') => {
                    queue.push_back((x + dx, y + dy, dx, dy));
                }
                // the beam is split into two beams going in each of the two directions
                (_, 1, '-') | (_, -1, '-') => {
                    queue.push_back((x + dx, y + dy, 1, 0));
                    queue.push_back((x + dx, y + dy, -1, 0));
                }
                // the beam is reflected 90 degrees
                (_, _, '\\') => {
                    queue.push_back((x + dx, y + dy, dy, dx));
                }
                // the beam is reflected 90 degrees
                (_, _, '/') => {
                    queue.push_back((x + dx, y + dy, -dy, -dx));
                }
                _ => unreachable!(),
            }
        }
    }
    visited
        .iter()
        .map(|state| (state.0, state.1))
        .unique()
        .count()
        - 1
}

fn maximize(contraption: &FxHashMap<(i32, i32), char>, width: i32, height: i32) -> Option<usize> {
    let mut inits = vec![];

    for x in 0..width {
        // top row
        inits.push((x, -1, 0, 1));

        // bottom row
        inits.push((x, height, 0, -1));
    }

    for y in 0..height {
        // leftmost column
        inits.push((-1, y, 1, 0));

        // rightmost column
        inits.push((width, y, -1, 0));
    }

    inits.iter().map(|init| solve(contraption, init)).max()
}

#[aoc(2023, 16)]
pub fn main() {
    let data = aoc_input!(2023, 16).unwrap();
    let contraption = parse(&data);

    // Part I
    let init = (-1, 0, 1, 0);
    println!("{}", solve(&contraption, &init));

    // Part II
    let energy = maximize(&contraption, 110, 110);
    println!("{}", energy.unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

    #[test]
    fn test_part1() {
        let contraption = parse(EXAMPLE);
        let init = (-1, 0, 1, 0);
        let n = solve(&contraption, &init);
        assert_eq!(n, 46);
    }

    #[test]
    fn test_part2() {
        let contraption = parse(EXAMPLE);
        let energy = maximize(&contraption, 10, 10);
        assert_eq!(energy, Some(51));
    }
}
