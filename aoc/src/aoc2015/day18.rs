use aoc::{aoc, aoc_input};
use std::fmt;

struct Grid {
    size_x: usize,
    size_y: usize,
    grid: Vec<bool>,
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut lines = vec![];

        for i in 0..self.size_y {
            let start = i * self.size_x;
            let end = start + self.size_x;
            let line: String = (start..end)
                .map(|i| match self.grid.get(i) {
                    Some(true) => '#',
                    Some(false) => '.',
                    _ => ' ',
                })
                .collect::<String>();
            lines.push(line);
        }
        write!(f, "{}", lines.join("\n"))
    }
}

impl<S> From<S> for Grid
where
    S: Into<String>,
{
    fn from(value: S) -> Self {
        let value = value.into();
        let size_x = value.find('\n').unwrap();
        let grid = value
            .chars()
            .filter(|x| *x != '\n')
            .map(|x| x == '#')
            .collect::<Vec<bool>>();
        let size_y = grid.len() / size_x;

        Grid {
            size_x,
            size_y,
            grid,
        }
    }
}

impl Grid {
    /// Convert 2D index into 1D
    fn idx(&self, x: usize, y: usize) -> usize {
        self.size_x * y + x
    }

    /// 1D indices of neighbors
    fn neighbors(&self, x: usize, y: usize) -> Vec<usize> {
        let mut ns = vec![];

        for i in [-1, 0, 1] {
            for j in [-1, 0, 1] {
                if i == 0 && j == 0 {
                    continue;
                }
                ns.push((x as i32 + i, y as i32 + j))
            }
        }

        ns.iter()
            .filter(|&p| {
                p.0 >= 0 && p.0 < self.size_x as i32 && p.1 >= 0 && p.1 < self.size_y as i32
            })
            .map(|p| self.idx(p.0 as usize, p.1 as usize))
            .collect::<Vec<_>>()
    }

    /// Number of neighbors that is on
    fn on_neighbors(&self, x: usize, y: usize) -> usize {
        self.neighbors(x, y)
            .iter()
            .filter(|&idx| self.grid[*idx])
            .count()
    }

    /// Part I logic
    fn step(&mut self) {
        let mut out = vec![false; self.size_x * self.size_y];

        for i in 0..self.size_x {
            for j in 0..self.size_y {
                let idx = self.idx(i, j);
                let n_on = self.on_neighbors(i, j);
                let new_state = match self.grid[idx] {
                    true => matches!(n_on, 2 | 3),
                    false => matches!(n_on, 3),
                };
                out[idx] = new_state;
            }
        }
        self.grid = out;
    }

    /// Part I simulation
    fn sim(&mut self, n: usize) {
        for _ in 0..n {
            self.step();
        }
    }

    /// Fix corners for Part II
    fn fix_corners(&mut self) {
        for i in [
            0,
            self.size_y - 1,
            self.idx(0, self.size_y - 1),
            self.idx(self.size_x - 1, self.size_y - 1),
        ] {
            self.grid[i] = true;
        }
    }

    /// Count lights on in the grid
    fn count(&self) -> usize {
        self.grid.iter().filter(|&x| *x).count()
    }
}

#[aoc(2015, 18)]
pub fn main() {
    let data = aoc_input!(2015, 18).unwrap();
    let mut grid = Grid::from(&data);
    let n = 100;

    // Part I
    grid.sim(n);
    println!("{}", grid.count());

    // Part II
    grid = Grid::from(&data);
    grid.fix_corners();

    for _ in 0..n {
        grid.step();
        grid.fix_corners();
    }

    println!("{}", grid.count());
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = ".#.#.#\n\
                            ...##.\n\
                            #....#\n\
                            ..#...\n\
                            #.#..#\n\
                            ####..";

    #[test]
    fn test_on_neighbors() {
        let grid = Grid::from(EXAMPLE);
        assert_eq!(grid.on_neighbors(0, 0), 1);
    }

    #[test]
    fn test_part1() {
        let mut grid = Grid::from(EXAMPLE);
        grid.sim(4);
        assert_eq!(4, grid.count());
    }
}
