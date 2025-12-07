use aoc::{aoc, aoc_input};
use itertools::Itertools;

struct Grid<'a> {
    data: Vec<&'a [u8]>,
    width: usize,
    height: usize,
}

impl<'a> From<&'a str> for Grid<'a> {
    fn from(value: &'a str) -> Self {
        let data: Vec<_> = value.lines().map(|line| line.as_bytes()).collect();
        let width = data[0].len();
        let height = data.len();

        Grid {
            data,
            width,
            height,
        }
    }
}

impl Grid<'_> {
    fn get(&self, x: usize, y: usize) -> Option<&u8> {
        self.data.get(y).and_then(|row| row.get(x))
    }

    fn size(&self) -> usize {
        self.width * self.height
    }

    #[inline]
    fn iter_left(&self, x: usize, y: usize) -> impl Iterator<Item = u8> + '_ {
        (0..x).rev().map(move |i| self.data[y][i])
    }

    #[inline]
    fn iter_right(&self, x: usize, y: usize) -> impl Iterator<Item = u8> + '_ {
        ((x + 1)..(self.width)).map(move |i| self.data[y][i])
    }

    #[inline]
    fn iter_up(&self, x: usize, y: usize) -> impl Iterator<Item = u8> + '_ {
        (0..y).rev().map(move |i| self.data[i][x])
    }

    #[inline]
    fn iter_down(&self, x: usize, y: usize) -> impl Iterator<Item = u8> + '_ {
        ((y + 1)..(self.height)).map(move |i| self.data[i][x])
    }
}

fn visible_trees(grid: &Grid, x: isize, y: isize, dx: isize, dy: isize, seen: &mut [bool]) {
    let mut x = x;
    let mut y = y;
    let mut max_height = -1;

    while let Some(&tree) = grid.get(x as usize, y as usize) {
        if tree as isize > max_height {
            seen[y as usize * grid.width + x as usize] = true;
            max_height = tree as isize;
        }
        x += dx;
        y += dy;
    }
}

fn scenic_score(grid: &Grid, x: usize, y: usize) -> usize {
    let max_h = grid.data[y][x];

    fn count(iter: impl Iterator<Item = u8>, max_h: u8) -> usize {
        iter.take_while_inclusive(|tree| *tree < max_h).count()
    }

    let factors = [
        count(grid.iter_left(x, y), max_h),
        count(grid.iter_right(x, y), max_h),
        count(grid.iter_up(x, y), max_h),
        count(grid.iter_down(x, y), max_h),
    ];

    factors.iter().product()
}

#[aoc(2022, 8)]
pub fn main() {
    let data = aoc_input!(2022, 8).unwrap();
    let grid = Grid::from(data.as_str());
    let mut seen = vec![false; grid.size()];
    let (width, height) = (grid.width as isize, grid.height as isize);

    // Part I
    for x in 0..width {
        visible_trees(&grid, x, 0, 0, 1, &mut seen);
        visible_trees(&grid, x, height - 1, 0, -1, &mut seen);
    }
    for y in 0..height {
        visible_trees(&grid, 0, y, 1, 0, &mut seen);
        visible_trees(&grid, height - 1, y, -1, 0, &mut seen);
    }

    println!("{}", seen.iter().filter(|x| **x).count());

    // Part II
    let max_score = (0..width)
        .cartesian_product(0..height)
        .map(|(x, y)| scenic_score(&grid, x as usize, y as usize))
        .max()
        .unwrap();

    println!("{}", max_score);
}
