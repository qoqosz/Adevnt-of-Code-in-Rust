use aoc::{aoc, aoc_input};
use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    pos: (i32, i32),
    dir: (i32, i32),
}

impl State {
    fn new(start: (i32, i32)) -> Self {
        Self {
            pos: start,
            dir: (1, 0),
        }
    }

    fn move_forward(&self) -> Self {
        Self {
            pos: (self.pos.0 + self.dir.0, self.pos.1 + self.dir.1),
            dir: self.dir,
        }
    }

    fn turn_left(&self) -> Self {
        Self {
            pos: self.pos,
            dir: (self.dir.1, -self.dir.0),
        }
    }

    fn turn_right(&self) -> Self {
        Self {
            pos: self.pos,
            dir: (-self.dir.1, self.dir.0),
        }
    }
}

fn find_in_map(map: &FxHashMap<(i32, i32), char>, x: char) -> Option<(i32, i32)> {
    map.iter().find(|(_, ch)| **ch == x).map(|(pos, _)| *pos)
}

fn find_best_paths(
    map: &FxHashMap<(i32, i32), char>,
    start: (i32, i32),
    end: (i32, i32),
) -> (Option<i32>, Vec<Vec<(i32, i32)>>) {
    let mut best_score = i32::MAX;
    let mut best_paths = vec![];
    let mut visited = FxHashMap::default();
    let mut queue = VecDeque::with_capacity(10_000);
    queue.push_back((0, State::new(start), vec![start]));

    while let Some((score, state, path)) = queue.pop_front() {
        // can't be the best path
        if score > best_score {
            continue;
        }

        // has been visited?
        if let Some(&prev_score) = visited.get(&state) {
            // yes and with better score
            if prev_score < score {
                continue;
            }
        }
        // prev_score >= score -> update the score
        visited.insert(state.clone(), score);

        // at the end
        if state.pos == end {
            if score == best_score {
                best_paths.push(path);
            } else {
                best_score = score;
                best_paths = vec![path];
            }
            continue;
        }

        // turn left or right
        queue.push_back((score + 1000, state.turn_left(), path.clone()));
        queue.push_back((score + 1000, state.turn_right(), path.clone()));

        // move forward
        let next_state = state.move_forward();

        if map.get(&next_state.pos) != Some(&'#') {
            let mut next_path = path.clone();
            next_path.push(next_state.pos);
            queue.push_back((score + 1, next_state, next_path));
        }
    }

    (
        if best_score < i32::MAX {
            Some(best_score)
        } else {
            None
        },
        best_paths,
    )
}

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

#[aoc(2024, 16)]
pub fn main() {
    let data = aoc_input!(2024, 16).unwrap();
    let map = parse(&data);
    let start = find_in_map(&map, 'S').unwrap();
    let end = find_in_map(&map, 'E').unwrap();
    let (best_score, best_paths) = find_best_paths(&map, start, end);

    // Part I
    println!("{}", best_score.unwrap());

    // Part II
    println!("{}", best_paths.iter().flatten().unique().count());
}
