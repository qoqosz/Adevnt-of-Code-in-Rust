use aoc::{aoc, aoc_input};
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::ops::AddAssign;

#[derive(Default, Clone)]
struct EnergyState {
    grid: FxHashMap<(i32, i32), i32>,
}

impl EnergyState {
    fn increase_all(&mut self) {
        for v in self.grid.values_mut() {
            *v += 1;
        }
    }

    fn adj(&self, pos: &(i32, i32)) -> impl Iterator<Item = (i32, i32)> {
        [-1, 0, 1]
            .iter()
            .cartesian_product([-1, 0, 1].iter())
            .filter(|(i, j)| **i != 0 || **j != 0)
            .map(move |(i, j)| (pos.0 + *i, pos.1 + *j))
            .filter(|p| self.grid.contains_key(p))
    }
}

impl AddAssign for EnergyState {
    fn add_assign(&mut self, rhs: Self) {
        for (k, v) in rhs.grid {
            self.grid.entry(k).and_modify(|c| *c += v).or_insert(v);
        }
    }
}

impl TryFrom<&str> for EnergyState {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let grid = value
            .trim()
            .lines()
            .enumerate()
            .flat_map(|(i, line)| {
                line.as_bytes()
                    .iter()
                    .enumerate()
                    .map(move |(j, ch)| ((i as i32, j as i32), (*ch - b'0') as i32))
            })
            .collect();
        Ok(Self { grid })
    }
}

#[derive(Default)]
struct Cavern {
    energy: EnergyState,
    n_flashes: usize,
    n_steps: usize,
}

impl Cavern {
    fn new(energy: EnergyState) -> Self {
        Self {
            energy,
            ..Default::default()
        }
    }

    fn flash(&mut self, pos: &(i32, i32)) -> EnergyState {
        self.n_flashes += 1;
        self.energy.grid.insert(*pos, 0);
        let grid = self.energy.adj(pos).map(|n| (n, 1)).collect();

        EnergyState { grid }
    }

    fn step(&mut self, n: usize) {
        for _ in 0..n {
            self.n_steps += 1;
            self.energy.increase_all();
            let mut flashed = FxHashSet::default();

            loop {
                let mut update = EnergyState::default();
                let grid = self.energy.grid.clone();

                for (k, v) in grid {
                    if v > 9 && !flashed.contains(&k) {
                        update += self.flash(&k);
                        flashed.insert(k);
                    }
                }

                if update.grid.is_empty() {
                    break;
                }

                self.energy += update;
            }

            for k in flashed {
                self.energy.grid.insert(k, 0);
            }
        }
    }
}

#[aoc(2021, 11)]
pub fn main() {
    let data = aoc_input!(2021, 11).unwrap();
    let energy = EnergyState::try_from(data.as_str()).unwrap();

    // Part I
    let mut c = Cavern::new(energy.clone());
    c.step(100);
    println!("{:?}", c.n_flashes);

    // Part II
    let mut c = Cavern::new(energy);
    let mut prev;
    let n_octopus = c
        .energy
        .grid
        .keys()
        .max()
        .map(|(a, b)| (*a as usize + 1) * (*b as usize + 1))
        .unwrap();

    loop {
        prev = c.n_flashes;
        c.step(1);

        if c.n_flashes - prev == n_octopus {
            break;
        }
    }

    println!("{}", c.n_steps);
}
