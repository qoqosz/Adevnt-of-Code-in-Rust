use aoc::{aoc, aoc_input};

fn parse(data: &str) -> (Vec<(u8, u8)>, Vec<Vec<u8>>) {
    let (rules, updates) = data.split_once("\n\n").unwrap();
    let rules = rules
        .trim()
        .lines()
        .map(|line| {
            let (x, y) = line.split_once('|').unwrap();
            (x.parse::<u8>().unwrap(), y.parse::<u8>().unwrap())
        })
        .collect::<Vec<(_, _)>>();
    let updates = updates
        .trim()
        .lines()
        .map(|line| {
            line.split(',')
                .flat_map(|x| x.parse::<u8>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();
    (rules, updates)
}

fn index(vec: &[u8], x: &u8) -> Option<usize> {
    vec.iter().position(|r| *r == *x)
}

fn get_middle(vec: &[u8]) -> u8 {
    let n = vec.len();
    vec[n / 2]
}

// Part I
fn validate(update: &[u8], rules: &[(u8, u8)]) -> bool {
    for (x, y) in rules {
        if let (Some(i), Some(j)) = (index(update, x), index(update, y)) {
            if i > j {
                return false;
            }
        }
    }

    true
}

// Part II
fn correct(update: &[u8], rules: &[(u8, u8)]) -> Vec<u8> {
    let mut tmp = Vec::from(update);
    let mut is_valid = false;

    while !is_valid {
        is_valid = true;

        for (x, y) in rules {
            if let (Some(i), Some(j)) = (index(&tmp, x), index(&tmp, y)) {
                if i > j {
                    tmp.swap(i, j);
                    is_valid = false;
                    break;
                }
            }
        }
    }

    tmp
}

#[aoc(2024, 5)]
pub fn main() {
    let data = aoc_input!(2024, 5).unwrap();
    let (rules, updates) = parse(&data);

    // Part I
    let sum = updates
        .iter()
        .filter(|update| validate(update, &rules))
        .map(|update| get_middle(update) as usize)
        .sum::<usize>();
    println!("{sum}");

    // Part II
    let sum = updates
        .iter()
        .filter(|update| !validate(update, &rules))
        .map(|update| {
            let corrected = correct(update, &rules);
            get_middle(&corrected) as usize
        })
        .sum::<usize>();
    println!("{sum}");
}
