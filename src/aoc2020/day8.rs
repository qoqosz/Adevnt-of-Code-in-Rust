use aoc::aoc_input;

fn main() {
    let data = aoc_input!(2020, 8).unwrap();
    let prog = data
        .lines()
        .filter_map(|l| l.trim().split_once(' '))
        .map(|(cmd, val)| (cmd, val.parse::<i32>().unwrap()))
        .collect::<Vec<_>>();

    // Part I
    let (mut acc, mut i) = (0, 0);
    let mut executed = vec![false; prog.len()];

    while !executed[i as usize] {
        executed[i as usize] = true;
        let (cmd, val) = prog[i as usize];

        match cmd {
            "acc" => (acc, i) = (acc + val, i + 1),
            "jmp" => i += val,
            _ => i += 1,
        }
    }

    println!("{:?}", acc);

    // Part II
    fn exe(prog: &Vec<(&str, i32)>) -> (bool, i32) {
        let (mut acc, mut i) = (0, 0);
        let mut executed = vec![false; prog.len()];

        loop {
            match executed.get(i as usize) {
                Some(val) => {
                    if *val {
                        return (false, acc);
                    }
                }
                _ => return (true, acc),
            }

            executed[i as usize] = true;
            let (cmd, val) = prog[i as usize];

            match cmd {
                "acc" => (acc, i) = (acc + val, i + 1),
                "jmp" => i += val,
                _ => i += 1,
            }
        }
    }

    for (i, (cmd, val)) in prog.iter().enumerate() {
        if ["nop", "jmp"].contains(cmd) {
            let mut new_prog = prog.clone();

            match *cmd {
                "nop" => new_prog[i] = ("jmp", *val),
                "jmp" => new_prog[i] = ("nop", *val),
                _ => {}
            }

            let (exe_code, acc) = exe(&new_prog);

            if exe_code {
                println!("{}", acc);
                break;
            }
        }
    }
}
