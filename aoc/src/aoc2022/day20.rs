use aoc::{aoc, aoc_input};

fn modif(line: &[i64], t: usize) -> Vec<i64> {
    let mut data: Vec<_> = line.iter().enumerate().collect();
    let n = line.len();

    for _ in 0..t {
        for (i, val) in line.iter().enumerate() {
            let idx = data.iter().position(|x| *x == (i, val)).unwrap();
            data.remove(idx);
            data.insert(
                (idx as i64 + *val).rem_euclid(n as i64 - 1) as usize,
                (i, val),
            );
        }
    }

    data.iter().map(|(_, x)| **x).collect()
}

fn score(data: &[i64]) -> i64 {
    let indices = [1000, 2000, 3000];
    let n = data.len();

    indices
        .iter()
        .map(|i| {
            let zero = data.iter().position(|x| *x == 0).unwrap();
            let j = (zero + i).rem_euclid(n);
            data[j]
        })
        .sum()
}

#[aoc(2022, 20)]
pub fn main() {
    let data = aoc_input!(2022, 20).unwrap();
    let line: Vec<i64> = data.trim().lines().flat_map(|x| x.parse()).collect();

    // Part I
    println!("{}", score(&modif(&line, 1)));

    // Part II
    let decryption_key = 811589153;
    let line: Vec<_> = line.iter().map(|&x| decryption_key * x).collect();
    println!("{}", score(&modif(&line, 10)));
}
