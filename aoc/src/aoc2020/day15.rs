use aoc::aoc;
use rustc_hash::FxHashMap;

#[derive(Copy, Clone)]
enum Occurrence {
    First(usize),
    SeenBefore(usize, usize),
}

fn observe(cache: &mut FxHashMap<usize, Occurrence>, value: usize, i: usize) {
    cache
        .entry(value)
        .and_modify(
            |x| match *x {
                Occurrence::First(j) => *x = Occurrence::SeenBefore(i, j),
                Occurrence::SeenBefore(j, _) => *x = Occurrence::SeenBefore(i, j),
            }, //*x = Occurrence::SeenBefore(i))
        )
        .or_insert(Occurrence::First(i));
}

fn solve(input: &[usize], n: usize) -> usize {
    let mut cache: FxHashMap<usize, Occurrence> = FxHashMap::default();
    let warmup = input.len();
    let mut prev = 0; // cutting corners

    for i in 1..(n + 1) {
        if i <= warmup {
            prev = input[i - 1];
            observe(&mut cache, prev, i);
            continue;
        }

        match cache.get(&prev) {
            Some(Occurrence::First(_)) => prev = 0,
            Some(Occurrence::SeenBefore(m, n)) => prev = m - n,
            _ => unreachable!(),
        }

        observe(&mut cache, prev, i);
    }

    prev
}

#[aoc(2020, 15)]
pub fn main() {
    let data = vec![0, 14, 6, 20, 1, 4];

    // Part I
    println!("{}", solve(&data, 2020));

    // Part II
    println!("{}", solve(&data, 30000000));
}
