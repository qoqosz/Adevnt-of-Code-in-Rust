use aoc::{aoc, aoc_input};
use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::{cmp::Ordering, marker::PhantomData};

/// Part I type of game.
#[derive(Debug, Clone)]
struct RegularGame {}

/// Part II type of game.
#[derive(Debug, Clone)]
struct JokerGame {}

trait GameType {
    /// Joker strength for cards ordering.
    fn joker_strength() -> u8;

    /// Count cards of the same type to determine HandType.
    fn count(hand: &str) -> FxHashMap<char, usize>;
}

impl GameType for RegularGame {
    fn joker_strength() -> u8 {
        100
    }

    fn count(hand: &str) -> FxHashMap<char, usize> {
        hand.chars()
            .fold(FxHashMap::default(), |mut counter, card| {
                *counter.entry(card).or_insert(0) += 1;
                counter
            })
    }
}
impl GameType for JokerGame {
    fn joker_strength() -> u8 {
        1
    }

    fn count(hand: &str) -> FxHashMap<char, usize> {
        let mut counter = hand
            .chars()
            .fold(FxHashMap::default(), |mut counter, card| {
                *counter.entry(card).or_insert(0) += 1;
                counter
            });
        if let Some(n_jokers) = counter.remove(&'J') {
            let max_key = counter
                .iter()
                .max_by(|a, b| a.1.cmp(b.1))
                .map(|(k, _)| k)
                .unwrap_or(&'J');
            *counter.entry(*max_key).or_insert(0) += n_jokers;
        }
        counter
    }
}

trait CardStrength {
    /// Helper function for card ordering.
    fn card_strength<G>(&self) -> u8
    where
        G: GameType;
}

impl CardStrength for char {
    fn card_strength<G>(&self) -> u8
    where
        G: GameType,
    {
        match self {
            'A' => 200,
            'K' => 190,
            'Q' => 180,
            'J' => G::joker_strength(),
            'T' => 90,
            ch @ '2'..='9' => *ch as u8,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
struct Hand<'a, G = RegularGame>
where
    G: GameType,
{
    cards: &'a str,
    typ: PhantomData<G>,
}

impl<'a, G> From<&'a str> for Hand<'a, G>
where
    G: GameType,
{
    fn from(cards: &'a str) -> Self {
        assert_eq!(cards.len(), 5);
        Hand {
            cards,
            typ: PhantomData,
        }
    }
}

impl<'a, G> Hand<'a, G>
where
    G: GameType,
{
    fn as_vec(&'a self) -> Vec<u8> {
        self.cards.chars().map(|c| c.card_strength::<G>()).collect()
    }

    fn count(&self) -> FxHashMap<char, usize> {
        G::count(self.cards)
    }

    fn hand_type(&self) -> HandType {
        let counts = self
            .count()
            .values()
            .sorted()
            .rev()
            .copied()
            .collect::<Vec<_>>();

        match counts.as_slice() {
            [5] => HandType::FiveOfAKind,
            [4, 1] => HandType::FourOfAKind,
            [3, 2] => HandType::FullHouse,
            [3, 1, 1] => HandType::ThreeOfAKind,
            [2, 2, 1] => HandType::TwoPair,
            [2, 1, 1, 1] => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

impl<'a, G> PartialEq for Hand<'a, G>
where
    G: GameType,
{
    fn eq(&self, other: &Self) -> bool {
        self.cards.eq(other.cards)
    }
}

impl<'a, G> Eq for Hand<'a, G> where G: GameType {}

impl<'a, G> Ord for Hand<'a, G>
where
    G: GameType,
{
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type().cmp(&other.hand_type()) {
            Ordering::Equal => self.as_vec().cmp(&other.as_vec()),
            cmp => cmp,
        }
    }
}

impl<'a, G> PartialOrd for Hand<'a, G>
where
    G: GameType,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

fn parse<G>(data: &str) -> (Vec<Hand<'_, G>>, Vec<u64>)
where
    G: GameType,
{
    let (mut hands, mut bids) = (vec![], vec![]);

    for line in data.lines().filter(|x| !x.is_empty()) {
        let (hand, bid) = line.split_once(' ').unwrap();
        hands.push(Hand::from(hand));
        bids.push(bid.parse::<u64>().unwrap())
    }

    (hands, bids)
}

fn score<G>(hands: &[Hand<'_, G>], bids: &[u64]) -> u64
where
    G: GameType,
{
    std::iter::zip(hands, bids)
        .sorted_by(|(lhs, _), (rhs, _)| lhs.cmp(rhs))
        .enumerate()
        .map(|(rank, (_, bid))| (rank as u64 + 1) * bid)
        .sum::<u64>()
}

#[aoc(2023, 7)]
pub fn main() {
    let data = aoc_input!(2023, 7).unwrap();

    // Part I
    let (hands, bids) = parse::<RegularGame>(&data);
    println!("{}", score(&hands, &bids));

    // Part II
    let (hands, bids) = parse::<JokerGame>(&data);
    println!("{}", score(&hands, &bids));
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE1: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    static EXAMPLE2: &str = "2345A 1
Q2KJJ 13
Q2Q2Q 19
T3T3J 17
T3Q33 11
2345J 3
J345A 2
32T3K 5
T55J5 29
KK677 7
KTJJT 34
QQQJA 31
JJJJJ 37
JAAAA 43
AAAAJ 59
AAAAA 61
2AAAA 23
2JJJJ 53
JJJJ2 41";

    #[test]
    fn test_hand_types() {
        let (hands, _) = parse::<RegularGame>(EXAMPLE1);
        let hand_types = hands.iter().map(|h| h.hand_type()).collect::<Vec<_>>();
        let expected = vec![
            HandType::OnePair,
            HandType::ThreeOfAKind,
            HandType::TwoPair,
            HandType::TwoPair,
            HandType::ThreeOfAKind,
        ];

        assert_eq!(hand_types, expected);
    }

    #[test]
    fn test_card_strength() {
        let mut cards = [
            '9', '8', '7', '6', '5', '4', '3', '2', 'A', 'T', 'K', 'Q', 'J',
        ];
        cards.sort_by_key(|c| c.card_strength::<RegularGame>());
        let expected = [
            '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
        ];
        assert_eq!(cards, expected);
    }

    #[test]
    fn test_strength_order() {
        let (hands, _) = parse::<RegularGame>(EXAMPLE1);
        let sorted = hands.iter().sorted().cloned().collect::<Vec<_>>();
        let expected = vec![
            Hand::from("32T3K"),
            Hand::from("KTJJT"),
            Hand::from("KK677"),
            Hand::from("T55J5"),
            Hand::from("QQQJA"),
        ];
        assert_eq!(sorted, expected);
    }

    #[test]
    fn test_ordering() {
        let four_of_a_kind = HandType::FourOfAKind;
        let two_pair = HandType::TwoPair;
        assert!(four_of_a_kind > two_pair);
        assert!(four_of_a_kind >= two_pair);
        assert!(four_of_a_kind >= four_of_a_kind);
        assert!(four_of_a_kind == four_of_a_kind);
    }

    #[test]
    fn test_edge_case() {
        let lhs: Hand<'_, RegularGame> = Hand::from("2345A");
        let rhs = Hand::from("2345J");
        assert!(lhs > rhs);
    }

    #[test]
    fn test_edge_case_hands() {
        let expected = &["2345J", "2345A", "J345A", "32T3K", "Q2KJJ"];
        let cards = &["2345A", "Q2KJJ", "2345J", "32T3K", "J345A"];

        assert_eq!(
            expected
                .iter()
                .map(|c| Hand::from(*c))
                .collect::<Vec<Hand<'_, RegularGame>>>(),
            cards
                .iter()
                .map(|c| Hand::from(*c))
                .sorted()
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_part1() {
        let (hands, bids): (Vec<Hand<'static>>, _) = parse(EXAMPLE2);

        for h in hands.iter().sorted() {
            println!("{:?}", h);
        }

        assert_eq!(score(&hands, &bids), 6592);
    }

    #[test]
    fn test_part2() {
        let (hands, bids): (Vec<Hand<'static, JokerGame>>, _) = parse(EXAMPLE2);

        for h in hands.iter().sorted() {
            println!("{:?} {:?}", h, h.hand_type());
        }

        assert_eq!(score(&hands, &bids), 6839);
    }
}
