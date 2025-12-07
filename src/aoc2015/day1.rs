use aoc::aoc_input;

fn step(c: char, lvl: i32) -> i32 {
    match c {
        '(' => lvl + 1,
        _ => lvl - 1,
    }
}

fn dest_floor(instructions: &str) -> i32 {
    // Calculate the final floor where Santa ends given the instructions.
    instructions.chars().fold(0, |lvl, c| step(c, lvl))
}

fn reach_basement(instructions: &str) -> Option<usize> {
    // The position of the character that causes Santa to first enter the basement.
    instructions
        .chars()
        .scan(0, |pos, c| {
            *pos = step(c, *pos);
            Some(*pos)
        })
        .position(|pos| pos == -1)
        .map(|i| i + 1)
}

pub fn main() {
    let data = aoc_input!(2015, 1).unwrap();

    // Part I
    let ans1 = dest_floor(&data);
    println!("{}", ans1);

    // Part II
    let ans2 = reach_basement(&data);
    println!("{}", ans2.unwrap());
}
