use aoc::aoc_input;

fn exe(prog: &[(&str, i32)]) -> (bool, i32) {
    let (mut acc, mut i) = (0, 0);
    let mut executed = vec![false; prog.len()];

    loop {
        match executed.get_mut(i) {
            Some(&mut true) => return (false, acc),
            Some(x @ &mut false) => *x = true,
            _ => return (true, acc),
        }

        match prog[i] {
            ("acc", val) => (acc, i) = (acc + val, i + 1),
            ("jmp", val) => i = i.saturating_add_signed(val as isize),
            _ => i += 1,
        }
    }
}

pub fn main() {
    let data = aoc_input!(2020, 8).unwrap();
    let prog = data
        .lines()
        .filter_map(|l| l.trim().split_once(' '))
        .map(|(cmd, val)| (cmd, val.parse::<i32>().unwrap()))
        .collect::<Vec<_>>();

    // Part I
    println!("{:?}", exe(&prog).1);

    // Part II
    for (i, (cmd, val)) in prog
        .iter()
        .enumerate()
        .filter(|(_, (c, _))| ["nop", "jmp"].contains(c))
    {
        let mut new_prog = prog.clone();

        match *cmd {
            "nop" => new_prog[i] = ("jmp", *val),
            "jmp" => new_prog[i] = ("nop", *val),
            _ => {}
        }

        if let (true, acc) = exe(&new_prog) {
            println!("{}", acc);
            break;
        }
    }
}
