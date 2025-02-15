use aoc::{aoc, aoc_input};
use itertools::Itertools;
use rustc_hash::FxHashMap;

#[derive(Debug, Clone, Copy)]
enum Job<'a> {
    Value(Option<i64>),
    MathOp(&'a str, &'a str, &'a str),
}

fn parse(data: &str) -> FxHashMap<&str, Job> {
    data.trim()
        .lines()
        .map(|line| {
            let (monkey, op) = line.trim().split_once(": ").unwrap();
            let rhs = if let Result::Ok(val) = op.parse::<i64>() {
                Job::Value(Some(val))
            } else {
                let val = op
                    .split_whitespace()
                    .collect_tuple::<(&str, &str, &str)>()
                    .unwrap();
                Job::MathOp(val.0, val.1, val.2)
            };
            (monkey, rhs)
        })
        .collect()
}

fn eval_node(monkeys: &FxHashMap<&str, Job>, node: &str) -> Option<i64> {
    match monkeys.get(node) {
        Some(Job::Value(val)) => *val,
        Some(Job::MathOp(m1, op, m2)) => {
            let lhs = eval_node(monkeys, m1);
            let rhs = eval_node(monkeys, m2);

            if lhs.is_none() || rhs.is_none() {
                return None;
            }

            let (lhs, rhs) = (lhs.unwrap(), rhs.unwrap());

            match *op {
                "+" => Some(lhs + rhs),
                "-" => Some(lhs - rhs),
                "*" => Some(lhs * rhs),
                "/" => Some(lhs / rhs),
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

fn propagate(monkeys: &FxHashMap<&str, Job>, node: &str, value: i64) -> i64 {
    if node == "humn" {
        return value;
    }

    match monkeys.get(node) {
        Some(Job::MathOp(m1, op, m2)) => match (eval_node(&monkeys, m1), eval_node(&monkeys, m2)) {
            (Some(val), None) => match *op {
                "+" => propagate(monkeys, m2, value - val),
                "-" => propagate(monkeys, m2, val - value),
                "/" => propagate(monkeys, m2, val / value),
                "*" => propagate(monkeys, m2, value / val),
                _ => unreachable!(),
            },
            (None, Some(val)) => match *op {
                "+" => propagate(monkeys, m1, value - val),
                "-" => propagate(monkeys, m1, value + val),
                "/" => propagate(monkeys, m1, val * value),
                "*" => propagate(monkeys, m1, value / val),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

fn find(monkeys: &FxHashMap<&str, Job>) -> i64 {
    let mut monkeys = monkeys.clone();
    let (m1, m2) = match monkeys.get(&"root") {
        Some(Job::MathOp(m1, _, m2)) => (*m1, *m2),
        _ => unreachable!(),
    };

    monkeys.insert("humn", Job::Value(None));
    let (a, b) = (eval_node(&monkeys, m1), eval_node(&monkeys, m2));

    propagate(&monkeys, "root", 2 * (a.unwrap_or(0) + b.unwrap_or(0)))
}

#[aoc(2022, 21)]
pub fn main() {
    let data = aoc_input!(2022, 21).unwrap();
    let monkeys = parse(&data);

    // Part I
    println!("{}", eval_node(&monkeys, "root").unwrap());

    // Part II
    println!("{}", find(&monkeys));
}
