use aoc::aoc_input;
use std::str::FromStr;

#[derive(Debug, Default)]
struct Set {
    green: u32,
    blue: u32,
    red: u32,
}

#[derive(Debug, PartialOrd, PartialEq, Eq)]
struct ParseSetError;

impl FromStr for Set {
    type Err = ParseSetError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut game = Self::default();

        for color in s.trim().split(',') {
            if color.contains(" green") {
                game.green = color.trim().replace(" green", "").parse::<u32>().unwrap();
            }
            if color.contains(" blue") {
                game.blue = color.trim().replace(" blue", "").parse::<u32>().unwrap();
            }
            if color.contains(" red") {
                game.red = color.trim().replace(" red", "").parse::<u32>().unwrap();
            }
        }
        Ok(game)
    }
}

impl Set {
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

#[derive(Debug, PartialOrd, PartialEq, Eq)]
struct ParseGameError;

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (id, sets) = line.split_once(':').unwrap();
        let game = Game {
            id: id.replace("Game", "").trim().parse::<u32>().unwrap(),
            sets: sets.split(';').flat_map(Set::from_str).collect::<Vec<_>>(),
        };

        Ok(game)
    }
}

impl Game {
    fn is_possible(&self, bag: &Set) -> bool {
        self.sets.iter().all(|set| bag.contains(set))
    }

    fn minimum_set(&self) -> Set {
        let green = self.sets.iter().map(|s| s.green).max().unwrap_or(0);
        let blue = self.sets.iter().map(|s| s.blue).max().unwrap_or(0);
        let red = self.sets.iter().map(|s| s.red).max().unwrap_or(0);

        Set { green, blue, red }
    }
}

fn parse(data: &str) -> Vec<&str> {
    data.lines().filter(|x| !x.is_empty()).collect()
}

pub fn main() {
    let data = aoc_input!(2023, 2).unwrap();
    let lines = parse(&data);

    let bag = Set {
        red: 12,
        green: 13,
        blue: 14,
    };

    let games = lines
        .iter()
        .flat_map(|x| Game::from_str(x))
        .collect::<Vec<_>>();

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
