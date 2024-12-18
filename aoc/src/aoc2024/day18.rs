use aoc::{aoc, aoc_input};
use glam::IVec2 as Point;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;

static DIRS: [Point; 4] = [
    Point::new(1, 0),
    Point::new(-1, 0),
    Point::new(0, 1),
    Point::new(0, -1),
];

struct Memory {
    grid: FxHashMap<Point, char>,
    size_x: usize,
    size_y: usize,
}

impl Memory {
    fn new(bytes: &[Point], size_x: usize, size_y: usize) -> Self {
        let grid: FxHashMap<Point, char> = (0..=size_x)
            .cartesian_product(0..=size_y)
            .map(|(x, y)| (Point::new(x as i32, y as i32), '.'))
            .collect();

        let mut memory = Memory {
            grid,
            size_x,
            size_y,
        };

        for byte in bytes {
            memory.insert(*byte);
        }

        memory
    }

    fn insert(&mut self, byte: Point) {
        if 0 <= byte.x
            && byte.x <= self.size_x as i32
            && 0 <= byte.y
            && byte.y <= self.size_y as i32
        {
            self.grid.insert(byte, '#');
        }
    }
}

fn neighbors(p: Point) -> impl Iterator<Item = Point> {
    DIRS.iter().map(move |&d| p + d)
}

fn find_shortest_path(grid: &FxHashMap<Point, char>, start: &Point, end: &Point) -> Option<usize> {
    let mut best_dist = usize::MAX;
    let mut visited = FxHashSet::default();
    let mut queue = VecDeque::new();
    queue.push_back((0, *start));

    while let Some((dist, pos)) = queue.pop_front() {
        // println!("dist = {dist:?}, pos = {pos:?}");
        if dist > best_dist {
            continue;
        }
        if !visited.insert(pos) {
            continue;
        }
        if pos == *end {
            best_dist = std::cmp::min(best_dist, dist);
            continue;
        }

        for adj in neighbors(pos) {
            if grid.get(&adj) == Some(&'.') {
                queue.push_back((dist + 1, adj));
            }
        }
    }

    if best_dist == usize::MAX {
        None
    } else {
        Some(best_dist)
    }
}

fn parse(data: &str) -> Vec<Point> {
    data.trim()
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            Point::new(x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

#[aoc(2024, 18)]
pub fn main() {
    // test();
    let data = aoc_input!(2024, 18).unwrap();
    let bytes = parse(&data);
    let start = Point::new(0, 0);
    let end = Point::new(70, 70);

    // Part I
    let mut memory = Memory::new(&bytes[..1024], 70, 70);
    let dist = find_shortest_path(&memory.grid, &start, &end);
    println!("{}", dist.unwrap());

    // Part II
    let n = bytes.len();

    for i in 1024..n {
        memory.insert(bytes[i]);

        if find_shortest_path(&memory.grid, &start, &end).is_none() {
            println!("{},{}", bytes[i].x, bytes[i].y);
            break;
        }
    }
}
