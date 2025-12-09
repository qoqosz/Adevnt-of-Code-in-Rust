use aoc::{aoc, aoc_input};
use itertools::Itertools;
use std::num::ParseIntError;

struct Point {
    x: u64,
    y: u64,
}

impl TryFrom<&str> for Point {
    type Error = ParseIntError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (x, y) = value.split_once(',').unwrap();

        Ok(Self {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

/// Rectangle spanned by two corners in a cannonical form, i.e. such that:
///   (p.x, p.y) <= (q.x, q.y)
struct Rectangle {
    p: Point,
    q: Point,
}

impl Rectangle {
    /// Create a new rectangle.
    fn from_corners(p: &Point, q: &Point) -> Self {
        Self {
            p: Point {
                x: p.x.min(q.x),
                y: p.y.min(q.y),
            },
            q: Point {
                x: p.x.max(q.x),
                y: p.y.max(q.y),
            },
        }
    }

    /// Area of a rectangle.
    fn area(&self) -> u64 {
        (self.q.x - self.p.x + 1) * (self.q.y - self.p.y + 1)
    }

    /// Check if a rectangle intersects with other.
    /// Uses the AABB (Axis-Aligned Bounding Box) algorithm.
    fn intersects(&self, other: &Self) -> bool {
        other.p.x < self.q.x && other.p.y < self.q.y && other.q.x > self.p.x && other.q.y > self.p.y
    }
}

#[aoc(2025, 9)]
pub fn main() {
    let data = aoc_input!(2025, 9).unwrap();
    let red_tiles = data.lines().flat_map(Point::try_from).collect::<Vec<_>>();

    let (mut a, mut b) = (0, 0);

    'outer: for (u, v) in red_tiles.iter().tuple_combinations() {
        let rect = Rectangle::from_corners(u, v);
        let area = rect.area();

        a = a.max(area);

        for (p, q) in red_tiles.iter().circular_tuple_windows() {
            if Rectangle::from_corners(p, q).intersects(&rect) {
                continue 'outer;
            }
        }

        b = b.max(area);
    }

    // Part I
    println!("{a}");

    // Part II
    println!("{b}");
}
