use aoc::{aoc, aoc_input};
use itertools::Itertools;

#[derive(Debug)]
struct Grid {
    data: Vec<u8>,
    x: usize,
    y: usize,
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let x = value.find('\n').unwrap();
        let y = value.trim_end().matches('\n').count() + 1;
        let data = value
            .as_bytes()
            .iter()
            .filter(|ch| **ch != 10)
            .map(|ch| *ch - 48)
            .collect();
        Self { data, x, y }
    }
}

macro_rules! any {
    ($a:expr, $b:expr, $($args:expr),*) => {
        any!((($a) || ($b)), $($args),*)
    };
    ($a:expr, $b:expr) => {
        ($a) || ($b)
    };
}

macro_rules! prod {
    ($a:expr, $($args:expr),*) => {
        $a * prod!($($args),*)
    };

    ($a:expr) => {
        $a
    };
}

macro_rules! is_visible {
    ($it:expr, $height:expr) => {
        $it.all(|p| *p < $height)
    };
}

macro_rules! count_trees {
    ($it:expr, $height:expr) => {
        $it.take_while_inclusive(|&&p| p < $height).count()
    };
}

impl Grid {
    fn iter_pos(&self) -> impl Iterator<Item = (usize, usize)> {
        (1..(self.x - 1)).cartesian_product(1..(self.y - 1))
    }

    fn iter_left(&self, pos: (usize, usize)) -> impl Iterator<Item = &u8> {
        let start = self.y * pos.1;
        self.data[start..(start + pos.0)].iter().rev()
    }

    fn iter_right(&self, pos: (usize, usize)) -> impl Iterator<Item = &u8> {
        let start = self.y * pos.1;
        self.data[(start + pos.0 + 1)..(start + self.y)].iter()
    }

    fn iter_up(&self, pos: (usize, usize)) -> impl Iterator<Item = &u8> {
        let n = self.data.len() - 1;
        self.data
            .iter()
            .rev()
            .enumerate()
            .filter_map(move |(j, p)| match (n - j) % self.y == pos.0 {
                true => Some(p),
                _ => None,
            })
            .skip(self.y - pos.1)
    }

    fn iter_down(&self, pos: (usize, usize)) -> impl Iterator<Item = &u8> {
        self.data
            .iter()
            .enumerate()
            .filter_map(move |(j, p)| match j % self.y == pos.0 {
                true => Some(p),
                _ => None,
            })
            .skip(pos.1 + 1)
    }

    fn is_visible(&self, pos: (usize, usize)) -> bool {
        let height = self.data[pos.0 + self.x * pos.1];

        any!(
            is_visible!(self.iter_left(pos), height),
            is_visible!(self.iter_right(pos), height),
            is_visible!(self.iter_up(pos), height),
            is_visible!(self.iter_down(pos), height)
        )
    }

    fn count_visible(&self) -> usize {
        let count = self
            .iter_pos()
            .filter(|&(i, j)| self.is_visible((i, j)))
            .count();
        count + 2 * (self.x + self.y - 2)
    }

    fn scenic_score(&self, pos: (usize, usize)) -> usize {
        let height = self.data[pos.0 + self.y * pos.1];

        prod!(
            count_trees!(self.iter_left(pos), height),
            count_trees!(self.iter_right(pos), height),
            count_trees!(self.iter_up(pos), height),
            count_trees!(self.iter_down(pos), height)
        )
    }
}

#[aoc(2022, 8)]
pub fn main() {
    let data = aoc_input!(2022, 8).unwrap();
    let grid = Grid::from(data.as_str());

    // Part I
    println!("{}", grid.count_visible());

    // Part II
    println!(
        "{}",
        grid.iter_pos()
            .map(|pos| grid.scenic_score(pos))
            .max()
            .unwrap()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_part1() {
        let grid = Grid::from(EXAMPLE);
        assert_eq!(grid.count_visible(), 21);
    }

    #[test]
    fn test_part2() {
        let grid = Grid::from(EXAMPLE);
        assert_eq!(
            grid.iter_pos()
                .map(|pos| grid.scenic_score(pos))
                .max()
                .unwrap(),
            8
        );
    }
}
