use aoc::{aoc, aoc_input};
use std::str::FromStr;

#[derive(Debug, Default)]
struct Set {
    green: u32,
    blue: u32,
    red: u32,
}

#[derive(Debug, PartialOrd, PartialEq, Eq)]
struct ParseError;

impl FromStr for Set {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut set = Self::default();

        for word in s.trim().split(',') {
            for color in ["green", "blue", "red"] {
                if word.contains(color) {
                    let value = word.replace(color, "").trim().parse::<u32>().unwrap();
                    set.set_value(color, value);
                }
            }
        }

        Ok(set)
    }
}

impl Set {
    fn set_value(&mut self, color: &str, value: u32) {
        match color.trim() {
            "green" => self.green = value,
            "blue" => self.blue = value,
            _ => self.red = value,
        }
    }

    fn contains(&self, other: &Set) -> bool {
        self.green >= other.green && self.red >= other.red && self.blue >= other.blue
    }

    fn power(&self) -> u32 {
        self.green * self.blue * self.red
    }
}

#[derive(Debug, Default)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

impl FromStr for Game {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (id, sets) = line.split_once(':').unwrap();
        Ok(Game {
            id: id.replace("Game", "").trim().parse::<u32>().unwrap(),
            sets: sets.split(';').flat_map(Set::from_str).collect::<Vec<_>>(),
        })
    }
}

impl Game {
    fn is_possible(&self, bag: &Set) -> bool {
        self.sets.iter().all(|set| bag.contains(set))
    }

    fn _get_max(&self, f: impl Fn(&Set) -> u32) -> u32 {
        self.sets.iter().map(f).max().unwrap_or(0)
    }

    fn minimum_set(&self) -> Set {
        Set {
            green: self._get_max(|s: &Set| s.green),
            blue: self._get_max(|s: &Set| s.blue),
            red: self._get_max(|s: &Set| s.red),
        }
    }
}

fn parse(data: &str) -> Vec<&str> {
    data.lines().filter(|x| !x.is_empty()).collect()
}

#[aoc(2023, 2)]
pub fn main() {
    let data = aoc_input!(2023, 2).unwrap();
    let lines = parse(&data);

    let bag = Set {
        red: 12,
        green: 13,
        blue: 14,
    };

    let games = lines.iter().flat_map(|x| x.parse()).collect::<Vec<Game>>();

    // Part I
    let sum_ids = games
        .iter()
        .filter(|game| game.is_possible(&bag))
        .map(|game| game.id)
        .sum::<u32>();

    println!("{sum_ids}");

    // Part II
    let power_sum = games
        .iter()
        .map(|game| game.minimum_set().power())
        .sum::<u32>();

    println!("{power_sum}");
}
