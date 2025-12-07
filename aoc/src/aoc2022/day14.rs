use aoc::{aoc, aoc_input};
use glam::i32::IVec2 as Point;
use itertools::Itertools;
use rustc_hash::FxHashSet;
use std::cmp::{max, min};

enum LineType {
    Horizontal,
    Vertical,
}

struct Line {
    start: Point,
    end: Point,
    typ: LineType,
}

impl Line {
    fn create(start: Point, end: Point) -> Self {
        let typ = if start.x == end.x {
            LineType::Vertical
        } else {
            LineType::Horizontal
        };

        Self { start, end, typ }
    }

    fn minx(&self) -> i32 {
        min(self.start.x, self.end.x)
    }

    fn maxx(&self) -> i32 {
        max(self.start.x, self.end.x)
    }

    fn miny(&self) -> i32 {
        min(self.start.y, self.end.y)
    }

    fn maxy(&self) -> i32 {
        max(self.start.y, self.end.y)
    }

    fn points(&self) -> FxHashSet<Point> {
        match self.typ {
            LineType::Horizontal => {
                let (a, b) = (self.minx(), self.maxx());

                (a..=b).map(move |i| Point::new(i, self.start.y)).collect()
            }
            LineType::Vertical => {
                let (a, b) = (self.miny(), self.maxy());

                (a..=b).map(move |i| Point::new(self.start.x, i)).collect()
            }
        }
    }
}

struct Path {
    lines: Vec<Line>,
}

impl Path {
    fn create(points: impl IntoIterator<Item = Point>) -> Self {
        let lines: Vec<_> = points
            .into_iter()
            .tuple_windows()
            .map(|(p, q)| Line::create(p, q))
            .collect();

        Self { lines }
    }

    fn points(&self) -> FxHashSet<Point> {
        self.lines.iter().flat_map(|line| line.points()).collect()
    }
}

struct Cave {
    rocks: FxHashSet<Point>,
    source: Point,
    max_depth: i32,
    prevs: Vec<Point>,
}

impl Cave {
    fn create(paths: impl IntoIterator<Item = Path>, source: Point, max_depth: i32) -> Self {
        Self {
            rocks: paths.into_iter().flat_map(|path| path.points()).collect(),
            source,
            max_depth,
            prevs: vec![source],
        }
    }

    fn is_air(&self, point: &Point) -> bool {
        !self.rocks.contains(point)
    }

    fn pour(&mut self) -> Option<Point> {
        let mut source = if let Some(&x) = self.prevs.last() {
            x
        } else {
            self.source
        };

        loop {
            if source.y >= self.max_depth {
                return None;
            }

            let dts = [Point::new(0, 1), Point::new(-1, 1), Point::new(1, 1)];
            let mut move_on = false;

            for dt in dts {
                let dest = source + dt;

                if self.is_air(&dest) {
                    source += dt;
                    move_on = true;
                    self.prevs.push(dest);
                    break;
                }
            }

            if move_on {
                continue;
            }

            self.rocks.insert(self.prevs.pop().unwrap());
            break;
        }
        Some(source)
    }

    fn find(&mut self) -> Option<usize> {
        (0..).find(|_| {
            let filled = self.pour();
            filled.is_none() || filled == Some(self.source)
        })
    }
}

#[aoc(2022, 14)]
pub fn main() {
    let data = aoc_input!(2022, 14).unwrap();

    let sand = Point::new(500, 0);
    let paths = data.trim().lines().map(|line| {
        Path::create(line.split(" -> ").map(|p| {
            let (x, y) = p.split_once(',').unwrap();
            Point::new(x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
        }))
    });

    // Part I
    let mut cave = Cave::create(paths.clone(), sand, 200);
    println!("{}", cave.find().unwrap());

    // Part II
    let floor_y = 2 + paths
        .clone()
        .filter_map(|path| path.points().iter().map(|point| point.y).max())
        .max()
        .unwrap();
    let floor = Path::create([Point::new(-100_000, floor_y), Point::new(100_000, floor_y)]);
    let mut cave = Cave::create(paths.chain([floor]), sand, floor_y);
    println!("{}", cave.find().map(|n| n + 1).unwrap());
}
