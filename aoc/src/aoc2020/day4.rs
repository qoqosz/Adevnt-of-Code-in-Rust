use aoc::{aoc, aoc_input};
use lazy_static::lazy_static;
use rustc_hash::{FxHashMap, FxHashSet};

lazy_static! {
    static ref FIELDS: FxHashSet<&'static str> =
        FxHashSet::from_iter(["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"]);
    static ref ECL: FxHashSet<&'static str> =
        FxHashSet::from_iter(["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]);
}

fn is_valid1(passport: &&FxHashMap<&str, &str>) -> bool {
    if FIELDS.iter().all(|&k| passport.contains_key(k)) {
        return true;
    }
    if passport.keys().len() == 7 {
        return !passport.contains_key(&"cid");
    }
    false
}

fn is_valid2(passport: &&FxHashMap<&str, &str>) -> bool {
    if !is_valid1(passport) {
        return false;
    }

    let is_byr = (1920..=2002).contains(&passport[&"byr"].parse::<usize>().unwrap());
    let is_iyr = (2010..=2020).contains(&passport[&"iyr"].parse::<usize>().unwrap());
    let is_eyr = (2020..=2030).contains(&passport[&"eyr"].parse::<usize>().unwrap());

    let hgt = passport[&"hgt"];
    let (hgt_val, hgt_unit) = hgt.split_at(hgt.len() - 2);
    let hgt_val = hgt_val.parse::<usize>().unwrap_or_default();

    let is_hgt = match hgt_unit {
        "in" => (59..=76).contains(&hgt_val),
        "cm" => (150..=193).contains(&hgt_val),
        _ => false,
    };

    let hcl = passport[&"hcl"];
    let is_hcl =
        hcl.starts_with('#') && hcl.len() == 7 && hcl[1..].chars().all(|x| x.is_ascii_hexdigit());

    let is_ecl = ECL.contains(passport[&"ecl"]);

    let pid = passport[&"pid"];
    let is_pid = pid.chars().all(|x| x.is_ascii_digit()) && pid.len() == 9;

    [is_byr, is_iyr, is_eyr, is_hgt, is_hcl, is_ecl, is_pid]
        .iter()
        .all(|x| *x)
}

#[aoc(2020, 4)]
pub fn main() {
    let data = aoc_input!(2020, 4).unwrap();
    let passports = data
        .split("\n\n")
        .map(|line| line.split([' ', '\n']).collect::<Vec<_>>())
        .map(|line| {
            line.iter()
                .filter_map(|word| word.split_once(':'))
                .collect()
        })
        .collect::<Vec<FxHashMap<&str, &str>>>();

    // Part I
    println!("{}", passports.iter().filter(is_valid1).count());

    // Part II
    println!("{}", passports.iter().filter(is_valid2).count());
}
