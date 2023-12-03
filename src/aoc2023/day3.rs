use aoc::aoc_input;
use lazy_static::lazy_static;
use regex_lite::{Match, Regex};

lazy_static! {
    static ref RE_NUMBER: Regex = Regex::new(r"\d+").unwrap();
    static ref RE_SYMBOL: Regex = Regex::new(r"[^.\d]").unwrap();
}

#[derive(Debug)]
struct Number {
    value: u32,
    y: usize,
    x_min: usize,
    x_max: usize,
}

impl Number {
    /// Check if the number is adjacent to a symbol.
    fn is_adjacent(&self, symbol: &Symbol) -> bool {
        self.y.abs_diff(symbol.y) <= 1
            && self.x_min.saturating_sub(1) <= symbol.x
            && self.x_max >= symbol.x
    }

    /// Check if the number is adjacent to any symbol.
    fn is_adjacent_any(&self, symbols: &[Symbol]) -> bool {
        symbols.iter().any(|s| self.is_adjacent(s))
    }
}

impl TryFrom<&Match<'_>> for Number {
    type Error = std::num::ParseIntError;

    fn try_from(m: &Match<'_>) -> Result<Self, Self::Error> {
        Ok(Number {
            value: m.as_str().parse::<u32>()?,
            y: 0,
            x_min: m.start(),
            x_max: m.end(),
        })
    }
}

#[derive(Debug)]
struct Symbol<'h> {
    symbol: &'h str,
    y: usize,
    x: usize,
}

impl<'h> From<&Match<'h>> for Symbol<'h> {
    fn from(m: &Match<'h>) -> Self {
        Symbol {
            symbol: m.as_str(),
            y: 0,
            x: m.start(),
        }
    }
}

impl<'h> Symbol<'h> {
    /// Collect all numbers adjacent to the symbol.
    fn neighbors<I>(&self, numbers: &[Number]) -> I
    where
        I: FromIterator<u32>,
    {
        numbers
            .iter()
            .filter(|n| n.is_adjacent(self))
            .map(|n| n.value)
            .collect()
    }
}

fn read_schema<'h>(lines: &'h [&str]) -> (Vec<Number>, Vec<Symbol<'h>>) {
    let mut numbers = vec![];
    let mut symbols = vec![];

    for (i, line) in lines.iter().enumerate() {
        for captures in RE_NUMBER.captures_iter(line) {
            let mut number = Number::try_from(&captures.get(0).unwrap()).unwrap();
            number.y = i;
            numbers.push(number);
        }
        for captures in RE_SYMBOL.captures_iter(line) {
            let mut symbol = Symbol::from(&captures.get(0).unwrap());
            symbol.y = i;
            symbols.push(symbol);
        }
    }

    (numbers, symbols)
}

fn parse(data: &str) -> Vec<&str> {
    data.lines().filter(|x| !x.is_empty()).collect()
}

pub fn main() {
    let data = aoc_input!(2023, 3).unwrap();
    let lines = parse(&data);
    let (numbers, symbols) = read_schema(&lines);

    // Part I
    let sum_parts = numbers
        .iter()
        .filter(|n| n.is_adjacent_any(&symbols))
        .map(|n| n.value)
        .sum::<u32>();
    println!("{sum_parts}");

    // Part II
    let sum_gear_ratios = symbols
        .iter()
        .filter(|s| s.symbol == "*")
        .filter_map(|s| match s.neighbors::<Vec<_>>(&numbers) {
            adj if adj.len() == 2 => Some(adj.iter().product::<u32>()),
            _ => None,
        })
        .sum::<u32>();
    println!("{sum_gear_ratios}");
}
