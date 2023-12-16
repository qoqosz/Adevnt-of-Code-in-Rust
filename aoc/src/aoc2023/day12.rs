use aoc::{aoc, aoc_input};
use itertools::Itertools;

fn parse(data: &str) -> Vec<(String, Vec<usize>)> {
    data.lines()
        .filter(|x| !x.is_empty())
        .flat_map(|line| line.split_once(' '))
        .map(|(rec, arr)| {
            (
                rec.to_owned(),
                arr.split(',')
                    .flat_map(|n| n.parse::<usize>())
                    .collect::<Vec<_>>(),
            )
        })
        .collect()
}

// TODO: rework memoization manually
#[memoize::memoize]
fn calc(record: String, groups: Vec<usize>) -> usize {
    if groups.is_empty() {
        return if record.contains('#') { 0 } else { 1 };
    }
    if record.is_empty() {
        return 0;
    }

    let ch = record.chars().next().unwrap();
    let count = *groups.first().unwrap();

    if ch == '.' {
        // skip char
        return calc(record[1..].to_owned(), groups);
    }
    if ch == '#' {
        // record is not long enough
        if record.len() < count {
            return 0;
        }

        // we can't build a group
        if record[..count].contains('.') {
            return 0;
        }

        // record is just the right len
        if record.len() == count {
            // and this is the last group
            return match groups.len() {
                1 => 1,
                _ => 0,
            };
        }
        // record is longer
        if record.chars().nth(count).unwrap() == '#' {
            return 0;
        }
        return calc(record[count + 1..].to_owned(), groups[1..].to_vec());
    }
    if ch == '?' {
        return calc(".".to_owned() + &record[1..], groups.clone())
            + calc("#".to_owned() + &record[1..], groups);
    }
    unreachable!()
}

#[aoc(2023, 12)]
pub fn main() {
    let data = aoc_input!(2023, 12).unwrap();
    let input = parse(&data);

    // Part I
    let total = input
        .iter()
        .map(|(r, g)| calc(r.clone(), g.clone()))
        .sum::<usize>();
    println!("{total}");

    // Part II
    let total = input
        .iter()
        .map(|(r, g)| {
            let new_r = vec![r.clone(); 5].iter().join("?");
            let new_g = vec![g.clone(); 5]
                .iter()
                .flatten()
                .copied()
                .collect::<Vec<_>>();
            calc(new_r, new_g)
        })
        .sum::<usize>();
    println!("{total}");
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    static EXAMPLE2: &str = "????.######..#####. 1,6,5";

    #[test]
    fn test_arrangement() {
        let input = parse(EXAMPLE);
        let expected = [1, 4, 1, 1, 4, 10];
        assert_eq!(
            input
                .iter()
                .map(|(r, g)| calc(r.to_string(), g.clone()))
                .collect::<Vec<_>>(),
            expected
        );
    }
}
