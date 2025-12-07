use aoc::aoc;
use rustc_hash::FxHashMap;

#[derive(Copy, Clone)]
enum Occurrence {
    First(usize),
    SeenBefore(usize, usize),
}

fn solve(input: &[usize], n: usize) -> usize {
    let mut cache: FxHashMap<usize, Occurrence> = FxHashMap::default();
    let (mut speak, warmup) = (0, input.len());
    let mut prev = &Occurrence::First(0); // cutting corners

    for i in 1..=n {
        speak = match (i, &prev) {
            (j, _) if j <= warmup => input[j - 1],
            (_, Occurrence::First(_)) => 0,
            (_, Occurrence::SeenBefore(m, n)) => m - n,
        };
        prev = cache
            .entry(speak)
            .and_modify(|x| {
                *x = match *x {
                    Occurrence::First(j) | Occurrence::SeenBefore(j, _) => {
                        Occurrence::SeenBefore(i, j)
                    }
                }
            })
            .or_insert(Occurrence::First(i));
    }

    speak
}

#[aoc(2020, 15)]
pub fn main() {
    let data = vec![0, 14, 6, 20, 1, 4];

    // Part I
    println!("{}", solve(&data, 2020));

    // Part II
    println!("{}", solve(&data, 30000000));
}
