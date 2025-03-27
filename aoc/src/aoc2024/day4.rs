use aoc::{aoc, aoc_input};
use rustc_hash::FxHashMap;

static XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

fn parse(data: &str) -> FxHashMap<(i32, i32), char> {
    data.trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, ch)| ((x as i32, y as i32), ch))
        })
        .collect()
}

// Part I
fn xmas_count(grid: &FxHashMap<(i32, i32), char>) -> usize {
    grid.iter()
        .filter(|&(_, &v)| v == 'X')
        .map(|(k, _)| count_all_dirs(&grid, k))
        .sum::<usize>()
}

fn count_all_dirs(grid: &FxHashMap<(i32, i32), char>, key: &(i32, i32)) -> usize {
    let mut count = 0;
    let dirs = vec![
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    for dir in dirs {
        let mut is_xmas = true;

        for i in 1..4 {
            let new_key = (key.0 + i * dir.0, key.1 + i * dir.1);

            if let Some(ch) = grid.get(&new_key) {
                if *ch != XMAS[i as usize] {
                    is_xmas = false;
                }
            } else {
                is_xmas = false;
                break;
            }
        }

        if is_xmas {
            count += 1;
        }
    }

    count
}

// Part II
fn x_mas_count(grid: &FxHashMap<(i32, i32), char>) -> usize {
    grid.iter()
        .filter(|&(_, &v)| v == 'A')
        .filter(|(k, _)| is_mas_cross(&grid, k))
        .count()
}

fn is_mas_cross(grid: &FxHashMap<(i32, i32), char>, key: &(i32, i32)) -> bool {
    let is_ms = |x: &(char, char)| *x == ('M', 'S') || *x == ('S', 'M');

    let left = (
        *grid.get(&(key.0 - 1, key.1 + 1)).unwrap_or(&' '),
        *grid.get(&(key.0 + 1, key.1 - 1)).unwrap_or(&' '),
    );
    let right = (
        *grid.get(&(key.0 - 1, key.1 - 1)).unwrap_or(&' '),
        *grid.get(&(key.0 + 1, key.1 + 1)).unwrap_or(&' '),
    );

    is_ms(&left) && is_ms(&right)
}

#[aoc(2024, 4)]
pub fn main() {
    let data = aoc_input!(2024, 4).unwrap();
    let grid = parse(&data);

    // Part I
    let n = xmas_count(&grid);
    println!("{n}");

    // Part II
    let n = x_mas_count(&grid);
    println!("{n}");
}
