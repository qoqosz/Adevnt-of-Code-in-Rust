use aoc::{aoc, aoc_input};
use itertools::Itertools;
use regex_lite::Regex;
use rustc_hash::FxHashMap;

#[derive(Debug)]
struct Reindeer {
    name: String,
    v0: u32,
    dt0: u32,
    t: u32,
}

impl Reindeer {
    fn dist(&self, t: u32) -> u32 {
        let (n, tau) = (t / self.t, t % self.t);
        n * self.v0 * self.dt0 + self.v0 * std::cmp::min(self.dt0, tau)
    }
}

impl From<&str> for Reindeer {
    fn from(line: &str) -> Self {
        let name = line.split(' ').next().unwrap();
        let re: Regex = Regex::new(r"\d+").unwrap();

        let nums: Vec<_> = re
            .find_iter(line)
            .flat_map(|d| d.as_str().parse::<u32>())
            .collect();

        Reindeer {
            name: name.to_string(),
            v0: nums[0],
            dt0: nums[1],
            t: nums[1] + nums[2],
        }
    }
}

#[aoc(2015, 14)]
pub fn main() {
    let data = aoc_input!(2015, 14).unwrap();
    let reindeers = data
        .lines()
        .filter(|x| !x.is_empty())
        .map(Reindeer::from)
        .collect::<Vec<_>>();

    // Part I
    let t = 2503;
    let max_dist = reindeers.iter().map(|r| r.dist(t)).max().unwrap();
    println!("{}", max_dist);

    // Part II
    let mut scores: FxHashMap<&String, u32> = FxHashMap::default();

    for t in 1..=t {
        let leaders = reindeers
            .iter()
            .map(|r| (r.dist(t), &r.name))
            .max_set_by_key(|k| k.0);

        for (_, l) in leaders {
            scores.entry(l).and_modify(|s| *s += 1).or_insert(1);
        }
    }

    println!("{}", scores.values().max().unwrap());
}
