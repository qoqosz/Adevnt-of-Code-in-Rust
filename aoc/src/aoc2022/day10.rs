use aoc::{aoc, aoc_input};
use itertools::Itertools;

enum Instruction {
    NoOp,
    Add(i32),
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        return if value.starts_with("noop") {
            Self::NoOp
        } else {
            let x = value.split_once(' ').unwrap().1.parse::<i32>().unwrap();
            Self::Add(x)
        };
    }
}

#[allow(non_snake_case)]
struct CPU<'ops> {
    X: i32,
    cycle: i32,
    instructions: &'ops [Instruction],
    states: Vec<(i32, i32)>,
}

impl<'ops> CPU<'ops> {
    fn new(instructions: &'ops [Instruction]) -> Self {
        Self {
            X: 1,
            cycle: 0,
            instructions,
            states: Vec::with_capacity(300),
        }
    }

    #[inline]
    fn step(&mut self) {
        self.cycle += 1;
        self.states.push((self.cycle, self.X));
    }

    fn addx(&mut self, x: i32) {
        self.step();
        self.step();
        self.X += x;
    }

    fn eval(&mut self) {
        for instruction in self.instructions {
            match instruction {
                Instruction::NoOp => self.step(),
                Instruction::Add(x) => self.addx(*x),
            }
        }
    }
}

#[aoc(2022, 10)]
pub fn main() {
    let data = aoc_input!(2022, 10).unwrap();
    let instructions = data
        .trim_end()
        .lines()
        .map(Instruction::from)
        .collect::<Vec<_>>();
    let mut cpu = CPU::new(&instructions);
    cpu.eval();

    // Part I
    let cycles = [20, 60, 100, 140, 180, 220];
    println!(
        "{}",
        cpu.states
            .iter()
            .filter(|(i, _)| cycles.contains(i))
            .map(|(i, v)| i * v)
            .sum::<i32>()
    );

    // Part II
    let mut idx_crt = 0;
    let mut display: Vec<char> = Vec::with_capacity(320);

    for (_, value) in cpu.states {
        display.push(if (idx_crt - value).abs() <= 1 {
            '█'
        } else {
            ' '
        });
        idx_crt = (idx_crt + 1) % 40;
    }

    println!(
        "{}",
        display
            .chunks(40)
            .map(|line| line.iter().collect::<String>())
            .join("\n")
    );
}
