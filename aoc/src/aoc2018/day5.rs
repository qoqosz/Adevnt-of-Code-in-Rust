use aoc::{aoc, aoc_input};

static DIFF: u8 = 32;

fn react(polymer: &[u8]) -> Vec<u8> {
    let mut res = vec![];
    let mut was_consumed = false;

    for win in polymer.windows(2) {
        if was_consumed {
            was_consumed = false;
            continue;
        }

        if win[0].abs_diff(win[1]) == DIFF {
            was_consumed = true;
            continue;
        }

        res.push(win[0]);
    }

    if polymer.len() > 1 {
        res.push(*polymer.last().unwrap());
    }

    res
}

fn fully_react(polymer: &str) -> usize {
    let mut polymer = polymer.as_bytes().to_vec();

    loop {
        let reduced = react(&polymer);

        if reduced.len() == polymer.len() {
            break;
        }

        polymer = reduced;
    }

    polymer.len()
}

fn improved(polymer: &str) -> Option<usize> {
    let polymer = polymer.as_bytes();

    (b'A'..=b'Z')
        .map(|ch| {
            let tmp = polymer
                .iter()
                .filter(|x| **x != ch && **x != ch + DIFF)
                .map(|x| *x as char)
                .collect::<String>();
            fully_react(&tmp)
        })
        .min()
}

#[aoc(2018, 5)]
pub fn main() {
    let data = aoc_input!(2018, 5).unwrap();
    let polymer = data.trim();

    // Part I
    println!("{}", fully_react(polymer));

    // Part II
    println!("{}", improved(polymer).unwrap());
}
