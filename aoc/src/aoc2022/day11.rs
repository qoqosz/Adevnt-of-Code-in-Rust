use aoc::{aoc, aoc_input};
use itertools::Itertools;
use num::Integer;
use std::collections::VecDeque;

#[derive(Debug)]
enum Op {
    Add(usize),
    Mul(usize),
    Square,
}

impl Op {
    fn eval(&self, old: usize) -> usize {
        match self {
            Self::Add(val) => old + val,
            Self::Mul(val) => old * val,
            Self::Square => old * old,
        }
    }
}

impl From<&str> for Op {
    fn from(value: &str) -> Self {
        let (op, val) = value.split_once(' ').unwrap();

        match (op, val) {
            ("*", "old") => Self::Square,
            ("*", _) => Self::Mul(val.parse::<usize>().unwrap()),
            _ => Self::Add(val.parse::<usize>().unwrap()),
        }
    }
}

#[derive(Debug)]
struct Monkey {
    n_inspects: usize,
    relief_factor: usize,
    items: VecDeque<usize>,
    op: Op,
    divisor: usize,
    if_true: usize,
    if_false: usize,
}

impl From<&str> for Monkey {
    fn from(value: &str) -> Self {
        let mut lines = value.lines();
        let line = lines.next().unwrap();
        let (_, id) = line.split_once(' ').unwrap();
        let id = id.trim_end_matches(':').parse::<usize>().unwrap();
        let items = lines.next().unwrap().split_once(": ").unwrap().1;
        let items = items
            .split(", ")
            .filter_map(|v| v.parse::<usize>().ok())
            .collect::<VecDeque<_>>();
        let op = Op::from(&lines.next().unwrap()[23..]);
        let divisor = lines.next().unwrap()[21..].parse::<usize>().unwrap();
        let if_true = lines.next().unwrap()[29..].parse::<usize>().unwrap();
        let if_false = lines.next().unwrap()[30..].parse::<usize>().unwrap();

        Self {
            n_inspects: 0,
            relief_factor: 3,
            items,
            op,
            divisor,
            if_true,
            if_false,
        }
    }
}

impl Monkey {
    #[inline]
    fn test_item(&self, value: usize) -> usize {
        match value % self.divisor {
            0 => self.if_true,
            _ => self.if_false,
        }
    }

    fn inspect(&mut self, lcm: usize) {
        self.items.iter_mut().for_each(|x| {
            *x = self.op.eval(*x);
            *x = (*x / self.relief_factor) % lcm;
            self.n_inspects += 1;
        });
    }

    fn throw(&mut self) -> Vec<(usize, usize)> {
        let mut res = Vec::with_capacity(self.items.len());

        while let Some(item) = self.items.pop_front() {
            let i = self.test_item(item);
            res.push((i, item));
        }

        res
    }
}

struct Gang<'m>(&'m mut [Monkey], usize);

impl<'m> Gang<'m> {
    fn new(monkeys: &'m mut [Monkey]) -> Self {
        let mut gang = Self(monkeys, 0);
        gang.1 = gang.lcm();
        println!("lcm: {}", gang.1);
        gang
    }

    fn lcm(&self) -> usize {
        self.0
            .iter()
            .map(|m| m.divisor)
            .reduce(|acc, x| acc.lcm(&x))
            .unwrap()
    }

    fn round(&mut self) {
        let lcm = self.1;

        for i in 0..self.0.len() {
            self.0[i].inspect(lcm);
            let throw = self.0[i].throw();
            throw
                .iter()
                .for_each(|(j, item)| self.0[*j].items.push_back(*item));
        }
    }

    fn monkey_business(&self) -> usize {
        let vals = self
            .0
            .iter()
            .map(|m| m.n_inspects)
            .sorted_unstable()
            .rev()
            .take(2)
            .collect::<Vec<_>>();
        vals[0] * vals[1]
    }
}

#[aoc(2022, 11)]
pub fn main() {
    let data = aoc_input!(2022, 11).unwrap();

    // Part I
    let mut monkeys = data.split("\n\n").map(Monkey::from).collect::<Vec<_>>();
    let mut gang = Gang::new(&mut monkeys);
    (0..20).for_each(|_| gang.round());
    println!("{}", gang.monkey_business());

    // Part II
    let mut monkeys = data
        .split("\n\n")
        .map(Monkey::from)
        .map(|mut m| {
            m.relief_factor = 1;
            m
        })
        .collect::<Vec<_>>();
    let mut gang = Gang::new(&mut monkeys);
    (0..10_000).for_each(|_| gang.round());
    println!("{}", gang.monkey_business());
}
