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
        let start = self.src;
        let end = self.src + self.length;

        if (start..end).contains(&num) {
            let diff = num - start;
            return Some(self.dest + diff);
        }
        None
    }

    fn inv(&self) -> Self {
        Map {
            dest: self.src,
            src: self.dest,
            length: self.length,
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
            maps: self.maps.iter().map(|m| m.inv()).collect::<Vec<_>>(),
        }
    }
}

fn part1(seeds: &[u64], mappings: &[Mapping]) -> String {
    let mut seeds = seeds.to_vec();

    for category_mapping in mappings {
        seeds = seeds
            .iter()
            .map(|seed| category_mapping.convert(*seed))
            .collect::<Vec<_>>();
    }

    let sol = seeds.iter().min().unwrap();
    format!("{sol}")
}

fn part2(seeds: &[u64], mappings: &[Mapping]) -> String {
    let inv_maps = mappings.iter().rev().map(|m| m.inv()).collect::<Vec<_>>();

    for seed in 0..u64::MAX {
        let mut num = seed;

        for category_mapping in &inv_maps {
            num = category_mapping.convert(num);
        }

        for win in seeds.chunks(2) {
            if win[0] <= num && num < win[0] + win[1] {
                return format!("{seed}");
            }
        }
    }
    unreachable!()
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

#[aoc(2023, 5)]
pub fn main() {
    let data = aoc_input!(2023, 5).unwrap();
    let (seeds, mappings) = parse(&data);

    // Part I
    println!("{}", part1(&seeds, &mappings));

    // Part II
    println!("{}", part2(&seeds, &mappings));
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE1: &str = "seeds: 79 14 55 13

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
        let (seeds, mappings) = parse(EXAMPLE1);
        assert_eq!(part1(&seeds, &mappings), "35");
    }

    #[test]
    fn test_part2() {
        let (seeds, mappings) = parse(EXAMPLE1);
        assert_eq!(part2(&seeds, &mappings), "46");
    }
}
