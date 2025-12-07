use aoc::{aoc, aoc_input};

fn eval(input: i32, program: &[i32]) -> i32 {
    let mut p = program.to_vec();
    let mut i = 0;
    let mut p2 = 0;

    loop {
        let cmd = format!("{:0>5}", p[i]);
        let opcode: i32 = cmd[3..].parse().unwrap();
        let cmd1 = cmd[2..3].parse::<i32>().unwrap();
        let cmd2 = cmd[1..2].parse::<i32>().unwrap();
        let p1 = if cmd1 != 0 {
            p[i + 1]
        } else {
            p[p[i + 1] as usize]
        };
        if cmd2 != 0 {
            p2 = p[i + 2];
        } else {
            let idx = p[i + 2] as usize;

            if let Some(val) = p.get(idx) {
                p2 = *val;
            }
        };
        match opcode {
            1 => {
                let idx = p[i + 3] as usize;
                p[idx] = p1 + p2;
                i += 4;
            }
            2 => {
                let idx = p[i + 3] as usize;
                p[idx] = p1 * p2;
                i += 4;
            }
            3 => {
                let idx = p[i + 1] as usize;
                p[idx] = input;
                i += 2;
            }
            4 => {
                if p1 != 0 {
                    return p1;
                }
                i += 2;
            }
            5 => i = if p1 != 0 { p2 as usize } else { i + 3 },
            6 => i = if p1 == 0 { p2 as usize } else { i + 3 },
            7 => {
                let idx = p[i + 3] as usize;
                p[idx] = if p1 < p2 { 1 } else { 0 };
                i += 4;
            }
            8 => {
                let idx = p[i + 3] as usize;
                p[idx] = if p1 == p2 { 1 } else { 0 };
                i += 4;
            }
            _ => {}
        }
    }
}

#[aoc(2019, 5)]
pub fn main() {
    let data = aoc_input!(2019, 5).unwrap();
    let program = data
        .trim()
        .split(',')
        .flat_map(|n| n.parse::<i32>())
        .collect::<Vec<_>>();

    // Part I
    println!("{}", eval(1, &program));

    // Part II
    println!("{}", eval(5, &program));
}
