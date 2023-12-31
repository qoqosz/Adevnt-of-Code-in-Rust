#![allow(non_snake_case)]
/// Part 2
///
/// (r, v) - unknonwn
///
/// Because
///  r_i + v_i t = r + v t => (r_i - r) = (v - v_i) t
///
/// we have
///  0 = (r - r_i) ^ (v - v_i)
///
/// r ^ v = const, so from a pair of eqs for i and j:
///
///  0 = r ^ (v_j - v_i) ^ (r_j - r_i) + r_i ^ v_i ^ r_j + r_j ^ v_j ^ r_i
///
/// Having 3 such eqs allow to solve for all coordinates of r
// Debug:
// 20361
// 558415252330828
// Elapsed: 34ms

// Day 24, 2023
// 20361
// 558415252330828
use aoc::{aoc, aoc_input};
use itertools::Itertools;
use std::fmt;

type Vec3 = (f64, f64, f64);

struct Hailstone {
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
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
        (self.x + self.vx * t, self.y + self.vy * t)
    }

    fn cross(&self, other: &Hailstone) -> f64 {
        let num = (other.y - self.y) + other.vy / other.vx * (self.x - other.x);
        let den = self.vy - other.vy / other.vx * self.vx;

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
            .map(|x| x.trim().parse::<f64>().unwrap())
            .tuples::<(_, _, _)>()
            .next()
            .unwrap();
        let (vx, vy, vz) = vel
            .split(',')
            .map(|x| x.trim().parse::<f64>().unwrap())
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
        }
    }

    n
}

// TODO: Implement Vec3d struct + all needed funcs
fn multiply(a: f64, u: Vec3) -> Vec3 {
    (a * u.0, a * u.1, a * u.2)
}

fn add(u: Vec3, v: Vec3) -> Vec3 {
    (u.0 + v.0, u.1 + v.1, u.2 + v.2)
}

fn subtract(u: Vec3, v: Vec3) -> Vec3 {
    (u.0 - v.0, u.1 - v.1, u.2 - v.2)
}

fn dot(u: Vec3, v: Vec3) -> f64 {
    u.0 * v.0 + u.1 * v.1 + u.2 * v.2
}

fn cross(v: Vec3, w: Vec3) -> Vec3 {
    (
        v.1 * w.2 - v.2 * w.1,
        v.2 * w.0 - v.0 * w.2,
        v.0 * w.1 - v.1 * w.0,
    )
}

// Coefficients in a row of: Ax = b
fn get_coeffs(ri: Vec3, vi: Vec3, rj: Vec3, vj: Vec3) -> (Vec3, f64) {
    let a = cross(subtract(vj, vi), subtract(rj, ri));
    let b = dot(ri, cross(vi, rj)) + dot(rj, cross(vj, ri));
    (a, -b)
}

fn gauss_elim(A: &[Vec3; 3], b: Vec3) -> Vec3 {
    let mut A: [Vec3; 3] = *A;
    let mut b = b;

    // 1st round
    let (a0, a1, a2) = (A[0].0, A[1].0, A[2].0);
    A[1] = subtract(A[1], multiply(a1 / a0, A[0]));
    A[2] = subtract(A[2], multiply(a2 / a0, A[0]));
    b.1 -= a1 / a0 * b.0;
    b.2 -= a2 / a0 * b.0;

    // 2nd round
    let (b1, b2) = (A[1].1, A[2].1);
    A[2] = subtract(A[2], multiply(b2 / b1, A[1]));
    b.2 -= b2 / b1 * b.1;

    // solve
    let z = b.2 / A[2].2;
    let y = (b.1 - A[1].2 * z) / A[1].1;
    let x = (b.0 - A[0].1 * y - A[0].2 * z) / A[0].0;

    (x, y, z)
}

fn find_single_pos(r: &[Vec3; 6], v: &[Vec3; 6]) -> Vec3 {
    let (A0, b0) = get_coeffs(r[0], v[0], r[1], v[1]);
    let (A1, b1) = get_coeffs(r[2], v[2], r[3], v[3]);
    let (A2, b2) = get_coeffs(r[4], v[4], r[5], v[5]);

    let A = [A0, A1, A2];
    let b = (b0, b1, b2);

    gauss_elim(&A, b)
}

fn find_pos(hailstones: &[Hailstone]) -> Option<u64> {
    let results = hailstones
        .windows(6)
        .map(|w| {
            let (mut r, mut v) = (vec![], vec![]);
            for h in w {
                r.push((h.x, h.y, h.z));
                v.push((h.vx, h.vy, h.vz));
            }
            find_single_pos(&r.try_into().unwrap(), &v.try_into().unwrap())
        })
        .take(30)
        .collect::<Vec<Vec3>>();

    // average over 30 hailstones
    let r0 = *results.first()?;
    let adj: Vec3 = results[1..].iter().map(|&r| subtract(r0, r)).reduce(add)?;

    let res = subtract(r0, multiply(1.0 / results.len() as f64, adj));

    Some(res.0.round() as u64 + res.1.round() as u64 + res.2.round() as u64)
}

#[aoc(2023, 24)]
pub fn main() {
    let data = aoc_input!(2023, 24).unwrap();
    let hailstones = parse(&data);

    // Part I
    let min = 200_000_000_000_000.0;
    let max = 400_000_000_000_000.0;
    println!("{}", count_intersections2d(&hailstones, min, max));

    // Part II
    let r = find_pos(&hailstones).unwrap();
    println!("{}", r);
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
        assert_eq!(z, 2);
    }

    #[test]
    fn test_multiply() {
        let v = (13.0, 0.0, -9.0);
        let v2 = multiply(2.0, v);
        assert_eq!(v2, (26.0, 0.0, -18.0));
    }

    #[test]
    fn test_subtract() {
        let u = (90.0, -18.0, 7.0);
        let v = (20.0, 16.6, 3.4);
        let expected = (70.0, -34.6, 3.6);
        assert_eq!(subtract(u, v), expected);
    }

    #[test]
    fn test_dot() {
        let u = (17.0, 3.4, 0.0);
        let v = (9.1, 2.9, 2.3);
        let expected = 164.56;
        assert_eq!(dot(u, v), expected);
    }

    #[test]
    fn test_cross() {
        let u = (1.0, 2.0, 3.0);
        let v = (4.0, 5.0, 6.0);
        let expected = (-3.0, 6.0, -3.0);
        assert_eq!(cross(u, v), expected);
    }

    // Solve[{3x+2y+z==39, 2x+3y+z==34, x+2y+3z=26}, {x, y, z}]
    // x = 37/4 and y = 17/4 and z = 11/4
    #[test]
    fn test_gauss_elim() {
        let A = [(3.0, 2.0, 1.0), (2.0, 3.0, 1.0), (1.0, 2.0, 3.0)];
        let b = (39.0, 34.0, 26.0);
        let expected = (9.25, 4.25, 2.75);
        assert_eq!(gauss_elim(&A, b), expected);
    }
}
