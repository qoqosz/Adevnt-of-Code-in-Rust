use aoc::{aoc, aoc_input};
use std::collections::HashSet;

struct Containers {
    containers: Vec<i32>,
}

impl From<Vec<i32>> for Containers {
    fn from(value: Vec<i32>) -> Self {
        Containers { containers: value }
    }
}

impl Containers {
    fn partitions(&self, total: i32) -> ContainersPartitionsIter<'_> {
        ContainersPartitionsIter::new(&self.containers, total)
    }
}

struct ContainersPartitionsIter<'a> {
    containers: &'a Vec<i32>,
    queue: Vec<(i32, Vec<i32>)>,
    visited: HashSet<(i32, Vec<i32>)>,
}

impl<'a> ContainersPartitionsIter<'a> {
    fn new(containers: &'a Vec<i32>, total: i32) -> Self {
        let coeffs = vec![0; containers.len()];

        ContainersPartitionsIter {
            containers,
            queue: vec![(total, coeffs)],
            visited: HashSet::new(),
        }
    }
}

impl<'a> Iterator for ContainersPartitionsIter<'a> {
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(state) = self.queue.pop() {
            if self.visited.contains(&state) || state.0 < 0 {
                continue;
            }
            self.visited.insert(state.clone());
            let (rem, coeffs) = state;

            if rem == 0 {
                return Some(coeffs);
            }

            for (i, qty) in self.containers.iter().enumerate() {
                let mut tmp = coeffs.clone();

                match coeffs[i] {
                    0 => {
                        tmp[i] += 1;
                        self.queue.push((rem - qty, tmp));
                    }
                    _ => continue,
                }
            }
        }
        None
    }
}

#[aoc(2015, 17)]
pub fn main() {
    let data = aoc_input!(2015, 17).unwrap();
    let containers = Containers::from(
        data.lines()
            .flat_map(|x| x.parse::<i32>())
            .collect::<Vec<_>>(),
    );

    let mut n_combinations = 0;
    let mut n_min_combinations = 0;
    let mut n_min_containers = i32::MAX;

    for partition in containers.partitions(150) {
        n_combinations += 1;

        let n_containers = partition.iter().filter(|&x| *x > 0).count() as i32;

        if n_containers < n_min_containers {
            n_min_combinations = 1;
            n_min_containers = n_containers;
        } else if n_containers == n_min_containers {
            n_min_combinations += 1;
        }
    }
    // Part I
    println!("{}", n_combinations);

    // Part II
    println!("{}", n_min_combinations);
}
