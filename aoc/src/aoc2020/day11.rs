use aoc::{aoc, aoc_input};
use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::fmt;
use std::hash::Hasher;

#[derive(Debug)]
struct Layout {
    grid: FxHashMap<(i32, i32), bool>,
    max_x: i32,
    max_y: i32,
}

impl Layout {
    fn new(grid: FxHashMap<(i32, i32), bool>) -> Self {
        let max_x = grid.keys().max_by_key(|k| k.0).unwrap().0;
        let max_y = grid.keys().max_by_key(|k| k.1).unwrap().1;
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

    fn simulate1(&mut self) {
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

    fn simulate2(&mut self) {
        let mut out = FxHashMap::default();

        for (k, v) in self.grid.iter() {
            let n_occupied = self.seat_count_distant(k);
            let new_v = match (v, n_occupied) {
                (false, 0) => true,
                (true, 5..) => false,
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

    fn contains(&self, key: &(i32, i32)) -> bool {
        (0..=self.max_x).contains(&key.0) && (0..=self.max_y).contains(&key.1)
    }

    fn seat_count_distant(&self, key: &(i32, i32)) -> usize {
        let mut count = 0;
        let pos0 = *key;

        for dir in self.adj_dir() {
            let mut i = 1;

            loop {
                let pos = (pos0.0 + i * dir.0, pos0.1 + i * dir.1);

                if !self.contains(&pos) {
                    break;
                }

                if let Some(sym) = self.grid.get(&pos) {
                    if *sym {
                        count += 1;
                    }
                    break;
                }

                i += 1;
            }
        }

        count
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
                        .filter_map(move |(x, ch)| match *ch {
                            b'L' => Some(((x as i32, y as i32), false)),
                            b'#' => Some(((x as i32, y as i32), true)),
                            _ => None,
                        })
                })
                .collect(),
        )
    }
}

fn solver<F>(layout: &mut Layout, mut f: F) -> usize
where
    F: FnMut(&mut Layout),
{
    let mut id = layout.hash();

    loop {
        f(layout);
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
    let seat_count1 = solver(&mut layout, |l| l.simulate1());
    println!("{}", seat_count1);

    let mut layout = Layout::from(data.as_str());
    let seat_count2 = solver(&mut layout, |l| l.simulate2());
    println!("{}", seat_count2);
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE1: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    static EXAMPLE2: &str = ".......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#.....";

    #[test]
    fn test_part1() {
        let mut layout = Layout::from(EXAMPLE1);
        let seat_count = solver(&mut layout, |l| l.simulate1());
        assert_eq!(seat_count, 37);
    }

    #[test]
    fn test_part2_occupied_seats() {
        let layout = Layout::from(EXAMPLE2);
        assert_eq!(layout.seat_count_distant(&(3, 4)), 8);
    }

    #[test]
    fn test_part2() {
        let mut layout = Layout::from(EXAMPLE1);
        let seat_count = solver(&mut layout, |l| l.simulate2());
        assert_eq!(seat_count, 26);
    }
}
