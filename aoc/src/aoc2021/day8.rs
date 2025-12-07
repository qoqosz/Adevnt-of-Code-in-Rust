use aoc::{aoc, aoc_input};
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::{num::ParseIntError, str::Utf8Error, sync::LazyLock};

static CHARS: &'static str = "abcdefg";
static ENCODING: LazyLock<FxHashMap<usize, &'static str>> = LazyLock::new(|| {
    FxHashMap::from_iter([
        (0, "abcefg"),
        (1, "cf"),
        (2, "acdeg"),
        (3, "acdfg"),
        (4, "bcdf"),
        (5, "abdfg"),
        (6, "abdefg"),
        (7, "acf"),
        (8, "abcdefg"),
        (9, "abcdfg"),
    ])
});
static DECODING: LazyLock<FxHashMap<&'static str, usize>> = LazyLock::new(|| {
    ENCODING
        .iter()
        .map(|(k, v)| (*v, *k))
        .collect::<FxHashMap<_, _>>()
});
static PERMUTATIONS: LazyLock<Vec<Vec<u8>>> = LazyLock::new(|| {
    let n = CHARS.len();
    CHARS.bytes().permutations(n).collect()
});
static TARGET: LazyLock<FxHashSet<&&'static str>> =
    LazyLock::new(|| FxHashSet::from_iter(ENCODING.values()));

fn find_permutation(line: &str) -> Option<String> {
    for p in PERMUTATIONS.iter() {
        let permutation = str::from_utf8(p).ok()?;
        let signal = translate(line, permutation).ok()?;

        if signal
            .split_whitespace()
            .map(sort_word)
            .all(|w| TARGET.contains(&w.as_str()))
        {
            return Some(permutation.to_owned());
        }
    }
    None
}

fn output_value(output: &str, permutation: &str) -> Result<usize, ParseIntError> {
    match translate(output, permutation) {
        Ok(tr) => tr
            .split(' ')
            .map(sort_word)
            .flat_map(|w| DECODING.get(w.as_str()))
            .join("")
            .parse::<usize>(),
        _ => "a12".parse::<usize>(),
    }
}

fn translate(signal: &str, permutation: &str) -> Result<String, Utf8Error> {
    let dict: FxHashMap<u8, u8> = permutation.bytes().zip(CHARS.bytes()).collect();
    let out = signal
        .bytes()
        .map(move |ch| *dict.get(&ch).unwrap_or(&ch))
        .collect::<Vec<_>>();
    str::from_utf8(&out).map(|x| x.to_owned())
}

fn sort_word(word: &str) -> String {
    word.chars().sorted().collect()
}

fn parse(line: &str) -> (&str, &str) {
    let (lhs, rhs) = line.split_once('|').unwrap();
    (lhs.trim(), rhs.trim())
}

#[aoc(2021, 8)]
pub fn main() {
    let data = aoc_input!(2021, 8).unwrap();
    let entries = data.trim().lines().map(parse).collect::<Vec<_>>();
    let (mut count, mut outval) = (0, 0);

    // Part I
    let expected = [2, 3, 4, 7];

    for (signal, output) in entries {
        // Part I
        count += output
            .split_whitespace()
            .filter(|x| expected.contains(&x.len()))
            .count();
        // Part II
        if let Some(permutation) = find_permutation(signal) {
            outval += output_value(output, &permutation).unwrap_or_default();
        }
    }

    println!("{count}\n{outval}");
}
