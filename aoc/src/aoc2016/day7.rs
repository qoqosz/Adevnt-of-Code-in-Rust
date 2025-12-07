use aoc::{aoc, aoc_input};
use rustc_hash::FxHashSet;

fn is_abba(txt: &[u8]) -> bool {
    txt[0] != txt[1] && txt[0] == txt[3] && txt[1] == txt[2]
}

fn is_tls(txt: &[u8]) -> bool {
    let mut in_hypernet = false;
    let mut is_support = false;

    for win in txt.windows(4) {
        match win[0] {
            b'[' => in_hypernet = true,
            b']' => in_hypernet = false,
            _ => {
                if is_abba(win) {
                    if in_hypernet {
                        return false;
                    }
                    is_support = true;
                }
            }
        }
    }

    is_support
}

fn is_aba(txt: &[u8]) -> bool {
    txt[0] != txt[1] && txt[0] == txt[2]
}

fn is_ssl(txt: &[u8]) -> bool {
    let mut aba_set = FxHashSet::default();
    let mut bab_set = FxHashSet::default();
    let mut in_hypernet = false;

    for win in txt.windows(3) {
        match win[0] {
            b'[' => in_hypernet = true,
            b']' => in_hypernet = false,
            _ => {
                if is_aba(win) {
                    let (a, b) = (win[0], win[1]);

                    if in_hypernet {
                        bab_set.insert((b, a));
                    } else {
                        aba_set.insert((a, b));
                    }
                }
            }
        }
    }

    aba_set.intersection(&bab_set).count() > 0
}

#[aoc(2016, 7)]
pub fn main() {
    let data = aoc_input!(2016, 7).unwrap();

    // Part I
    let n = data
        .trim()
        .lines()
        .filter(|ip| is_tls(ip.as_bytes()))
        .count();
    println!("{n}");

    // Part II
    let n = data
        .trim()
        .lines()
        .filter(|ip| is_ssl(ip.as_bytes()))
        .count();
    println!("{n}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let examples = vec![
            "abba[mnop]qrst",
            "abcd[bddb]xyyx",
            "aaaa[qwer]tyui",
            "ioxxoj[asdfgh]zxcvbn",
        ];
        let expected = [true, false, false, true];

        for (case, exp) in examples.iter().zip(expected) {
            assert_eq!(is_tls(case.as_bytes()), exp);
        }
    }

    #[test]
    fn test_part2() {
        let examples = vec!["aba[bab]xyz", "xyx[xyx]xyx", "aaa[kek]eke", "zazbz[bzb]cdb"];
        let expected = [true, false, true, true];

        for (case, exp) in examples.iter().zip(expected) {
            assert_eq!(is_ssl(case.as_bytes()), exp);
        }
    }
}
