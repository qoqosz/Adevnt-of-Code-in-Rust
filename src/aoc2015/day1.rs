use aoc::aoc_input;

fn step(c: char, lvl: i32) -> i32 {
    match c {
        '(' => lvl + 1,
        _ => lvl - 1,
    }
}

fn dest_floor(instructions: &str) -> i32 {
    // Calculate the final floor where Santa ends given the instructions.
    instructions.chars().fold(0, |mut lvl, c| {
        lvl = step(c, lvl);
        lvl
    })
}

fn reach_basement(instructions: &str) -> Option<usize> {
    // The position of the character that causes Santa to first enter the basement.
    let mut pos: i32 = 0;

    for (i, c) in instructions.chars().enumerate() {
        pos = step(c, pos);

        if pos == -1 {
            return Some(i + 1);
        }
    }
    None
}

fn main() {
    let data = aoc_input!(2015, 1).unwrap();

    // Part I
    let ans1 = dest_floor(&data);
    println!("{}", ans1);

    // Part II
    let ans2 = reach_basement(&data);
    println!("{}", ans2.unwrap());
}
