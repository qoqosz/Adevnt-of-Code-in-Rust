use aoc::{aoc, aoc_input};
use std::num::ParseIntError;
use tinyvec::ArrayVec;

static MARKED: u8 = u8::MAX;

#[derive(Debug)]
struct Board {
    grid: ArrayVec<[u8; 25]>,
}

impl TryFrom<&str> for Board {
    type Error = ParseIntError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let grid = value
            .trim()
            .lines()
            .flat_map(|line| line.split_whitespace().flat_map(|elem| elem.parse::<u8>()))
            .collect::<ArrayVec<_>>();
        Ok(Board { grid })
    }
}

impl Board {
    fn mark(&mut self, n: u8) {
        for x in self.grid.iter_mut() {
            if *x == n {
                *x = MARKED;
            }
        }
    }

    fn row(&self, i: usize) -> impl Iterator<Item = &u8> {
        (0..5).flat_map(move |j| self.grid.get(i * 5 + j))
    }

    fn col(&self, i: usize) -> impl Iterator<Item = &u8> {
        (0..5).flat_map(move |j| self.grid.get(i + 5 * j))
    }

    fn score(&self, n: u8) -> usize {
        let sum = self
            .grid
            .iter()
            .filter(|x| **x != MARKED)
            .map(|x| *x as usize)
            .sum::<usize>();
        sum * n as usize
    }

    fn is_bingo(&self) -> bool {
        fn check<'a>(it: impl Iterator<Item = &'a u8>) -> bool {
            it.into_iter().all(|x| *x == MARKED)
        }
        (0..5).any(|i| check(self.row(i)) || check(self.col(i)))
    }
}

#[aoc(2021, 4)]
pub fn main() {
    let data = aoc_input!(2021, 4).unwrap();
    let mut segments = data.split("\n\n");
    let nums = segments
        .next()
        .unwrap()
        .split(',')
        .flat_map(|x| x.parse::<u8>());
    let mut boards = segments.flat_map(Board::try_from).collect::<Vec<_>>();
    let mut scores = Vec::with_capacity(boards.len());

    for n in nums {
        boards.retain_mut(|board| {
            board.mark(n);

            if board.is_bingo() {
                scores.push(board.score(n));
                false
            } else {
                true
            }
        });
    }

    // Part I
    println!("{}", scores.first().unwrap());

    // Part II
    println!("{}", scores.last().unwrap());
}
