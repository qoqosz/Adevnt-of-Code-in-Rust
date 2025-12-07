use aoc::{aoc, aoc_input};
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;

type Point = (i32, i32);
type Height = u8;
struct HeightMap(FxHashMap<Point, Height>);

impl HeightMap {
    fn adj(&self, x: &Point) -> impl Iterator<Item = Point> {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .map(|d| (x.0 + d.0, x.1 + d.1))
            .filter(|p| self.0.contains_key(p))
    }

    fn adj_heights<'a>(&'a self, x: &'a Point) -> impl Iterator<Item = &'a Height> {
        self.adj(x).flat_map(|p| self.0.get(&p))
    }

    fn is_low_point(&self, p: &Point) -> bool {
        match self.0.get(p) {
            Some(h) => self.adj_heights(p).all(|n| *n > *h),
            _ => false,
        }
    }

    fn low_points(&self) -> impl Iterator<Item = &Point> {
        self.0.keys().filter(|p| self.is_low_point(p))
    }

    fn total_risk_level(&self) -> usize {
        self.low_points()
            .flat_map(|p| self.0.get(p))
            .map(|h| 1 + *h as usize)
            .sum()
    }

    fn basin_size(&self, p: &Point) -> Option<usize> {
        if self.0.get(p).is_none() {
            return None;
        }

        let mut queue = VecDeque::from([*p]);
        let mut visited = FxHashSet::default();
        let mut count = 0;

        while let Some(q) = queue.pop_front() {
            if !visited.insert(q) {
                continue;
            }
            if self.0.get(&q) == Some(&9) {
                continue;
            }

            count += 1;
            queue.extend(self.adj(&q));
        }

        Some(count)
    }
}

impl TryFrom<&str> for HeightMap {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let map = value
            .trim()
            .lines()
            .enumerate()
            .flat_map(|(i, line)| {
                line.bytes()
                    .enumerate()
                    .map(move |(j, ch)| ((i as i32, j as i32), ch - b'0'))
            })
            .collect();
        Ok(Self(map))
    }
}

#[aoc(2021, 9)]
pub fn main() {
    let data = aoc_input!(2021, 9).unwrap();
    let height_map = HeightMap::try_from(data.as_str()).unwrap();

    // Part I
    println!("{}", height_map.total_risk_level());

    // Part II
    let ans = height_map
        .low_points()
        .flat_map(|p| height_map.basin_size(p))
        .k_largest(3)
        .product::<usize>();
    println!("{ans}");
}
