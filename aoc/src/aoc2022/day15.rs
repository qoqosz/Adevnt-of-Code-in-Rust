use aoc::{aoc, aoc_input};
use glam::I64Vec2 as Point;
use regex_lite::Regex;
use rustc_hash::FxHashSet;

fn dist(s: &Point, b: &Point) -> i64 {
    (s.x - b.x).abs() + (s.y - b.y).abs()
}

fn find(s: &Point, b: &Point, y: i64) -> FxHashSet<i64> {
    let d = dist(s, b);
    let d_prime = (s.y - y).abs();
    let mut res = FxHashSet::default();

    if d > d_prime {
        for i in (s.x)..=(s.x + d - d_prime) {
            res.insert(i);
        }
    }
    if d >= d_prime {
        for i in (s.x - d + d_prime)..=(s.x) {
            res.insert(i);
        }
    }

    res
}

fn iter_circle(x0: &Point, d: i64) -> FxHashSet<Point> {
    let mut visited = FxHashSet::default();

    for i in 0..=d {
        for s in [1, -1] {
            for t in [1, -1] {
                let p = Point::new(x0.x + t * (d - i), x0.y + i * s);

                if !(0 <= p.x && p.x <= 4_000_000 && 0 <= p.y && p.y <= 4_000_000) {
                    continue;
                }

                visited.insert(p);
            }
        }
    }

    visited
}

fn is_beacon(x: &Point, data: &[(Point, Point, i64)]) -> bool {
    for (s, _, d) in data {
        if dist(s, x) <= *d {
            return false;
        }
    }
    true
}

#[aoc(2022, 15)]
pub fn main() {
    let data = aoc_input!(2022, 15).unwrap();
    let re = Regex::new(r"-?\d+").unwrap();

    let points: Vec<_> = data
        .trim()
        .lines()
        .map(|line| {
            re.find_iter(line)
                .filter_map(|num| num.as_str().parse::<i64>().ok())
                .collect::<Vec<_>>()
        })
        .map(|v| (Point::new(v[0], v[1]), Point::new(v[2], v[3])))
        .collect();

    // Part I
    let mut res = FxHashSet::default();
    let target_y = 2_000_000;
    let beacons_on_target = points
        .iter()
        .filter(|(_, b)| b.y == target_y)
        .map(|(_, b)| b.x)
        .collect::<FxHashSet<_>>();

    for (s, b) in &points {
        res.extend(find(s, b, target_y));
    }

    println!("{}", res.difference(&beacons_on_target).count());

    // Part II
    let data_w_dist = points
        .iter()
        .map(|(s, b)| (*s, *b, dist(s, b)))
        .collect::<Vec<_>>();

    'outer: for (s, _, d) in &data_w_dist {
        for x in iter_circle(s, d + 1) {
            if is_beacon(&x, &data_w_dist) {
                println!("{}", x.x * 4_000_000 + x.y);
                break 'outer;
            }
        }
    }
}
