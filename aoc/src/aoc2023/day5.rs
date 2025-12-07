use aoc::{aoc, aoc_input};
use std::str::FromStr;

#[derive(Debug)]
struct Map {
    dest: u64,
    src: u64,
    length: u64,
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums = s.split_whitespace().flat_map(|x| x.parse::<u64>());

        Ok(Map {
            dest: nums.next().unwrap(),
            src: nums.next().unwrap(),
            length: nums.next().unwrap(),
        })
    }
}

impl Map {
    fn convert(&self, num: u64) -> Option<u64> {
        let (start, end) = (self.src, self.src + self.length);

        match (start..end).contains(&num) {
            false => None,
            true => {
                let diff = num - start;
                Some(self.dest + diff)
            }
        }
    }
}

#[derive(Debug)]
struct Mapping {
    maps: Vec<Map>,
}

impl FromStr for Mapping {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Mapping {
            maps: s
                .lines()
                .skip(1)
                .flat_map(Map::from_str)
                .collect::<Vec<_>>(),
        })
    }
}

impl Mapping {
    fn convert(&self, num: u64) -> u64 {
        self.maps.iter().find_map(|m| m.convert(num)).unwrap_or(num)
    }

    fn inv(&self) -> Self {
        Mapping {
            maps: self
                .maps
                .iter()
                .map(|m| Map {
                    src: m.dest,
                    dest: m.src,
                    length: m.length,
                })
                .collect::<Vec<_>>(),
        }
    }
}

fn parse(data: &str) -> (Vec<u64>, Vec<Mapping>) {
    let mut sections = data.split("\n\n");
    let seeds = sections
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .flat_map(|x| x.parse::<u64>())
        .collect();
    let mappings = sections.flat_map(Mapping::from_str).collect::<Vec<_>>();

    (seeds, mappings)
}

/// Project the input `seed` through the `mappings` to the final location.
#[inline]
fn project(seed: u64, mappings: &[Mapping]) -> u64 {
    mappings
        .iter()
        .fold(seed, |current_seed, mapping| mapping.convert(current_seed))
}

/// Run `project`ion on each seed and return `min`.
fn part1(seeds: &[u64], mappings: &[Mapping]) -> Option<u64> {
    seeds.iter().map(|seed| project(*seed, mappings)).min()
}

/// Check if `seed` falls in any of the `seeds` intervals.
#[inline]
fn is_seed(seeds: &[u64], seed: u64) -> bool {
    seeds.chunks(2).any(|win| {
        let (start, end) = (win[0], win[0] + win[1]);

        (start..end).contains(&seed)
    })
}

/// Iterate final `location`s and project them through the inverse
/// `mappings` to get the seed.
fn part2(seeds: &[u64], mappings: &[Mapping]) -> Option<u64> {
    let inv_maps = mappings.iter().rev().map(|m| m.inv()).collect::<Vec<_>>();

    (0..u64::MAX).find(|location| {
        let seed = project(*location, &inv_maps);
        is_seed(seeds, seed)
    })
}

#[aoc(2023, 5)]
pub fn main() {
    let data = aoc_input!(2023, 5).unwrap();
    let (seeds, mappings) = parse(&data);

    // Part I
    println!("{}", part1(&seeds, &mappings).unwrap());

    // Part II
    println!("{}", part2(&seeds, &mappings).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_part1() {
        let (seeds, mappings) = parse(EXAMPLE);
        assert_eq!(part1(&seeds, &mappings).unwrap(), 35);
    }

    #[test]
    fn test_part2() {
        let (seeds, mappings) = parse(EXAMPLE);
        assert_eq!(part2(&seeds, &mappings).unwrap(), 46);
    }
}
