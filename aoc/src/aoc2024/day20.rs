use aoc::{aoc, aoc_input, heap::MinHeap};
use glam::IVec2 as Point;
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::{
    hash_map::Entry::{Occupied, Vacant},
    VecDeque,
};

static DIRS: [Point; 4] = [
    Point::new(1, 0),
    Point::new(-1, 0),
    Point::new(0, 1),
    Point::new(0, -1),
];

trait Neighbors {
    type Item;

    fn neighbors(&self, x: Self::Item) -> impl Iterator<Item = Self::Item>;
    fn cheats(&self, x: Self::Item, duration: usize) -> impl IntoIterator<Item = Self::Item>;
}

impl Neighbors for FxHashMap<Point, char> {
    type Item = Point;

    fn neighbors(&self, x: Self::Item) -> impl Iterator<Item = Self::Item> {
        DIRS.iter()
            .map(move |&d| x + d)
            .filter(move |adj| self.get(adj) != Some(&'#'))
    }

    fn cheats(&self, x: Self::Item, duration: usize) -> impl IntoIterator<Item = Self::Item> {
        let mut neighbors = FxHashSet::default();
        let mut queue = VecDeque::new();
        queue.push_back((0, x));

        while let Some((dist, pos)) = queue.pop_front() {
            if dist > duration {
                continue;
            }
            if !neighbors.insert(pos) {
                continue;
            }
            for next in DIRS.iter().map(move |&d| pos + d) {
                queue.push_back((dist + 1, next));
            }
        }

        neighbors
            .iter()
            .filter(move |adj| self.get(adj) != Some(&'#') && **adj != x)
            .copied()
            .collect::<Vec<_>>()
    }
}

fn dijkstra(
    map: &FxHashMap<Point, char>,
    start: &Point,
) -> (FxHashMap<Point, i32>, FxHashMap<Point, Point>) {
    let mut visited = FxHashSet::default();
    let mut prev = FxHashMap::default();
    let mut dist = FxHashMap::default();
    dist.insert(*start, 0);

    let mut queue = MinHeap::new();
    queue.push(0, *start);

    while let Some((score, node)) = queue.pop() {
        if !visited.insert(node) {
            continue;
        }

        for adj in map.neighbors(node) {
            let next_score = score + 1;

            match dist.entry(adj) {
                Occupied(ent) => {
                    if next_score < *ent.get() {
                        *ent.into_mut() = next_score;
                        queue.push(next_score, adj);
                        prev.insert(adj, node);
                    }
                }
                Vacant(ent) => {
                    ent.insert(next_score);
                    queue.push(next_score, adj);
                    prev.insert(adj, node);
                }
            }
        }
    }

    (dist, prev)
}

fn shortest_path(end: &Point, prev: &FxHashMap<Point, Point>) -> Vec<Point> {
    let mut path = vec![*end];
    let mut u = *end;

    while let Some(&v) = prev.get(&u) {
        path.push(v);
        u = v;
    }

    path.iter().copied().collect()
}

fn solve(map: &FxHashMap<Point, char>, duration: usize) -> usize {
    let start = map
        .iter()
        .find(|(_, v)| **v == 'S')
        .map(|(k, _)| k)
        .unwrap();
    let end = map
        .iter()
        .find(|(_, v)| **v == 'E')
        .map(|(k, _)| k)
        .unwrap();

    let (dist, prev) = dijkstra(map, end);
    let path = shortest_path(&start, &prev);

    let mut ans = vec![];

    for (i, elem) in path.iter().enumerate() {
        for cheat in map.cheats(*elem, duration) {
            let shortcut = (elem.x - cheat.x).abs() as usize + (elem.y - cheat.y).abs() as usize;

            if cheat == *end {
                ans.push(i + shortcut);
            } else if let Some(&d) = dist.get(&cheat) {
                ans.push(i + shortcut + d as usize);
            }
        }
    }

    ans.iter()
        .filter(|x| **x < path.len())
        .map(|x| path.len() - *x)
        .filter(|x| *x >= 100)
        .count()
}

fn parse(data: &str) -> FxHashMap<Point, char> {
    data.trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, ch)| (Point::new(x as i32, y as i32), ch))
        })
        .collect()
}

#[aoc(2024, 20)]
pub fn main() {
    let data = aoc_input!(2024, 20).unwrap();
    let maze = parse(&data);

    // Part I
    println!("{}", solve(&maze, 2));

    // Part II
    println!("{}", solve(&maze, 20));
}
