use aoc::aoc;
use itertools::Itertools;

#[derive(Clone)]
struct Computer {
    a: usize,
    b: usize,
    c: usize,
    program: Vec<usize>,
    ptr: usize,
    output: Vec<usize>,
}

impl Computer {
    fn new(a: usize, b: usize, c: usize, program: &[usize]) -> Self {
        Self {
            a,
            b,
            c,
            program: program.to_vec(),
            ptr: 0,
            output: vec![],
        }
    }

    fn run(&mut self) {
        while self.ptr < self.program.len() - 1 {
            let opcode = self.program[self.ptr];
            let operand = self.program[self.ptr + 1];
            self.ptr += 2;

            match opcode {
                // adv
                0 => {
                    let num = self.a;
                    let den = 2_i32.pow(self.combo(operand) as u32) as usize;
                    self.a = num / den;
                }
                // bxl
                1 => {
                    self.b = self.b ^ operand;
                }
                // bst
                2 => {
                    self.b = self.combo(operand) % 8;
                }
                // jnz
                3 => {
                    if self.a != 0 {
                        self.ptr = operand;
                        continue;
                    }
                }
                // bxc
                4 => {
                    self.b = self.b ^ self.c;
                }
                // out
                5 => {
                    self.output.push(self.combo(operand) % 8);
                }
                // bdv
                6 => {
                    let num = self.a;
                    let den = 2_i32.pow(self.combo(operand) as u32) as usize;
                    self.b = num / den;
                }
                // cdv
                7 => {
                    let num = self.a;
                    let den = 2_i32.pow(self.combo(operand) as u32) as usize;
                    self.c = num / den;
                }
                // stop
                _ => break,
            }
        }
    }

    fn combo(&self, value: usize) -> usize {
        match value {
            0..=3 => value,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!(),
        }
    }
}

fn find_a(program: &[usize], a: usize, idx: usize) -> Option<usize> {
    let n = program.len();

    if idx == n {
        return Some(a);
    }

    for i in 0..8 {
        let mut computer = Computer::new(a * 8 + i, 0, 0, &program);
        computer.run();

        if computer.output[0] == program[n - idx - 1] {
            if let Some(a) = find_a(program, a * 8 + i, idx + 1) {
                return Some(a);
            }
        }
    }

    None
}

#[aoc(2024, 17)]
pub fn main() {
    let program = [2, 4, 1, 1, 7, 5, 1, 5, 4, 3, 5, 5, 0, 3, 3, 0];

    // Part I
    let mut computer = Computer::new(38610541, 0, 0, &program);
    computer.run();
    println!(
        "{}",
        computer.output.iter().map(|x| x.to_string()).join(",")
    );

    // Part II
    println!("{}", find_a(&program, 0, 0).unwrap());
}
