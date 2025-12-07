use aoc::aoc;
use num::integer::Roots;
use rustc_hash::FxHashMap;

#[aoc(2017, 3)]
pub fn main() {
    let n = 347991;

    // Part I - coordinates in Ulam's Spiral, adjusted
    let p = (4 * n + 1).sqrt();
    let q = n - (p * p / 4);
    let z: (i32, i32) = (q - (p + 1) / 4, (p + 2) / 4);

    println!("{}", z.0.abs() + z.1.abs() - 1);

    // Part II
    let mut grid = FxHashMap::default();
    grid.insert((0, 0), 1);
    let (mut i, mut j) = (0, 0);

    'outer: for s in (1..).step_by(2) {
        for (ds, di, dj) in vec![(0, 1, 0), (0, 0, -1), (1, -1, 0), (1, 0, 1)] {
            for _ in 0..s + ds {
                (i, j) = (i + di, j + dj);

                let mut tmp = 0;

                for k in i - 1..i + 2 {
                    for l in j - 1..j + 2 {
                        tmp += *grid.get(&(k, l)).unwrap_or(&0);
                    }
                }

                grid.insert((i, j), tmp);

                if tmp > n {
                    println!("{tmp}");
                    break 'outer;
                }
            }
        }
    }
}
