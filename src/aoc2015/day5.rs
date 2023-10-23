use std::collections::HashMap;

use aoc::aoc_input;

static VOWELS: &str = "aeiou";

fn has_at_least_three_vowels(txt: &str) -> bool {
    txt.chars()
        .map(|c| VOWELS.contains([c]) as usize)
        .sum::<usize>()
        >= 3
}

fn has_repeated_letter(txt: &str) -> bool {
    for window in txt.chars().collect::<Vec<_>>().windows(2) {
        if window[0] == window[1] {
            return true;
        }
    }
    false
}

fn does_not_have_selected(txt: &str) -> bool {
    for window in txt.chars().collect::<Vec<_>>().windows(2) {
        match window {
            ['a', 'b'] => return false,
            ['c', 'd'] => return false,
            ['p', 'q'] => return false,
            ['x', 'y'] => return false,
            _ => {}
        }
    }
    true
}

fn is_nice1(txt: &str) -> bool {
    has_at_least_three_vowels(txt) && has_repeated_letter(txt) && does_not_have_selected(txt)
}

fn ans1(data: &Vec<&str>) -> usize {
    data.iter().map(|txt| is_nice1(txt) as usize).sum()
}

fn is_in_between(txt: &str) -> bool {
    for window in txt.chars().collect::<Vec<_>>().windows(3) {
        if window[0] == window[2] {
            return true;
        }
    }
    false
}

fn has_pair_twice(txt: &str) -> bool {
    let mut visited: HashMap<(char, char), usize> = HashMap::new();

    for (i, window) in txt.chars().collect::<Vec<_>>().windows(2).enumerate() {
        let key = (window[0], window[1]);

        if visited.contains_key(&key) {
            let j = *visited.get(&key).unwrap();

            if i.abs_diff(j) > 1 {
                return true;
            }
        } else {
            visited.insert(key, i);
        }
    }
    false
}

fn is_nice2(txt: &str) -> bool {
    is_in_between(txt) && has_pair_twice(txt)
}

fn ans2(data: &Vec<&str>) -> usize {
    data.iter().map(|txt| is_nice2(txt) as usize).sum()
}

fn main() {
    let data = aoc_input!(2015, 5).unwrap();
    let data = data
        .split('\n')
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>();

    // Part I
    println!("{}", ans1(&data));

    // Part II
    println!("{}", ans2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert!(is_nice1("ugknbfddgicrmopn"));
    }

    #[test]
    fn test_case_2() {
        assert!(is_nice1("aaa"));
    }

    #[test]
    fn test_case_3() {
        assert!(!is_nice1("jchzalrnumimnmhp"));
    }

    #[test]
    fn test_case_4() {
        assert!(!is_nice1("haegwjzuvuyypxyu"));
    }

    #[test]
    fn test_case_5() {
        assert!(!is_nice1("dvszwmarrgswjxmb"));
    }
}
