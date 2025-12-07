use aoc::{aoc, aoc_input};
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::sync::LazyLock;

static PAIRS: LazyLock<FxHashMap<u8, u8>> = LazyLock::new(|| {
    FxHashMap::from_iter([(b'(', b')'), (b'[', b']'), (b'{', b'}'), (b'<', b'>')])
});
static OPEN: LazyLock<FxHashSet<&u8>> = LazyLock::new(|| PAIRS.keys().collect());

trait Middle: Iterator<Item = usize> {
    fn middle(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        let vals = self.sorted_unstable().collect::<Vec<_>>();
        vals.get(vals.len() / 2).copied()
    }
}

impl<I: Iterator<Item = usize>> Middle for I {}

fn check(line: &[u8], stack: &mut Vec<u8>) -> Option<u8> {
    for ch in line {
        match OPEN.contains(&ch) {
            true => stack.push(*ch),
            false => {
                if let Some(x) = stack.pop() {
                    if PAIRS.get(&x) != Some(ch) {
                        return Some(*ch);
                    }
                }
            }
        }
    }
    None
}

fn get_score(stack: &mut Vec<u8>) -> usize {
    let score = FxHashMap::from_iter([(b')', 1), (b']', 2), (b'}', 3), (b'>', 4)]);

    stack
        .iter()
        .rev()
        .filter_map(|c| score.get(PAIRS.get(c)?))
        .fold(0, |acc, x| 5 * acc + *x)
}

#[aoc(2021, 10)]
pub fn main() {
    let data = aoc_input!(2021, 10).unwrap();
    let lines = data.trim().lines().map(|line| line.as_bytes());

    // Part I
    let score = FxHashMap::from_iter([(b')', 3), (b']', 57), (b'}', 1197), (b'>', 25137)]);
    let res = lines
        .clone()
        .flat_map(|bytes| check(bytes, &mut vec![]))
        .flat_map(|x| score.get(&x))
        .sum::<usize>();
    println!("{res}");

    // Part II
    let res = lines
        .filter_map(|bytes| {
            let mut stack = vec![];

            match check(bytes, &mut stack) {
                None => Some(get_score(&mut stack)),
                Some(_) => None,
            }
        })
        .middle()
        .unwrap();
    println!("{res}");
}
