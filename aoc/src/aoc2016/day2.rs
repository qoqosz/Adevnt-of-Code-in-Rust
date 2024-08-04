use aoc::{aoc, aoc_input};
use rustc_hash::FxHashMap;

struct Keypad {
    grid: FxHashMap<(i8, i8), char>,
    pos: (i8, i8),
}

impl Keypad {
    fn create(grid: &str, pos: (i8, i8)) -> Self {
        let grid = grid
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, ch)| *ch != ' ')
                    .map(move |(col, ch)| ((row as i8, col as i8), ch))
            })
            .flatten()
            .collect::<FxHashMap<(i8, i8), char>>();
        Self { grid, pos }
    }

    fn eval_single(&mut self, instruction: &str) -> char {
        for ch in instruction.chars() {
            let ds = match ch {
                'D' => (1, 0),
                'U' => (-1, 0),
                'L' => (0, -1),
                'R' => (0, 1),
                _ => unreachable!("invalid instruction"),
            };
            let pos = (self.pos.0 + ds.0, self.pos.1 + ds.1);

            if self.grid.contains_key(&pos) {
                self.pos = pos;
            }
        }

        self.grid[&self.pos]
    }

    fn eval(&mut self, instructions: &str) -> String {
        instructions
            .lines()
            .map(|line| self.eval_single(line))
            .collect()
    }
}

#[aoc(2016, 2)]
pub fn main() {
    let data = aoc_input!(2016, 2).unwrap();

    // Part I
    let grid: &str = "123\n456\n789";
    let mut keypad = Keypad::create(&grid, (1, 1));
    let code = keypad.eval(&data);
    println!("{code}");

    // Part II
    let grid: &str = "  1\n 234\n56789\n ABC\n  D";
    let mut keypad = Keypad::create(&grid, (2, 2));
    let code = keypad.eval(&data);
    println!("{code}");
}
