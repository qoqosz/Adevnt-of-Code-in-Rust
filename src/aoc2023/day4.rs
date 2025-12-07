use aoc::aoc_input;
use rustc_hash::FxHashMap;
use std::str::FromStr;

#[derive(Debug)]
struct Card {
    // Card id
    id: usize,
    // Count of winning numbers
    count: usize,
}

impl FromStr for Card {
    type Err = std::num::ParseIntError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (header, content) = line.split_once(':').unwrap();
        let id = header.trim_start_matches("Card").trim().parse::<usize>()?;
        let (winning, owned) = content.split_once('|').unwrap();

        let winning_numbers = winning
            .split(' ')
            .flat_map(|n| n.trim().parse::<u8>())
            .collect::<Vec<_>>();
        let owned_numbers = owned.split(' ').flat_map(|n| n.trim().parse::<u8>());

        Ok(Card {
            id,
            count: owned_numbers
                .filter(|n| winning_numbers.contains(n))
                .count(),
        })
    }
}

impl Card {
    // Score for Part I
    fn score(&self) -> u32 {
        match self.count as u32 {
            0 => 0,
            n => 2_u32.pow(n - 1),
        }
    }
}

// Score for Part II
fn recursive_score(id: usize, cards: &[Card], cache: &mut FxHashMap<usize, u32>) -> u32 {
    if let Some(score) = cache.get(&id) {
        return *score;
    }

    let card = &cards[id - 1];
    let res = 1
        + (1..=card.count)
            .map(|i| recursive_score(id + i, cards, cache))
            .sum::<u32>();

    *cache.entry(id).or_insert(res)
}

fn parse(data: &str) -> Vec<&str> {
    data.lines().filter(|x| !x.is_empty()).collect()
}

pub fn main() {
    let data = aoc_input!(2023, 4).unwrap();
    let lines = parse(&data);
    let cards = lines
        .iter()
        .flat_map(|line| Card::from_str(line))
        .collect::<Vec<_>>();

    // Part I
    let score: u32 = cards.iter().map(|card| card.score()).sum();
    println!("{score}");

    // Part II
    let mut cache: FxHashMap<usize, u32> = FxHashMap::default();
    let recursive_score: u32 = cards
        .iter()
        .map(|card| recursive_score(card.id, &cards, &mut cache))
        .sum();
    println!("{recursive_score}");
}
