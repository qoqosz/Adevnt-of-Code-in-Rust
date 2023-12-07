use aoc::{aoc, aoc_input};
use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::cmp::Ordering;

trait CardStrength {
    fn card_strength(&self) -> usize;
}

impl CardStrength for char {
    fn card_strength(&self) -> usize {
        match self {
            'A' => 1_000_000,
            'K' => 100_000,
            'Q' => 10_000,
            'J' => 1_000,
            'T' => 100,
            ch @ '2'..='9' => *ch as usize,
            _ => 0,
        }
    }
}

#[derive(Debug, Clone)]
struct Hand<'a> {
    cards: &'a str,
}

impl<'a> From<&'a str> for Hand<'a> {
    fn from(cards: &'a str) -> Hand<'a> {
        Hand { cards }
    }
}

impl<'a> Hand<'a> {
    fn count(&self) -> FxHashMap<char, usize> {
        self.cards
            .chars()
            .fold(FxHashMap::default(), |mut counter, card| {
                *counter.entry(card).or_insert(0) += 1;
                counter
            })
    }

    fn hand_type(&self) -> HandType {
        let counts = self
            .count()
            .values()
            .sorted()
            .rev()
            .copied()
            .collect::<Vec<_>>();

        match counts.len() {
            1 => HandType::FiveOfAKind,
            2 => match (counts[0], counts[1]) {
                (4, 1) => HandType::FourOfAKind,
                _ => HandType::FullHouse,
            },
            3 => match (counts[0], counts[1], counts[2]) {
                (3, 1, 1) => HandType::ThreeOfAKind,
                _ => HandType::TwoPair,
            },
            4 => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

impl<'a> PartialEq for Hand<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.cards.eq(other.cards)
    }
}

impl<'a> Eq for Hand<'a> {}

impl<'a> Ord for Hand<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type().cmp(&other.hand_type()) {
            Ordering::Equal => {
                for (lhs, rhs) in std::iter::zip(self.cards.chars(), other.cards.chars()) {
                    match lhs.card_strength().cmp(&rhs.card_strength()) {
                        Ordering::Equal => continue,
                        cmp => return cmp,
                    }
                }
                Ordering::Equal
            }
            cmp => cmp,
        }
    }
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn index(&self) -> u8 {
        match self {
            HandType::FiveOfAKind => 7,
            HandType::FourOfAKind => 6,
            HandType::FullHouse => 5,
            HandType::ThreeOfAKind => 4,
            HandType::TwoPair => 3,
            HandType::OnePair => 2,
            HandType::HighCard => 1,
        }
    }
}

impl PartialEq for HandType {
    fn eq(&self, other: &Self) -> bool {
        self.index() == other.index()
    }
}

impl Eq for HandType {}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.index().cmp(&other.index())
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse<'a>(data: &'a str) -> (Vec<Hand<'a>>, Vec<u64>) {
    let (mut hands, mut bids) = (vec![], vec![]);

    for line in data.lines().filter(|x| !x.is_empty()) {
        let (hand, bid) = line.split_once(" ").unwrap();
        hands.push(Hand::from(hand));
        bids.push(bid.parse::<u64>().unwrap())
    }

    (hands, bids)
}

#[aoc(2023, 7)]
pub fn main() {
    let data = aoc_input!(2023, 7).unwrap();
    let (hands, bids) = parse(&data);

    // Part I
    let res = std::iter::zip(hands, bids)
        .sorted_by(|(lhs, _), (rhs, _)| lhs.cmp(rhs))
        .enumerate()
        .map(|(rank, (_, bid))| (rank as u64 + 1) * bid)
        .sum::<u64>();

    println!("{res}");

    // Part II
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
        let (hands, _) = parse(EXAMPLE1);
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
        cards.sort_by_key(|c| c.card_strength());
        let expected = [
            '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
        ];
        assert_eq!(cards, expected);
    }

    #[test]
    fn test_strength_order() {
        let (hands, _) = parse(EXAMPLE1);
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
        let lhs = Hand::from("2345A");
        let rhs = Hand::from("2345J");
        assert!(lhs > rhs);
    }

    #[test]
    fn test_edge_case_hands() {
        let expected = &["2345J", "2345A", "J345A", "32T3K", "Q2KJJ"];
        let cards = &["2345A", "Q2KJJ", "2345J", "32T3K", "J345A"];

        assert_eq!(
            expected.iter().map(|c| Hand::from(*c)).collect::<Vec<_>>(),
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

        for (h, b) in std::iter::zip(hands, bids).sorted_by(|a, b| a.1.cmp(&b.1)) {
            println!(">> {:?}, {}", h, b);
        }

        let mut res = 0;

        assert_eq!(res, 6592);
    }
}
