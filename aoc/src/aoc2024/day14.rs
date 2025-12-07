use aoc::{aoc, aoc_input};
use std::str::FromStr;

static X: i32 = 101;
static Y: i32 = 103;

struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl FromStr for Robot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let extract = |x: &str| {
            let (a, b) = x[2..].split_once(',').unwrap();
            (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap())
        };

        let (sp, sv) = s.trim().split_once(' ').unwrap();
        let (x, y) = extract(sp);
        let (vx, vy) = extract(sv);

        Ok(Self { x, y, vx, vy })
    }
}

impl Robot {
    fn get_pos(&self, t: i32) -> (i32, i32) {
        (
            (self.x + self.vx * t).rem_euclid(X),
            (self.y + self.vy * t).rem_euclid(Y),
        )
    }
}

fn safety_factor(pos: &[(i32, i32)]) -> usize {
    let q1 = pos.iter().filter(|&&(x, y)| x < X / 2 && y < Y / 2).count();
    let q2 = pos.iter().filter(|&&(x, y)| x > X / 2 && y < Y / 2).count();
    let q3 = pos.iter().filter(|&&(x, y)| x < X / 2 && y > Y / 2).count();
    let q4 = pos.iter().filter(|&&(x, y)| x > X / 2 && y > Y / 2).count();

    q1 * q2 * q3 * q4
}

fn variance(pos: &[(i32, i32)]) -> f64 {
    let n = pos.len();
    let total = pos.iter().fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));
    let mean = (total.0 as f64 / n as f64, total.1 as f64 / n as f64);
    let variance = pos
        .iter()
        .map(|x| (x.0 as f64 - mean.0, x.1 as f64 - mean.1))
        .map(|x| x.0 * x.0 + x.1 * x.1)
        .sum::<f64>()
        / (n as f64);

    variance
}

#[aoc(2024, 14)]
pub fn main() {
    let data = aoc_input!(2024, 14).unwrap();
    let robots: Vec<_> = data.trim().lines().flat_map(Robot::from_str).collect();

    // Part I
    let pos: Vec<_> = robots.iter().map(|r| r.get_pos(100)).collect();
    println!("{}", safety_factor(&pos));

    // Part II
    let (mut i_egg, mut min_var) = (0, f64::MAX);

    for i in 1..100_000 {
        let pos: Vec<_> = robots.iter().map(|r| r.get_pos(i)).collect();
        let var = variance(&pos);

        if var < min_var {
            min_var = var;
            i_egg = i;
        }
    }
    println!("{i_egg}");
}
