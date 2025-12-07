use aoc::{aoc, aoc_input};
use glam::I64Vec2 as Coord;
use regex_lite::Regex;
use std::str::FromStr;
use std::sync::LazyLock;

static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d+").unwrap());
static EPSILON: f64 = 1e-6;

struct Machine {
    a: Coord,
    b: Coord,
    prize: Coord,
}

impl Machine {
    fn shift(&self, ds: i64) -> Self {
        Self {
            prize: self.prize + Coord::new(ds, ds),
            ..*self
        }
    }

    fn solve(&self) -> Option<usize> {
        let (a, b, p) = (self.a.as_dvec2(), self.b.as_dvec2(), self.prize.as_dvec2());
        let det = a.x * b.y - a.y * b.x;

        if det.abs() < EPSILON {
            return None;
        }

        let a_coeff = (b.y * p.x - b.x * p.y) / det;
        let b_coeff = (a.x * p.y - a.y * p.x) / det;

        // solution must be a non-negative integer
        if (a_coeff < 0.0)
            || (b_coeff < 0.0 || a_coeff.fract() > EPSILON || b_coeff.fract() > EPSILON)
        {
            None
        } else {
            Some((3.0 * a_coeff + b_coeff) as usize)
        }
    }
}

impl FromStr for Machine {
    type Err = ();

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        let nums: Vec<i64> = RE
            .find_iter(data)
            .filter_map(|digits| digits.as_str().parse().ok())
            .collect();
        Ok(Machine {
            a: Coord::new(nums[0], nums[1]),
            b: Coord::new(nums[2], nums[3]),
            prize: Coord::new(nums[4], nums[5]),
        })
    }
}

#[aoc(2024, 13)]
pub fn main() {
    let data = aoc_input!(2024, 13).unwrap();
    let machines = data
        .trim()
        .split("\n\n")
        .flat_map(Machine::from_str)
        .collect::<Vec<_>>();

    // Part I
    let tokens = machines.iter().filter_map(|m| m.solve()).sum::<usize>();
    println!("{tokens}");

    // Part II
    let tokens = machines
        .iter()
        .filter_map(|m| m.shift(10000000000000).solve())
        .sum::<usize>();
    println!("{tokens}");
}
