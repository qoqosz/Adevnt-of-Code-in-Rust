use aoc::{aoc, aoc_input};
use itertools::Itertools;
use memoize::memoize;

fn parse(data: &str) -> Vec<(&str, Vec<usize>)> {
    data.lines()
        .filter(|x| !x.is_empty())
        .flat_map(|line| line.split_once(' '))
        .map(|(rec, arr)| {
            (
                rec,
                arr.split(',')
                    .flat_map(|n| n.parse::<usize>())
                    .collect::<Vec<_>>(),
            )
        })
        .collect()
}

fn arrangement(records: &str) -> Vec<usize> {
    let mut res = vec![];

    for (k, v) in &records.chars().group_by(|c| *c) {
        if k == '#' {
            res.push(v.count() as usize);
        }
    }

    res
}

fn candidates(records: &str, arr: &Vec<usize>) -> usize {
    let idx = records
        .match_indices('?')
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    let candidates = vec!['#', '.'];
    let mut res = 0;

    for perm in (0..idx.len())
        .map(|_| vec!['#', '.'])
        .multi_cartesian_product()
    {
        let mut new_string = records.chars().collect::<Vec<_>>();

        for (i, j) in idx.iter().enumerate() {
            new_string[*j] = perm[i];
        }

        let new_str = new_string.into_iter().collect::<String>();
        let new_arr = arrangement(&new_str);

        if &new_arr == arr {
            res += 1;
        }
    }
    res
}

// https://github.com/shemetz/advent_of_code_2023/blob/main/day12.py
#[memoize]
fn recursive_arrangements(records: String, groups: Vec<usize>) -> usize {
    if records.is_empty() {
        return match groups.len() {
            0 => 1,
            _ => 0,
        };
    }
    if records.starts_with('.') {
        return recursive_arrangements(records[1..].to_string(), groups);
    }
    if records.starts_with('?') {
        return recursive_arrangements(records.replacen('?', "#", 1), groups.clone())
            + recursive_arrangements(records.replacen('?', ".", 1), groups);
    }
    if records.starts_with('#') {
        if groups.is_empty() {
            return 0;
        }
        if records.len() < groups[0] {
            return 0;
        }
        if records[..groups[0]].contains('.') {
            return 0;
        }
        if groups.len() > 1 {
            if (records.len() < groups[0] + 1) || records.chars().nth(groups[0]).unwrap() == '#' {
                return 0;
            }
            return recursive_arrangements(
                records[groups[0] + 1..].to_string(),
                groups[1..].to_vec(),
            );
        } else {
            return recursive_arrangements(records[groups[0]..].to_string(), groups[1..].to_vec());
        }
    }

    unreachable!();
}

#[aoc(2023, 12)]
pub fn main() {
    let data = aoc_input!(2023, 12).unwrap();
    let input = parse(&data);

    // Part I
    let sum: usize = input.iter().map(|(r, a)| candidates(r, a)).sum();
    println!("{}", sum);

    //println!("{}", part1(&lines));

    // Part II
    let sum: usize = input
        .into_iter()
        .map(|(r, a)| {
            let v = a
                .iter()
                .cycle()
                .take(a.len() * 5)
                .copied()
                .collect::<Vec<_>>();
            let w = [r; 5].join("?");
            let c = recursive_arrangements(w, v.clone());
            println!("{}, {:?}, {}", r, v, c);
            c
        })
        .sum();
    println!("{}", sum);

    // println!("{}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE1: &str = "?###???????? 3,2,1";

    static EXAMPLE2: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_arrangement() {
        let records = "#....######..#####.";
        let expected = vec![1, 6, 5];
        assert_eq!(arrangement(records), expected);
    }

    #[test]
    fn test_part1() {
        let input = parse(EXAMPLE2);
        let sum: usize = input.iter().map(|(r, a)| candidates(r, a)).sum();
        assert_eq!(sum, 21);
    }

    #[test]
    fn test_part2() {
        let input = parse(EXAMPLE2);

        let sum: usize = input
            .into_iter()
            .map(|(r, a)| {
                let v = a
                    .iter()
                    .cycle()
                    .take(a.len() * 5)
                    .copied()
                    .collect::<Vec<_>>();
                let w = [r; 5].join("?");
                let c = recursive_arrangements(w, v.clone());
                println!("{}, {:?}, {}", r, v, c);
                c
            })
            .sum();
        //        assert_eq!(part2(&lines), 1);
    }
}
