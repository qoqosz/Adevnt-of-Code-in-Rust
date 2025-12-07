use itertools::Itertools;

fn increment(password: impl Into<String>) -> String {
    let mut chars = password.into().chars().rev().collect::<Vec<_>>();

    for c in chars.iter_mut() {
        match c {
            'z' => *c = 'a',
            _ => {
                *c = (*c as u8 + 1) as char;
                break;
            }
        }
    }
    chars.iter().rev().join("")
}

fn is_inc(password: impl Into<String>) -> bool {
    // must include one increasing straight of at least three letters
    for win in password
        .into()
        .chars()
        .map(|c| c as u8)
        .collect::<Vec<u8>>()
        .windows(3)
    {
        if (win[0] + 1 == win[1]) && (win[1] + 1) == win[2] {
            return true;
        }
    }
    false
}

fn is_forbidden(password: impl Into<String>) -> bool {
    // Passwords may not contain the letters i, o, or l
    password.into().contains(['i', 'o', 'l'])
}

fn has_pairs(password: impl Into<String>) -> bool {
    // Passwords must contain at least two different, non-overlapping pairs of letters
    let mut idx: usize = usize::MAX;
    let mut ch: char = '.';

    for (i, win) in password
        .into()
        .chars()
        .collect::<Vec<_>>()
        .windows(2)
        .enumerate()
    {
        if win[0] == win[1] {
            if idx == usize::MAX {
                idx = i;
                ch = win[0];
            } else if (i != idx + 1) && win[0] != ch {
                return true;
            }
        }
    }
    false
}

fn validate(password: impl Into<String>) -> bool {
    let txt = password.into();
    is_inc(&txt) && !is_forbidden(&txt) && has_pairs(&txt)
}

fn next_password(password: impl Into<String>) -> String {
    let mut next_password = increment(password);

    while !validate(&next_password) {
        next_password = increment(&next_password);
    }

    next_password
}

pub fn main() {
    let data = "hepxcrrq";

    // Part I
    let next = next_password(data);
    println!("{}", next);

    // Part II
    println!("{}", next_password(next));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment1() {
        assert_eq!(increment("aa"), "ab");
    }

    #[test]
    fn test_increment2() {
        assert_eq!(increment("bzz"), "caa");
    }

    #[test]
    fn test_increment3() {
        assert_eq!(increment("cazbzz"), "cazcaa");
    }

    #[test]
    fn test_is_inc() {
        assert!(is_inc("hijklmmn"));
    }

    #[test]
    fn test_is_inc2() {
        assert!(!is_inc("ghjaaabb"));
    }

    #[test]
    fn test_is_forbidden() {
        assert!(is_forbidden("hijklmmn"));
    }

    #[test]
    fn test_has_pairs() {
        assert!(has_pairs("abbceffg"));
    }

    #[test]
    fn test_has_no_pairs() {
        assert!(!has_pairs("abbcegjk"));
    }

    #[test]
    fn test_is_pass() {
        assert!(validate("abcdffaa"));
    }

    #[test]
    fn test_next_password() {
        let next = next_password("ghjaaaaa");
        assert_eq!(next, "ghjaabcc".to_string());
    }

    #[test]
    fn test_example_password() {
        let p = "ghjaabcc".to_string();
        assert!(is_inc(&p));
        assert!(!is_forbidden(&p));
        assert!(has_pairs(&p));
    }
}
