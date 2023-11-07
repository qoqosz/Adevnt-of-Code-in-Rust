use aoc::aoc_input;
use itertools::Itertools;

fn main() {
    let data = aoc_input!(2020, 2).unwrap();
    let input = data
        .lines()
        .map(|l| l.splitn(3, ' ').collect_tuple().unwrap())
        .collect::<Vec<_>>();

    let (mut p1_count, mut p2_count) = (0, 0);

    for (rng, char, text) in input.iter() {
        let char = char.chars().next().unwrap();
        let (min, max) = rng
            .splitn(2, '-')
            .map(|x| x.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();

        let char_count = text.chars().filter(|c| *c == char).count();

        if (min..=max).contains(&char_count) {
            p1_count += 1;
        }

        if (char == text.chars().nth(min - 1).unwrap())
            ^ (char == text.chars().nth(max - 1).unwrap())
        {
            p2_count += 1;
        }
    }

    // Part I
    println!("{}", p1_count);

    // Part II
    println!("{}", p2_count);
}
