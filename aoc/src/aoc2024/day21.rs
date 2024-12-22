use aoc::{aoc, aoc_input};
use glam::I16Vec2 as Point;
use itertools::Itertools;
use memoize::memoize;
use rustc_hash::FxHashMap;
use std::{collections::VecDeque, sync::LazyLock};

static NUMERIC_KEYPAD: LazyLock<FxHashMap<Point, char>> = LazyLock::new(|| {
    FxHashMap::from_iter([
        (Point::new(0, 0), '7'),
        (Point::new(1, 0), '8'),
        (Point::new(2, 0), '9'),
        (Point::new(0, 1), '4'),
        (Point::new(1, 1), '5'),
        (Point::new(2, 1), '6'),
        (Point::new(0, 2), '1'),
        (Point::new(1, 2), '2'),
        (Point::new(2, 2), '3'),
        (Point::new(1, 3), '0'),
        (Point::new(2, 3), 'A'),
    ])
});

static DIRECTIONAL_KEYPAD: LazyLock<FxHashMap<Point, char>> = LazyLock::new(|| {
    FxHashMap::from_iter([
        (Point::new(1, 0), '^'),
        (Point::new(2, 0), 'A'),
        (Point::new(0, 1), '<'),
        (Point::new(1, 1), 'v'),
        (Point::new(2, 1), '>'),
    ])
});

fn find_all_shortest_paths(keypad: &FxHashMap<Point, char>, start: char, end: char) -> Vec<String> {
    let src = keypad
        .iter()
        .find_map(|(k, v)| if *v == start { Some(k) } else { None })
        .unwrap();
    let dst = keypad
        .iter()
        .find_map(|(k, v)| if *v == end { Some(k) } else { None })
        .unwrap();
    let mut paths = vec![];
    let mut queue = VecDeque::with_capacity(16);
    queue.push_back((0, *src, "".to_owned()));
    let mut distance = usize::MAX;

    while let Some((dist, state, path)) = queue.pop_front() {
        if !keypad.contains_key(&state) {
            continue;
        }
        if dist > distance {
            continue;
        }
        if state == *dst {
            if dist < distance {
                distance = dist;
                paths = vec![path];
            } else {
                paths.push(path);
            }
            continue;
        }
        queue.push_back((dist + 1, state + Point::new(-1, 0), format!("{path}<")));
        queue.push_back((dist + 1, state + Point::new(1, 0), format!("{path}>")));
        queue.push_back((dist + 1, state + Point::new(0, -1), format!("{path}^")));
        queue.push_back((dist + 1, state + Point::new(0, 1), format!("{path}v")));
    }

    paths
}

#[memoize]
fn find_shortest_sequence(code: String, depth: usize, numeric: bool) -> usize {
    let keypad = if numeric {
        &NUMERIC_KEYPAD
    } else {
        &DIRECTIONAL_KEYPAD
    };

    format!("A{code}")
        .chars()
        .tuple_windows()
        .map(|(a, b)| {
            let shortest_paths = find_all_shortest_paths(keypad, a, b);

            match depth {
                0 => shortest_paths[0].len() + 1,
                _ => shortest_paths
                    .iter()
                    .map(|path| find_shortest_sequence(format!("{path}A"), depth - 1, false))
                    .min()
                    .unwrap(),
            }
        })
        .sum()
}

fn complexity(code: &str, n_layers: usize) -> usize {
    let numeric: usize = code[..3].parse().unwrap();
    numeric * find_shortest_sequence(code.to_owned(), n_layers, true)
}

#[aoc(2024, 21)]
pub fn main() {
    let data = aoc_input!(2024, 21).unwrap();
    let codes = data.trim().lines();

    // Part I
    let score: usize = codes.clone().map(|code| complexity(code, 2)).sum();
    println!("{score}");

    // Part II
    let score: usize = codes.map(|code| complexity(code, 25)).sum();
    println!("{score}");
}
