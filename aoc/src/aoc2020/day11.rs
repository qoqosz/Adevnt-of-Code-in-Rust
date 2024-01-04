use aoc::{aoc, aoc_input};
use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::fmt;
use std::hash::Hasher;

struct Layout {
    grid: FxHashMap<(i32, i32), bool>,
    max_x: i32,
    max_y: i32,
}

impl Layout {
    fn new(grid: FxHashMap<(i32, i32), bool>) -> Self {
        let (max_x, max_y) = *grid.keys().max().unwrap_or(&(0, 0));
        Self { grid, max_x, max_y }
    }

    fn adj_dir(&self) -> impl Iterator<Item = (i32, i32)> {
        (-1..=1)
            .cartesian_product(-1..=1)
            .filter(|(dx, dy)| *dx != 0 || *dy != 0)
    }

    fn adj<'a>(&'a self, key: &'a (i32, i32)) -> impl Iterator<Item = (i32, i32)> + 'a {
        self.adj_dir().map(|(dx, dy)| (key.0 + dx, key.1 + dy))
    }

    fn simulate(&mut self) {
        let mut out = FxHashMap::default();

        for (k, v) in self.grid.iter() {
            let n_occupied = self
                .adj(k)
                .filter_map(|n| self.grid.get(&n))
                .filter(|n| **n)
                .count();
            let new_v = match (v, n_occupied) {
                (false, 0) => true,
                (true, 4..) => false,
                (val, _) => *val,
            };
            out.insert(*k, new_v);
        }

        self.grid = out;
    }

    fn seat_count(&self) -> usize {
        self.grid.values().filter(|v| **v).count()
    }

    fn hash(&self) -> u64 {
        let mut hasher = rustc_hash::FxHasher::default();
        hasher.write(self.to_string().as_bytes());
        hasher.finish()
    }
}

impl fmt::Display for Layout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..=self.max_y {
            let row = (0..=self.max_x)
                .map(move |x| {
                    self.grid
                        .get(&(x, y))
                        .map_or_else(|| '.', |c| if *c { '#' } else { 'L' })
                })
                .collect::<String>();
            writeln!(f, "{}", row)?;
        }
        Ok(())
    }
}

impl From<&str> for Layout {
    fn from(value: &str) -> Self {
        Self::new(
            value
                .trim()
                .lines()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.as_bytes()
                        .iter()
                        .enumerate()
                        .filter(|(_, ch)| **ch == b'L')
                        .map(move |(x, _)| ((x as i32, y as i32), false))
                })
                .collect(),
        )
    }
}

fn find_balance(layout: &mut Layout) -> usize {
    let mut id = layout.hash();

    loop {
        layout.simulate();
        let new_id = layout.hash();

        if new_id == id {
            return layout.seat_count();
        }
        id = new_id;
    }
}

#[aoc(2020, 11)]
pub fn main() {
    let data = aoc_input!(2020, 11).unwrap();
    let mut layout = Layout::from(data.as_str());
    let seat_count = find_balance(&mut layout);
    println!("{}", seat_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn test_part1() {
        let mut layout = Layout::from(EXAMPLE);
        let seat_count = find_balance(&mut layout);
        println!("{}", seat_count);
    }
}
