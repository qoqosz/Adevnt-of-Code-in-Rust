use aoc::{aoc, aoc_input};
use rustc_hash::FxHashSet;
use std::collections::VecDeque;

#[derive(PartialEq, Eq, Clone, Hash)]
struct IndicatorLight {
    state: u16,
}

impl IndicatorLight {
    fn zero(&self) -> Self {
        Self { state: 0 }
    }

    fn toggle<'a, I>(&self, buttons: I) -> Self
    where
        I: IntoIterator<Item = &'a i16>,
    {
        let state = buttons
            .into_iter()
            .map(|i| 1 << *i)
            .fold(self.state, |acc, x| acc ^ x);

        Self { state }
    }
}

impl TryFrom<&str> for IndicatorLight {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let state = value
            .as_bytes()
            .iter()
            .enumerate()
            .map(|(i, ch)| match ch {
                b'#' => 1 << i,
                _ => 0,
            })
            .fold(0, |acc, x| acc | x);
        Ok(Self { state })
    }
}

fn parse(data: &str) -> Vec<(IndicatorLight, Vec<Vec<i16>>, Vec<i16>)> {
    data.trim().lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> (IndicatorLight, Vec<Vec<i16>>, Vec<i16>) {
    let (id, rem) = line.split_once("] ").unwrap();
    let (buttons, joltage) = rem.split_once('{').unwrap();

    let indicator = IndicatorLight::try_from(&id[1..]).unwrap();
    let buttons = buttons
        .split(' ')
        .filter(|g| !g.is_empty())
        .map(|g| {
            g.trim()
                .trim_start_matches('(')
                .trim_end_matches(')')
                .split(',')
                .flat_map(|b| b.parse::<i16>())
                .collect::<Vec<_>>()
        })
        .collect();
    let joltage = joltage
        .trim_end_matches('}')
        .split(',')
        .flat_map(|j| j.parse::<i16>())
        .collect();

    (indicator, buttons, joltage)
}

fn configure_indicator(target: &IndicatorLight, buttons: &[Vec<i16>]) -> Option<usize> {
    let init = target.zero();
    let mut queue = VecDeque::from_iter([(init, 0)]);
    let mut seen = FxHashSet::default();

    while let Some((state, n)) = queue.pop_front() {
        if !seen.insert(state.clone()) {
            continue;
        }

        if state == *target {
            return Some(n);
        }

        for b in buttons {
            queue.push_back((state.toggle(b), n + 1));
        }
    }

    None
}

#[aoc(2025, 10)]
pub fn main() {
    let data = aoc_input!(2025, 10).unwrap();
    let input = parse(&data);

    // Part I
    let n_buttons: usize = input
        .iter()
        .flat_map(|(target, buttons, _)| configure_indicator(target, buttons))
        .sum();
    println!("{n_buttons}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_indicator() {
        let diagram = ".##.";
        let indicator = IndicatorLight::try_from(diagram).unwrap();
        assert_eq!(indicator.state, 0b0110u16);
    }

    #[test]
    fn test_toggle() {
        let mut indicator = IndicatorLight { state: 0 };
        indicator = indicator.toggle(&[0, 2]);
        indicator = indicator.toggle(&[0, 1]);
        assert_eq!(indicator.state, 0b0110);
    }

    #[test]
    fn test_parse_line() {
        let line = "[.##.] (3) (1,3) (2) {3,5,4,7}";
        let (indicator, buttons, joltage) = parse_line(line);
        let IndicatorLight { state } = indicator;

        assert_eq!(state, 0b0110);
        assert_eq!(buttons, vec![vec![3], vec![1, 3], vec![2]]);
        assert_eq!(joltage, vec![3, 5, 4, 7]);
    }

    #[test]
    fn test_configure_indicator() {
        let target = IndicatorLight { state: 0b0110 };
        let buttons = vec![
            vec![3],
            vec![1, 3],
            vec![2],
            vec![2, 3],
            vec![0, 2],
            vec![0, 1],
        ];
        assert_eq!(Some(2), configure_indicator(&target, &buttons));
    }
}
