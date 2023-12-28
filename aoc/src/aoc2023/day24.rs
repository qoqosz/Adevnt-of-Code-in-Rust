use aoc::{aoc, aoc_input};
use itertools::Itertools;
use std::fmt;

struct Hailstone {
    x: i64,
    y: i64,
    z: i64,
    vx: i64,
    vy: i64,
    vz: i64,
}

impl fmt::Debug for Hailstone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}, {}, {} @ {}, {}, {}]",
            self.x, self.y, self.z, self.vx, self.vy, self.vz
        )
    }
}

impl Hailstone {
    fn eval(&self, t: f64) -> (f64, f64) {
        (
            self.x as f64 + self.vx as f64 * t,
            self.y as f64 + self.vy as f64 * t,
        )
    }

    fn cross(&self, other: &Hailstone) -> f64 {
        let (a, b) = (self.x as f64, self.y as f64);
        let (x, y) = (self.vx as f64, self.vy as f64);
        let (p, w) = (other.x as f64, other.y as f64);
        let (q, r) = (other.vx as f64, other.vy as f64);

        let num = (w - b) + r / q * (a - p);
        let den = y - r / q * x;

        num / den
    }
}

fn parse(data: &str) -> Vec<Hailstone> {
    let mut hailstones = vec![];

    for (pos, vel) in data
        .trim()
        .lines()
        .filter(|x| !x.is_empty())
        .map(|line| line.split_once('@').unwrap())
    {
        let (x, y, z) = pos
            .split(',')
            .map(|x| x.trim().parse::<i64>().unwrap())
            .tuples::<(_, _, _)>()
            .next()
            .unwrap();
        let (vx, vy, vz) = vel
            .split(',')
            .map(|x| x.trim().parse::<i64>().unwrap())
            .tuples::<(_, _, _)>()
            .next()
            .unwrap();

        hailstones.push(Hailstone {
            x,
            y,
            z,
            vx,
            vy,
            vz,
        });
    }

    hailstones
}

fn count_intersections2d(hailstones: &[Hailstone], min: f64, max: f64) -> usize {
    let mut n = 0;
    for pair in hailstones.iter().combinations(2) {
        let left = pair[0];
        let right = pair[1];

        let t = left.cross(right);
        let s = right.cross(left);
        let (x, y) = left.eval(t);

        if min <= x && x <= max && min <= y && y <= max && t > 0.0 && s > 0.0 {
            n += 1;

            // println!("{:?}\n{:?}", left, right);
            // println!("{:?} {}, --- {}, {}", x, y, t, s);
            // println!("{:?}\n\n", right.eval(t));
        }
    }

    n
}

#[aoc(2023, 24)]
pub fn main() {
    let data = aoc_input!(2023, 24).unwrap();
    let hailstones = parse(&data);

    // Part I
    let min = 200000000000000.0;
    let max = 400000000000000.0;
    println!("{}", count_intersections2d(&hailstones, min, max));

    // Part II
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    #[test]
    fn test_part1() {
        let hailstones = parse(EXAMPLE);
        let z = count_intersections2d(&hailstones, 7.0, 27.0);
        println!("{}", z);
    }

    #[test]
    fn test_part2() {
        //        assert_eq!(part2(&lines), 1);
    }
}
