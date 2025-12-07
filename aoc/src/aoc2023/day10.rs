use aoc::{aoc, aoc_input};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;

type Coord = (i32, i32);
type Diagram = FxHashMap<Coord, char>;

#[derive(Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

static TO_NORTH: &str = "S|F7";
static TO_SOUTH: &str = "S|JL";
static TO_EAST: &str = "S-7J";
static TO_WEST: &str = "S-FL";

trait Connection {
    fn is_connected(&self, src: &Coord, dest: &Coord, dir: Direction) -> bool;
}

impl Connection for Diagram {
    fn is_connected(&self, src: &Coord, dest: &Coord, dir: Direction) -> bool {
        let start = match self.get(src) {
            Some(start) => *start,
            None => {
                return false;
            }
        };
        let end = match self.get(dest) {
            Some(end) => *end,
            None => {
                return false;
            }
        };

        match dir {
            Direction::North => TO_SOUTH.contains(start) && TO_NORTH.contains(end),
            Direction::South => TO_NORTH.contains(start) && TO_SOUTH.contains(end),
            Direction::East => TO_WEST.contains(start) && TO_EAST.contains(end),
            Direction::West => TO_EAST.contains(start) && TO_WEST.contains(end),
        }
    }
}

fn find_loop(diagram: &Diagram) -> FxHashSet<Coord> {
    let mut visited: FxHashSet<Coord> = FxHashSet::default();
    let mut queue: VecDeque<Coord> = VecDeque::from_iter([get_start(diagram)]);

    while let Some((i, j)) = queue.pop_front() {
        if !visited.insert((i, j)) {
            continue;
        }
        if diagram.is_connected(&(i, j), &(i - 1, j), Direction::North) {
            queue.push_back((i - 1, j));
        }
        if diagram.is_connected(&(i, j), &(i, j - 1), Direction::West) {
            queue.push_back((i, j - 1));
        }
        if diagram.is_connected(&(i, j), &(i + 1, j), Direction::South) {
            queue.push_back((i + 1, j));
        }
        if diagram.is_connected(&(i, j), &(i, j + 1), Direction::East) {
            queue.push_back((i, j + 1));
        }
    }

    visited
}

fn area(diagram: &Diagram, r#loop: &FxHashSet<Coord>) -> usize {
    let y = diagram.keys().max_by_key(|k| k.0).unwrap().0;
    let x = diagram.keys().max_by_key(|k| k.1).unwrap().1;
    let mut res = 0;
    let toggle = "|F7"; // Or with "S" - depends on the input type

    for i in 0..y {
        let mut is_outside = true;
        for j in 0..x {
            let pos = (i, j);
            let ch = *diagram.get(&pos).unwrap();

            if !is_outside && !r#loop.contains(&pos) {
                // println!("{:?}", pos);
                res += 1;
            }
            if r#loop.contains(&pos) && toggle.contains(ch) {
                is_outside = !is_outside;
            }
        }
    }

    res
}

fn parse(data: &str) -> Diagram {
    data.lines()
        .enumerate()
        .filter(|(_, x)| !x.is_empty())
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(move |(j, ch)| ((i as i32, j as i32), ch))
        })
        .collect()
}

fn get_start(diagram: &Diagram) -> Coord {
    for (k, v) in diagram {
        if *v == 'S' {
            return *k;
        }
    }
    unreachable!()
}

#[aoc(2023, 10)]
pub fn main() {
    let data = aoc_input!(2023, 10).unwrap();
    let diagram = parse(&data);
    let r#loop = find_loop(&diagram);

    // Part I
    println!("{}", r#loop.len() / 2);

    // Part II
    println!("{}", area(&diagram, &r#loop));
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE1: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

    static EXAMPLE2: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    static EXAMPLE3: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    static EXAMPLE4: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJIF7FJ-
L---JF-JLJIIIIFJLJJ7
|F|F-JF---7IIIL7L|7|
|FFJF7L7F-JF7IIL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    static EXAMPLE5: &str = "OF----7F7F7F7F-7OOOO
O|F--7||||||||FJOOOO
O||OFJ||||||||L7OOOO
FJL7L7LJLJ||LJIL-7OO
L--JOL7IIILJS7F-7L7O
OOOOF-JIIF7FJ|L7L7L7
OOOOL7IF7||L7|IL7L7|
OOOOO|FJLJ|FJ|F7|OLJ
OOOOFJL-7O||O||||OOO
OOOOL---JOLJOLJLJOOO";

    #[test]
    fn test1_part1() {
        let diagram = parse(EXAMPLE1);
        let r#loop = find_loop(&diagram);
        println!("{}", r#loop.len() / 2);
    }

    #[test]
    fn test2_part1() {
        let diagram = parse(EXAMPLE2);
        let r#loop = find_loop(&diagram);
        println!("{}", r#loop.len() / 2);
    }

    #[test]
    fn test1_part2() {
        let diagram = parse(EXAMPLE3);
        let r#loop = find_loop(&diagram);
        println!("{}", r#loop.len() / 2);
    }

    #[test]
    fn test2_part2() {
        let diagram = parse(EXAMPLE4);
        let r#loop = find_loop(&diagram);

        println!("{}", area(&diagram, &r#loop));
        assert_eq!(area(&diagram, &r#loop), 10);
    }

    #[test]
    fn test3_part2() {
        let diagram = parse(EXAMPLE5);
        let r#loop = find_loop(&diagram);

        println!("{}", area(&diagram, &r#loop));
        assert_eq!(area(&diagram, &r#loop), 8);
    }
}
