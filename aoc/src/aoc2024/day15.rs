use aoc::{aoc, aoc_input};
use glam::IVec2 as Point;
use rustc_hash::FxHashMap;
use std::collections::VecDeque;

fn parse(data: &str) -> (FxHashMap<Point, char>, Vec<Point>) {
    let (map, moves) = data.trim().split_once("\n\n").unwrap();

    let map = map
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, ch)| (Point::new(x as i32, y as i32), ch))
        })
        .collect();

    let moves = moves
        .chars()
        .filter(|ch| *ch != '\n')
        .map(|ch| match ch {
            '<' => Point::new(-1, 0),
            '>' => Point::new(1, 0),
            'v' => Point::new(0, 1),
            '^' => Point::new(0, -1),
            _ => unreachable!(),
        })
        .collect();

    (map, moves)
}

fn simulate(map: &mut FxHashMap<Point, char>, moves: &[Point]) {
    let mut robot = map
        .iter()
        .find(|(_, &ch)| ch == '@')
        .map(|(k, _)| *k)
        .unwrap();

    'outer: for &dir in moves {
        let mut queue = VecDeque::from([robot]);
        let mut visited = vec![];

        while let Some(pos) = queue.pop_front() {
            if visited.contains(&pos) {
                continue;
            }
            visited.push(pos);

            let next = pos + dir;

            match map[&next] {
                '@' => continue,
                '.' => continue,
                '#' => continue 'outer,
                'O' => queue.push_back(next),
                '[' => {
                    queue.push_back(next);
                    queue.push_back(next + Point::new(1, 0));
                }
                ']' => {
                    queue.push_back(next);
                    queue.push_back(next + Point::new(-1, 0));
                }
                _ => unreachable!(),
            }
        }

        while let Some(pos) = visited.pop() {
            let next = pos + dir;

            if !visited.contains(&next) {
                map.insert(next, map[&pos]);
                map.insert(pos, '.');
            }
        }

        robot += dir;
    }
}

fn gps(map: &FxHashMap<Point, char>, tile: char) -> i32 {
    map.iter()
        .filter(|(_, ch)| **ch == tile)
        .map(|(p, _)| 100 * p.y + p.x)
        .sum()
}

#[aoc(2024, 15)]
pub fn main() {
    let data = aoc_input!(2024, 15).unwrap();

    // Part I
    let (mut map, moves) = parse(&data);
    simulate(&mut map, &moves);
    println!("{}", gps(&map, 'O'));

    // Part II
    let data = data
        .replace("#", "##")
        .replace("O", "[]")
        .replace(".", "..")
        .replace("@", "@.");
    let (mut map, moves) = parse(&data);
    simulate(&mut map, &moves);
    println!("{}", gps(&map, '['));
}
