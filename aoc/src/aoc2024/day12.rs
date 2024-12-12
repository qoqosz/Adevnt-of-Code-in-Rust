use aoc::{aoc, aoc_input};
use rustc_hash::{FxHashMap, FxHashSet};
use std::str::FromStr;

static DIRS: &[(i16, i16); 4] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];

trait Adjacent {
    fn adj(&self) -> impl Iterator<Item = Self>;
}

impl Adjacent for (i16, i16) {
    fn adj(&self) -> impl Iterator<Item = Self> {
        DIRS.iter().map(move |d| (self.0 + d.0, self.1 + d.1))
    }
}

struct Garden {
    map: FxHashMap<(i16, i16), char>,
}

impl FromStr for Garden {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            map: s
                .trim()
                .lines()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .map(move |(x, ch)| ((x as i16, y as i16), ch))
                })
                .collect(),
        })
    }
}

impl Garden {
    fn regions(&self) -> Vec<Region> {
        let mut map = self.map.clone();
        let mut regions = vec![];

        loop {
            let point = map.keys().next().unwrap();
            let region = self.find_region(point);

            for p in &region.points {
                map.remove(p);
            }

            regions.push(region);

            if map.is_empty() {
                break;
            }
        }

        regions
    }

    fn find_region(&self, start: &(i16, i16)) -> Region {
        let mut region = Region::default();
        let mut queue = vec![*start];
        let mut visited = FxHashSet::default();
        let plant = *self.map.get(start).unwrap();

        while let Some(point) = queue.pop() {
            if !visited.insert(point) {
                continue;
            }

            if let Some(p) = self.map.get(&point) {
                if *p == plant {
                    region.points.insert(point);

                    for adj in point.adj() {
                        queue.push(adj);
                    }
                }
            }
        }

        region
    }
}

#[derive(Default)]
struct Region {
    points: FxHashSet<(i16, i16)>,
}

impl Region {
    fn area(&self) -> usize {
        self.points.len()
    }

    fn perimeter(&self) -> usize {
        self.points
            .iter()
            .map(|p| 4 - p.adj().filter(|adj| self.points.contains(adj)).count())
            .sum::<usize>()
    }

    fn sides(&self) -> usize {
        let mut wall = FxHashSet::default();

        for p in &self.points {
            for dir in DIRS {
                if !self.points.contains(&(p.0 + dir.0, p.1 + dir.1)) {
                    wall.insert((*p, (dir.1, -dir.0)));
                }
            }
        }

        wall.iter()
            .filter(|(p, dir)| !wall.contains(&((p.0 + dir.0, p.1 + dir.1), *dir)))
            .count()
    }
}

#[aoc(2024, 12)]
pub fn main() {
    let data = aoc_input!(2024, 12).unwrap();
    let garden = Garden::from_str(&data).unwrap();
    let regions = garden.regions();
    let (mut cost1, mut cost2) = (0, 0);

    for region in regions {
        // Part I
        cost1 += region.area() * region.perimeter();
        // Part II
        cost2 += region.area() * region.sides();
    }

    println!("{cost1}");
    println!("{cost2}");
}
