use aoc::aoc_input;
use std::str::FromStr;

#[derive(Debug)]
struct Card {
    // Count of winning numbers
    count: usize,
}

impl FromStr for Card {
    type Err = std::num::ParseIntError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (_, content) = line.split_once(':').unwrap();
        let (winning, owned) = content.split_once('|').unwrap();

        let winning_numbers = winning
            .split(' ')
            .flat_map(|n| n.trim().parse::<u8>())
            .collect::<Vec<_>>();
        let owned_numbers = owned.split(' ').flat_map(|n| n.trim().parse::<u8>());

        Ok(Card {
            count: owned_numbers
                .filter(|n| winning_numbers.contains(n))
                .count(),
        })
    }
}

impl Card {
    // Score for Part I
    #[inline]
    fn score(&self) -> u32 {
        (1 << self.count) >> 1
    }
}

// Score for Part II
fn recursive_score(id: usize, cards: &[Card], cache: &mut [u32]) -> u32 {
    match cache.get(id) {
        None | Some(&u32::MAX) => {}
        Some(score) => return *score,
    }

    let card = &cards[id];
    let score = 1
        + (1..=card.count)
            .map(|i| recursive_score(id + i, cards, cache))
            .sum::<u32>();

    cache[id] = score;
    score
}

pub fn main() {
    let data = aoc_input!(2023, 4).unwrap();
    let cards = data.lines().flat_map(Card::from_str).collect::<Vec<_>>();

    // Part I
    let score: u32 = cards.iter().map(|card| card.score()).sum();
    println!("{score}");

    // Part II
    let mut cache: Vec<u32> = vec![u32::MAX; cards.len()];
    let recursive_score: u32 = cards
        .iter()
        .enumerate()
        .map(|(id, _)| recursive_score(id, &cards, &mut cache))
        .sum();
    println!("{recursive_score}");
}
