use aoc::aoc_input;

static NUMS: [&str; 10] = [
    ".", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn calc(digits: &[(usize, u32)]) -> u32 {
    let first = *digits.first().unwrap();
    let last = *digits.last().unwrap();
    10 * first.1 + last.1
}

fn enumerate_digits(line: impl Into<String>) -> Vec<(usize, u32)> {
    line.into()
        .chars()
        .enumerate()
        .filter(|(_, c)| c.is_ascii_digit())
        .map(|(i, c)| (i, c.to_digit(10).unwrap()))
        .collect::<Vec<_>>()
}

fn enumerate_words(line: impl Into<String> + Clone) -> Vec<(usize, u32)> {
    NUMS.iter()
        .enumerate()
        .flat_map(|(i, num)| {
            line.clone()
                .into()
                .match_indices(num)
                .map(|(j, _)| (j, i as u32))
                .collect::<Vec<_>>()
        })
        .collect()
}

pub fn main() {
    let data = aoc_input!(2023, 1).unwrap();
    let lines = data.lines().filter(|x| !x.is_empty()).collect::<Vec<_>>();

    // Part I
    let calib = lines
        .iter()
        .map(|l| {
            let digits = enumerate_digits(*l);
            calc(&digits)
        })
        .sum::<u32>();

    println!("{calib}");

    // Part II
    let calib = lines
        .iter()
        .map(|l| {
            let mut digits = enumerate_digits(*l);
            let words = enumerate_words(*l);
            digits.extend(words);
            digits.sort_by_key(|x| x.0);
            calc(&digits)
        })
        .sum::<u32>();

    println!("{calib}");
}
