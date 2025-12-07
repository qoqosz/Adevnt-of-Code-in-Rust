use aoc::{aoc, aoc_input};

fn parse(data: &str) -> Vec<(&str, i32)> {
    data.lines()
        .filter_map(|line| line.split_once(' '))
        .map(|(dir, x)| (dir, x.parse().unwrap()))
        .collect()
}

fn find_pos(commands: &[(&str, i32)]) -> i32 {
    let (mut horiz, mut depth) = (0, 0);

    for (dir, x) in commands {
        match *dir {
            "forward" => horiz += x,
            "down" => depth += x,
            "up" => depth -= x,
            _ => unreachable!(),
        }
    }

    horiz * depth
}

fn find_aim(commands: &[(&str, i32)]) -> i32 {
    let (mut horiz, mut depth, mut aim) = (0, 0, 0);

    for (dir, x) in commands {
        match *dir {
            "down" => aim += x,
            "up" => aim -= x,
            "forward" => {
                horiz += x;
                depth += aim * x;
            }
            _ => unreachable!(),
        }
    }

    horiz * depth
}

#[aoc(2021, 2)]
pub fn main() {
    let data = aoc_input!(2021, 2).unwrap();
    let commands = parse(&data);

    // Part I
    println!("{}", find_pos(&commands));

    // Part II
    println!("{}", find_aim(&commands));
}
