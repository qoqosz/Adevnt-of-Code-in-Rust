use aoc::{aoc, aoc_input};
use itertools::Itertools;
use rustc_hash::FxHashMap;

fn parse_line(line: &str) -> (Vec<i16>, Vec<Vec<i16>>, Vec<i16>) {
    let (id, rem) = line.split_once("] ").unwrap();
    let (buttons, joltage) = rem.split_once('{').unwrap();

    let indicator = id[1..]
        .as_bytes()
        .iter()
        .map(|ch| if *ch == b'.' { 0 } else { 1 })
        .collect();
    let buttons = buttons
        .split(' ')
        .filter(|g| !g.is_empty())
        .map(|g| {
            g.trim()
                .trim_start_matches('(')
                .trim_end_matches(')')
                .split(',')
                .flat_map(|b| b.parse::<i16>())
                .collect::<Vec<_>>()
        })
        .collect();
    let joltage = joltage
        .trim_end_matches('}')
        .split(',')
        .flat_map(|j| j.parse::<i16>())
        .collect();

    (indicator, buttons, joltage)
}

fn all_binary_vectors(n: usize) -> impl Iterator<Item = Vec<i16>> {
    std::iter::repeat([0, 1].iter().copied())
        .take(n)
        .multi_cartesian_product()
}

fn diagram_from_buttons<'a>(
    n: usize,
    configuration: impl IntoIterator<Item = &'a Vec<i16>>,
) -> Vec<i16> {
    let mut out = vec![0; n];

    for buttons in configuration {
        for button in buttons {
            out[*button as usize] += 1;
        }
    }

    out
}

fn solve(target: &Vec<i16>, buttons: &Vec<Vec<i16>>, joltage: &Vec<i16>) -> (usize, usize) {
    let (n, m) = (target.len(), buttons.len());
    let mut schemas = FxHashMap::default();
    let mut patterns = FxHashMap::default();

    // Part I
    for cfg in all_binary_vectors(m) {
        let pattern = diagram_from_buttons(
            n,
            buttons
                .iter()
                .zip(cfg.iter())
                .filter_map(|(b, i)| if *i == 1 { Some(b) } else { None }),
        );
        schemas.insert(cfg.clone(), pattern.clone());
        let pattern = pattern.iter().map(|x| *x % 2).collect::<Vec<_>>();
        patterns.entry(pattern).or_insert(vec![]).push(cfg);
    }

    let p1 = patterns[target]
        .iter()
        .map(|v| v.iter().sum::<i16>() as usize)
        .min()
        .unwrap();

    // Part II
    let mut cache = FxHashMap::default();

    fn find(
        joltage: &Vec<i16>,
        patterns: &FxHashMap<Vec<i16>, Vec<Vec<i16>>>,
        schemas: &FxHashMap<Vec<i16>, Vec<i16>>,
        cache: &mut FxHashMap<Vec<i16>, usize>,
    ) -> usize {
        if joltage.iter().all(|x| *x == 0) {
            return 0;
        }
        if let Some(&cached) = cache.get(joltage) {
            return cached;
        }
        let mut res = usize::MAX;
        let indicator = joltage.iter().map(|x| *x % 2).collect::<Vec<_>>();
        let configurations = patterns
            .get(&indicator)
            .unwrap_or(&vec![])
            .iter()
            .cloned()
            .collect::<Vec<_>>();

        for cfg in configurations {
            if let Some(diff) = schemas.get(&cfg) {
                let new_joltage = joltage
                    .iter()
                    .zip(diff.iter())
                    .map(|(a, b)| (*a - *b) / 2)
                    .collect::<Vec<_>>();

                if new_joltage.iter().any(|x| *x < 0) {
                    continue;
                }

                let val = (cfg.iter().sum::<i16>() as usize).saturating_add(
                    2usize.saturating_mul(find(&new_joltage, patterns, schemas, cache)),
                );
                res = res.min(val);
            }
        }
        cache.insert(joltage.clone(), res);
        res
    }

    let p2 = find(joltage, &patterns, &schemas, &mut cache);

    (p1, p2)
}

#[aoc(2025, 10)]
pub fn main() {
    let data = aoc_input!(2025, 10).unwrap();

    let (p1, p2) = data
        .trim()
        .lines()
        .map(|line| {
            let (target, buttons, joltage) = parse_line(line);
            solve(&target, &buttons, &joltage)
        })
        .fold((0, 0), |acc, (x, y)| (acc.0 + x, acc.1 + y));

    // Part I
    println!("{p1}");

    // Part II
    println!("{p2}");
}
