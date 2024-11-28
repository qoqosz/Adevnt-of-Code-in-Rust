use aoc::{aoc, aoc_input};
use std::rc::Rc;

fn is_aba(txt: &[u8]) -> bool {
    if txt[0] == txt[2] && txt[0] != txt[1] {
        return true;
    }
    false
}

fn is_abba(txt: &str) -> bool {
    for win in txt.as_bytes().windows(4) {
        if win[0] == win[3] && win[1] == win[2] && win[0] != win[1] {
            return true;
        }
    }
    false
}

#[derive(Debug)]
enum End<'a> {
    Text(&'a str),
    Address(Rc<Address<'a>>),
}

impl<'a> From<&'a str> for End<'a> {
    fn from(value: &'a str) -> Self {
        match value.contains('[') {
            true => End::Address(Rc::new(Address::from(value))),
            false => Self::Text(value),
        }
    }
}

#[derive(Debug)]
struct Address<'a> {
    left: &'a str,
    mid: &'a str,
    right: End<'a>,
}

impl<'a> From<&'a str> for Address<'a> {
    fn from(value: &'a str) -> Self {
        let (left, rem) = value.split_once('[').unwrap();
        let (mid, rest) = rem.split_once(']').unwrap();

        Self {
            left,
            mid,
            right: End::from(rest),
        }
    }
}

impl<'a> Address<'a> {
    fn is_valid(&self) -> bool {
        match &self.right {
            End::Text(_) => !is_abba(self.mid),
            End::Address(addr) => !is_abba(self.mid) && addr.is_valid(),
        }
    }

    fn is_tls(&self) -> bool {
        if !self.is_valid() {
            return false;
        }

        let is_left = is_abba(self.left);
        let is_right = match &self.right {
            End::Text(right) => is_abba(right),
            End::Address(addr) => addr.is_tls(), // mid may be invalid
                                                 // add is_valid?
        };
        is_left || is_right
    }

    fn aba_supernet(&self) -> &str {
        let mut aba = vec![];

        for win in self.left.as_bytes().windows(3) {
            if is_aba(&win) {
                aba.push(win);
            }
        }

        ""
    }
}

#[aoc(2016, 7)]
pub fn main() {
    let data = aoc_input!(2016, 7).unwrap();

    // Part I
    let count = data
        .lines()
        .filter(|x| !x.is_empty())
        .filter(|line| Address::from(*line).is_tls())
        .count();
    println!("{count}");

    // Part II
    // let count =
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
            let addr = Address::from(*case);
            assert_eq!(addr.is_tls(), exp);
        }
    }
}
