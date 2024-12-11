use aoc::{aoc, aoc_input};
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;

static DIRS: &[(i8, i8); 4] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];

fn parse(data: &str) -> FxHashMap<(i8, i8), i8> {
    data.trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, ch)| ((x as i8, y as i8), (ch as u8 - 48) as i8))
        })
        .collect()
}

fn neighbors<'a>(point: &'a (i8, i8)) -> impl Iterator<Item = (i8, i8)> + 'a {
    DIRS.iter().map(move |d| (point.0 + d.0, point.1 + d.1))
}

fn find_trails(map: &FxHashMap<(i8, i8), i8>, start: &(i8, i8)) -> FxHashSet<Vec<(i8, i8)>> {
    let mut queue = VecDeque::new();
    let mut visited = FxHashSet::default();
    let path = vec![*start];
    let mut hiking_trails = FxHashSet::default();
    queue.push_back(path);

    while let Some(path) = queue.pop_front() {
        if !visited.insert(path.clone()) {
            continue;
        }

        if let Some(pos) = path.last() {
            if let Some(&height) = map.get(pos) {
                if height == 9 {
                    hiking_trails.insert(path);
                    continue;
                }

                for n in neighbors(pos) {
                    if let Some(&next) = map.get(&n) {
                        if next == height + 1 {
                            let mut next_path = path.clone();
                            next_path.push(n);
                            queue.push_back(next_path);
                        }
                    }
                }
            }
        }
    }

    hiking_trails
}

#[aoc(2024, 10)]
pub fn main() {
    let data = aoc_input!(2024, 10).unwrap();
    let map = parse(&data);
    let hiking_trails: Vec<_> = map
        .iter()
        .filter(|(_, v)| **v == 0)
        .map(|(k, _)| find_trails(&map, k))
        .collect();

    // Part I
    let score = hiking_trails
        .iter()
        .map(|trails| trails.iter().flat_map(|p| p.last()).unique().count())
        .sum::<usize>();
    println!("{score}");

    // Part II
    let score = hiking_trails
        .iter()
        .map(|trails| trails.len())
        .sum::<usize>();
    println!("{score}");
}
