use aoc::aoc_input;
use regex_lite::{Match, Regex};

#[derive(Debug)]
struct Number {
    value: u32,
    y: usize,
    x_min: usize,
    x_max: usize,
}

impl Number {
    fn is_adjacent(&self, symbol: &Symbol) -> bool {
        self.y.abs_diff(symbol.y) <= 1
            && self.x_min.saturating_sub(1) <= symbol.x
            && self.x_max >= symbol.x
    }
}

impl From<&Match<'_>> for Number {
    fn from(m: &Match<'_>) -> Self {
        Number {
            value: m.as_str().parse::<u32>().unwrap(),
            y: 0,
            x_min: m.start(),
            x_max: m.end(),
        }
    }
}

#[derive(Debug)]
struct Symbol {
    symbol: char,
    y: usize,
    x: usize,
}

impl From<&Match<'_>> for Symbol {
    fn from(m: &Match<'_>) -> Self {
        Symbol {
            symbol: m.as_str().chars().next().unwrap(),
            y: 0,
            x: m.start(),
        }
    }
}

fn read_schema(lines: &[&str]) -> (Vec<Number>, Vec<Symbol>) {
    let re_num = Regex::new(r"\d+").unwrap();
    let re_sym = Regex::new(r"[^.\d]").unwrap();

    let mut numbers = vec![];
    let mut symbols = vec![];

    for (i, line) in lines.iter().enumerate() {
        for captures in re_num.captures_iter(line) {
            let mut number = Number::from(&captures.get(0).unwrap());
            number.y = i;
            numbers.push(number);
        }
        for captures in re_sym.captures_iter(line) {
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
        .filter(|n| symbols.iter().any(|s| n.is_adjacent(s)))
        .map(|n| n.value)
        .sum::<u32>();
    println!("{}", sum_parts);

    // Part II
    let sum_gear_ratios = symbols
        .iter()
        .filter(|s| s.symbol == '*')
        .filter_map(|s| {
            let adj = numbers
                .iter()
                .filter(|n| n.is_adjacent(s))
                .map(|n| n.value)
                .collect::<Vec<_>>();
            if adj.len() == 2 {
                return Some(adj.iter().product::<u32>());
            }
            None
        })
        .sum::<u32>();
    println!("{sum_gear_ratios}");
}
