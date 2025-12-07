use aoc::{aoc, aoc_input};

fn exit<F>(offsets: &Vec<i32>, f: F) -> usize
where
    F: Fn(&mut i32),
{
    let mut offsets = offsets.clone();
    let (mut i, mut s) = (0, 0);

    while let Some(offset) = offsets.get_mut(i as usize) {
        i += *offset;
        f(offset);
        s += 1;
    }

    s
}

#[aoc(2017, 5)]
pub fn main() {
    let data = aoc_input!(2017, 5).unwrap();
    let offsets = data
        .trim()
        .lines()
        .flat_map(|line| line.parse::<i32>())
        .collect::<Vec<_>>();

    // Part I
    println!("{}", exit(&offsets, |x| *x += 1));

    // Part II
    println!(
        "{}",
        exit(&offsets, |x| if *x >= 3 { *x -= 1 } else { *x += 1 })
    );
}
