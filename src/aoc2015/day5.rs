use aoc::aoc_input;
use rustc_hash::FxHashMap;

static VOWELS: &str = "aeiou";

fn has_at_least_three_vowels(txt: &str) -> bool {
    txt.chars()
        .map(|c| VOWELS.contains([c]) as usize)
        .sum::<usize>()
        >= 3
}

fn has_repeated_letter(txt: &str) -> bool {
    txt.chars()
        .collect::<Vec<_>>()
        .windows(2)
        .any(|window| window[0] == window[1])
}

fn does_not_have_selected(txt: &str) -> bool {
    !txt.chars()
        .collect::<Vec<_>>()
        .windows(2)
        .any(|window| matches!(window, ['a', 'b'] | ['c', 'd'] | ['p', 'q'] | ['x', 'y']))
}

fn is_nice1(txt: &str) -> bool {
    has_at_least_three_vowels(txt) && has_repeated_letter(txt) && does_not_have_selected(txt)
}

fn ans1(data: &[&str]) -> usize {
    data.iter().filter(|x| is_nice1(x)).count()
}

fn is_in_between(txt: &str) -> bool {
    txt.chars()
        .collect::<Vec<_>>()
        .windows(3)
        .any(|window| window[0] == window[2])
}

fn has_pair_twice(txt: &str) -> bool {
    let mut visited: FxHashMap<(char, char), usize> = FxHashMap::default();

    txt.chars()
        .collect::<Vec<_>>()
        .windows(2)
        .enumerate()
        .any(|(i, window)| {
            let key = (window[0], window[1]);
            let j = visited.entry(key).or_insert(i);
            i.abs_diff(*j) > 1
        })
}

fn is_nice2(txt: &str) -> bool {
    is_in_between(txt) && has_pair_twice(txt)
}

fn ans2(data: &[&str]) -> usize {
    data.iter().filter(|txt| is_nice2(txt)).count()
}

fn main() {
    let data = aoc_input!(2015, 5).unwrap();
    let data = data.lines().filter(|x| !x.is_empty()).collect::<Vec<_>>();

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
