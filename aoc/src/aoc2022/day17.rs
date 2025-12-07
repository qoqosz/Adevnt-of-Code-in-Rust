use aoc::{aoc, aoc_input};
use num::Integer;
use rustc_hash::FxHashMap;
use std::iter::Cycle;
use std::slice::Iter;
use std::str::Chars;

static BOUNDARIES: u16 = 0b100000001;
static PIECES: [&[u16]; 5] = [
    // flipped upside-down
    &[0b000111100], // ####
    &[
        0b000010000, //  #
        0b000111000, // ###
        0b000010000, //  #
    ],
    &[
        0b000111000, // ###
        0b000001000, //   #
        0b000001000, //   #
    ],
    &[
        0b000100000, // #
        0b000100000, // #
        0b000100000, // #
        0b000100000, // #
    ],
    &[
        0b000110000, // ##
        0b000110000, // ##
    ],
];

fn push(block: &[u16], jet: char) -> Vec<u16> {
    match jet {
        '<' => block.iter().map(|x| *x << 1).collect(),
        _ => block.iter().map(|x| *x >> 1).collect(),
    }
}

struct Tetris<'a> {
    h: usize,
    jiter: Cycle<Chars<'a>>,
    piter: Cycle<Iter<'static, &'static [u16]>>,
    board: FxHashMap<u16, u16>,
}

impl<'a> Tetris<'a> {
    fn new(jet_pattern: &'a str) -> Self {
        let jiter = jet_pattern.chars().cycle();
        let piter = PIECES.iter().cycle();

        Self {
            h: 3,
            jiter,
            piter,
            board: FxHashMap::default(),
        }
    }

    fn is_collision(&self, h: usize, piece: &[u16]) -> bool {
        let board_slice =
            (h..(h + piece.len())).map(|i| self.board.get(&(i as u16)).unwrap_or(&BOUNDARIES));
        board_slice.zip(piece).any(|(x, y)| (x & y) != 0)
    }

    fn simulate(&mut self, n: usize) {
        for _ in 0..n {
            self.process()
        }
    }

    fn process(&mut self) {
        let mut piece: Vec<u16> = self.piter.next().unwrap().to_vec();

        loop {
            let jet = self.jiter.next().unwrap();
            let shifted_piece = push(&piece, jet);

            if !self.is_collision(self.h, &shifted_piece) {
                piece = shifted_piece;
            }

            if self.h == 0 || self.is_collision(self.h.saturating_sub(1), &piece) {
                for (i, j) in (self.h..self.h + piece.len()).enumerate() {
                    let prev = *self.board.get(&(j as u16)).unwrap_or(&BOUNDARIES);
                    self.board.insert(j as u16, prev | piece[i]);
                }
                self.h = *self.board.keys().max().unwrap_or(&0) as usize + 4;
                break;
            } else {
                self.h -= 1;
            }
        }
    }
}

fn find_min(pattern: &[u16]) -> (usize, i64) {
    let n = pattern.len();
    let mut min_cost: i64 = 1_000_000_000_000;
    let mut i_min = 0;

    for i in 1_000..20_000 {
        let cost = pattern[i..]
            .iter()
            .zip(pattern[..(n - i)].iter())
            .map(|(x, y)| x.abs_diff(*y) as i64)
            .sum::<i64>();

        if cost < min_cost {
            min_cost = cost;
            i_min = i;
        }
    }

    (i_min, min_cost)
}

fn subfinder(mylist: &[u16], pattern: &[u16]) -> Vec<usize> {
    let n = pattern.len();
    let mut matches = vec![];

    for (i, x) in mylist.iter().enumerate() {
        if *x == pattern[0] && &mylist[i..i + n] == pattern {
            matches.push(i)
        }
    }

    matches
}

#[aoc(2022, 17)]
pub fn main() {
    let data = aoc_input!(2022, 17).unwrap();
    let jet_pattern = data.trim();

    // Part I
    let mut tetris = Tetris::new(jet_pattern);
    tetris.simulate(2022);
    println!("{}", tetris.h - 3);

    // Part II
    // First, simulate 50k rocks to record a pattern in the board
    let mut tetris = Tetris::new(jet_pattern);
    tetris.simulate(50_000);
    // h - 30 is arbitrary
    let pattern = (0..(tetris.h - 30))
        .map(|i| tetris.board[&(i as u16)])
        .collect::<Vec<_>>();

    // Then, start shifting a pattern and calculate the "cost".
    // When cost is minimized, the cycle in the pattern and shifted
    // one overlaps, which results in the minimal cost.
    let (i_min, _) = find_min(&pattern);
    let sub_pattern = &pattern[i_min..i_min + 40];

    // Find indices of the sub_pattern in the pattern
    let idx = subfinder(&pattern, &sub_pattern);

    // Run a simulation again to get a number of rocks that have to fall
    // in order to fill in the board up to a height of idx[0]
    let mut tetris = Tetris::new(jet_pattern);
    let mut i_max = 0;

    for i in 0..10_000 {
        tetris.process();

        if (tetris.h - 3) > idx[0] {
            i_max = i;
            break;
        }
    }

    let (a, b) = 1000000000000_i64.div_rem(&i_max);

    // Again, run a simulation b times to record the height
    let mut tetris = Tetris::new(jet_pattern);
    tetris.simulate(b as usize);

    println!("{}", a as usize * idx[0] + (tetris.h - 3));
}
