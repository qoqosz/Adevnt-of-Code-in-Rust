use aoc::{aoc, aoc_input};
use rustc_hash::FxHashSet;
use std::collections::VecDeque;

fn parse(data: &str) -> Vec<[i32; 3]> {
    data.trim()
        .lines()
        .flat_map(|line| {
            let v: Vec<_> = line.split(',').flat_map(|x| x.parse::<i32>()).collect();
            v.try_into()
        })
        .collect()
}

fn neighbors3d(p: &[i32; 3]) -> Vec<[i32; 3]> {
    let mut ns = vec![];

    for i in [-1, 1] {
        ns.push([p[0] + i, p[1], p[2]]);
        ns.push([p[0], p[1] + i, p[2]]);
        ns.push([p[0], p[1], p[2] + i]);
    }

    ns
}

fn count_faces(data: &[[i32; 3]]) -> usize {
    let mut n_total = 0;

    for p in data {
        for n in neighbors3d(p) {
            if data.contains(&n) {
                n_total += 1;
            }
        }
    }

    data.len() * 6 - n_total
}

fn lava_fill(data: &[[i32; 3]]) -> usize {
    let mut visited = FxHashSet::from_iter([[0, 0, 0]]);
    let mut queue = VecDeque::from_iter([[0, 0, 0]]);
    let mut n_faces = 0;

    while let Some(node) = queue.pop_front() {
        for w in neighbors3d(&node) {
            if w.iter().any(|&x| -1 > x || x > 20) {
                continue;
            }
            if data.contains(&w) {
                n_faces += 1;
            } else {
                if visited.insert(w) {
                    queue.push_back(w);
                }
            }
        }
    }

    n_faces
}

#[aoc(2022, 18)]
pub fn main() {
    let data = aoc_input!(2022, 18).unwrap();
    let data = parse(&data);

    // Part I
    println!("{}", count_faces(&data));

    // Part II
    println!("{}", lava_fill(&data));
}
