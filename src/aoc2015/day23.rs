use std::fmt::{Display, Formatter, Result};

use aoc::aoc_input;

#[derive(Debug)]
enum Instructions {
    Hlf(usize),
    Tpl(usize),
    Inc(usize),
    Jmp(i64),
    Jie(usize, i64),
    Jio(usize, i64),
    Invalid,
}

impl<S: Into<String>> From<S> for Instructions {
    fn from(value: S) -> Self {
        let value = value.into();
        let parts = value
            .split(' ')
            .map(|x| x.strip_suffix(',').unwrap_or(x))
            .collect::<Vec<&str>>();
        let register = (parts[1].chars().nth(0).unwrap() as u8).saturating_sub(b'a') as usize;
        let as_num = |x: &str| x.parse::<i64>().unwrap();

        match parts[0] {
            "hlf" => Self::Hlf(register),
            "tpl" => Self::Tpl(register),
            "inc" => Self::Inc(register),
            "jmp" => Self::Jmp(as_num(parts[1])),
            "jie" => Self::Jie(register, as_num(parts[2])),
            "jio" => Self::Jio(register, as_num(parts[2])),
            _ => Self::Invalid,
        }
    }
}

struct Program<'a> {
    pos: i64,
    registers: Vec<i64>,
    tape: &'a Vec<Instructions>,
}

impl<'a> Display for Program<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.registers.get(1).unwrap_or(&0))
    }
}

impl<'a> Program<'a> {
    fn new(tape: &'a Vec<Instructions>) -> Self {
        Self {
            pos: 0,
            registers: vec![0, 0],
            tape,
        }
    }

    fn eat(&mut self) -> Option<()> {
        match self.tape.get(self.pos as usize) {
            Some(Instructions::Hlf(reg)) => {
                self.registers[*reg] /= 2;
                self.pos += 1;
            }
            Some(Instructions::Tpl(reg)) => {
                self.registers[*reg] *= 3;
                self.pos += 1;
            }
            Some(Instructions::Inc(reg)) => {
                self.registers[*reg] += 1;
                self.pos += 1;
            }
            Some(Instructions::Jmp(off)) => self.pos += *off,
            Some(Instructions::Jie(reg, off)) => match self.registers[*reg] % 2 == 0 {
                true => self.pos += *off,
                false => self.pos += 1,
            },
            Some(Instructions::Jio(reg, off)) => match self.registers[*reg] == 1 {
                true => self.pos += *off,
                false => self.pos += 1,
            },
            _ => {
                return None;
            }
        }
        Some(())
    }

    fn run(&mut self) {
        while self.eat().is_some() {}
    }

    fn reset(&mut self, registers: &[i64]) {
        self.pos = 0;
        self.registers = registers.to_owned();
    }
}

pub fn main() {
    let data = aoc_input!(2015, 23).unwrap();
    let tape = data
        .lines()
        .filter(|x| !x.is_empty())
        .map(Instructions::from)
        .collect::<Vec<Instructions>>();

    // Part I
    let mut prog = Program::new(&tape);
    prog.run();
    println!("{}", prog);

    // Part I
    prog.reset(&[1, 0]);
    prog.run();
    println!("{}", prog);
}
