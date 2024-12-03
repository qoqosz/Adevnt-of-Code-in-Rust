use aoc::{aoc, aoc_input};
use regex_lite::{Match, Regex};

fn add_muls(txt: &str) -> u64 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let numbers = re
        .captures_iter(txt)
        .map(|caps| {
            let (_, [x, y]) = caps.extract();
            (x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap())
        })
        .collect::<Vec<(_, _)>>();
    numbers.iter().map(|(x, y)| x * y).sum::<u64>()
}

fn add_muls_flagged(txt: &str) -> u64 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();

    let mut flag = true;
    let mut sum = 0;
    let to_u64 = |x: Match<'_>| x.as_str().parse::<u64>().unwrap();

    for caps in re.captures_iter(txt) {
        if let Some(word) = caps.get(0).map(|m| m.as_str()) {
            match word {
                "do()" => flag = true,
                "don't()" => flag = false,
                _ => {
                    if flag {
                        if let (Some(x), Some(y)) = (caps.get(1), caps.get(2)) {
                            sum += to_u64(x) * to_u64(y);
                        }
                    }
                }
            }
        }
    }

    sum
}

#[aoc(2024, 3)]
pub fn main() {
    let data = aoc_input!(2024, 3).unwrap();
    let memory = data.replace('\n', "");

    // Part I
    let sum = add_muls(&memory);
    println!("{sum}");

    // Part II
    let sum = add_muls_flagged(&memory);
    println!("{sum}");
}
