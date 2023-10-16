use aoc::load_input;
use std::collections::{hash_map::Keys, HashMap, HashSet};

type Visits = HashMap<(i32, i32), usize>;

#[derive(Debug)]
struct Santa {
    x: i32,
    y: i32,
    visited: Visits,
}

impl Default for Santa {
    fn default() -> Self {
        Santa::new(0, 0)
    }
}

impl Santa {
    fn new(x: i32, y: i32) -> Self {
        let mut santa = Santa {
            x: x,
            y: y,
            visited: HashMap::new(),
        };
        santa.increment(x, y);
        santa
    }

    fn increment(&mut self, x: i32, y: i32) {
        self.visited
            .entry((x, y))
            .and_modify(|cnt| *cnt += 1)
            .or_insert(1);
    }

    fn visit(&mut self, c: char) {
        match c {
            '^' => self.y += 1,
            'v' => self.y -= 1,
            '>' => self.x += 1,
            '<' => self.x -= 1,
            _ => {}
        };
        self.increment(self.x, self.y);
    }

    fn visit_path(&mut self, path: &str) {
        for c in path.chars() {
            self.visit(c);
        }
    }

    fn unique_visits(&self) -> Keys<(i32, i32), usize> {
        self.visited.keys()
    }
}

fn at_least_one_present(path: &str) -> usize {
    // Solution for part I
    let mut santa = Santa::default();
    santa.visit_path(path);
    santa.unique_visits().count()
}

fn work_with_robo_santa(path: &str) -> usize {
    // Solution for part II
    let mut santa = Santa::default();
    let mut robo_santa = Santa::default();

    for (i, c) in path.chars().enumerate() {
        if i % 2 == 0 {
            santa.visit(c)
        } else {
            robo_santa.visit(c)
        }
    }
    let mut santa_keys = santa.unique_visits().collect::<HashSet<&(i32, i32)>>();
    let robo_keys = robo_santa.unique_visits().collect::<HashSet<&(i32, i32)>>();

    santa_keys.extend(&robo_keys);
    santa_keys.iter().count()
}

fn main() {
    let data = load_input!("/Users/qoqosz/Documents/Coding/Rust/Advent of Code/data/2015/day3.txt");

    // Part I
    println!("{}", at_least_one_present(&data));

    // Part II
    println!("{}", work_with_robo_santa(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let inp = "^>v<";
        let mut santa = Santa::default();

        for c in inp.chars() {
            santa.visit(c);
        }

        println!("{:?}", santa);
    }
}
