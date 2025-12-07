use aoc::aoc_input;
use rustc_hash::FxHashSet;

fn intersect<'a>(mut iter: impl Iterator<Item = &'a FxHashSet<char>> + Clone) -> FxHashSet<char> {
    let first: FxHashSet<char> = iter.next().unwrap().clone();

    first
        .iter()
        .filter(move |elem| iter.clone().all(|set| set.contains(elem)))
        .cloned()
        .collect()
}

fn main() {
    let data = aoc_input!(2020, 6).unwrap();
    let answers: Vec<Vec<FxHashSet<char>>> = data
        .trim_end()
        .split("\n\n")
        .map(|lines| {
            lines
                .lines()
                .map(|line| line.chars().collect::<FxHashSet<char>>())
                .collect()
        })
        .collect();

    // Part I
    println!(
        "{}",
        answers
            .iter()
            .map(|g| g.iter().flatten().collect::<FxHashSet<_>>().len())
            .sum::<usize>()
    );

    // Part II
    println!(
        "{:?}",
        answers
            .iter()
            .map(|g| intersect(g.iter()).len())
            .sum::<usize>()
    );
}
