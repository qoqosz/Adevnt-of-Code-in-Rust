use std::collections::HashSet;

use aoc::aoc_input;
use itertools::Itertools;

fn generate_molecultes(rules: &Vec<(&str, &str)>, molecule: &str) -> HashSet<String> {
    let mut molecules = HashSet::new();

    for (pattern, replacement) in rules {
        let n = pattern.len();
        let matches = molecule
            .match_indices(pattern)
            .map(|(i, _)| i)
            .collect::<Vec<_>>();

        for m in matches {
            let mut new_molecule = molecule.to_owned();
            new_molecule.replace_range(m..(m + n), replacement);
            molecules.insert(new_molecule);
        }
    }

    molecules
}

fn search(start: &str, end: &str, rules: &[(&str, &str)]) -> usize {
    // Invert & sort the rules
    let rules = rules
        .iter()
        .sorted_by_key(|(_, r)| r.len())
        .rev()
        .map(|(a, b)| (*b, *a))
        .collect::<Vec<_>>();

    let mut queue: HashSet<String> = HashSet::from([start.to_string()]);
    let mut n = 0;

    loop {
        n += 1;

        let next_queue = queue
            .iter()
            .flat_map(|state| generate_molecultes(&rules, state))
            .collect::<HashSet<String>>();

        if next_queue.contains(end) {
            return n;
        }

        // Some heuristics...
        let mut max_len = next_queue.iter().map(|x| x.len()).min().unwrap();
        if n < 10 {
            max_len += 1;
        }

        queue = HashSet::from_iter(
            next_queue
                .into_iter()
                .filter(|m| !m.contains('e') && m.len() <= max_len)
                .take(10), // heuristic...
        );
    }
}

fn main() {
    let data = aoc_input!(2015, 19).unwrap();
    let lines = data.lines().filter(|x| !x.is_empty()).collect::<Vec<_>>();
    let rules = lines
        .iter()
        .filter(|x| x.contains("=>"))
        .map(|x| {
            let parts = x.split(" => ").collect::<Vec<_>>();
            (parts[0], parts[1])
        })
        .collect::<Vec<_>>();
    let molecule = lines.last().unwrap().to_string();

    // Part I
    println!("{}", generate_molecultes(&rules, &molecule).len());

    // Part II
    println!("{}", search(&molecule, "e", &rules));
}
