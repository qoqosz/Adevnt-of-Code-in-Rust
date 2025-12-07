use aoc::{aoc, aoc_input};
use rustc_hash::{FxHashMap, FxHashSet};
use std::{collections::VecDeque, fmt::Debug};

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum Cave {
    START,
    END,
    SMALL(String),
    BIG(String),
}

impl From<&str> for Cave {
    fn from(value: &str) -> Self {
        let lower = value.to_ascii_lowercase();
        let upper = value.to_ascii_uppercase();

        match value {
            "start" => Cave::START,
            "end" => Cave::END,
            _ if value == lower => Cave::SMALL(lower),
            _ if value == upper => Cave::BIG(upper),
            _ => unreachable!(),
        }
    }
}

#[derive(Default)]
struct CaveSystem(FxHashMap<Cave, FxHashSet<Cave>>);

impl CaveSystem {
    fn node_mut<T>(&mut self, a: T) -> &mut FxHashSet<Cave>
    where
        T: Into<Cave>,
    {
        self.0.entry(a.into()).or_insert(FxHashSet::default())
    }

    fn add_edge<T>(&mut self, a: T, b: T)
    where
        T: Into<Cave> + Clone,
    {
        self.node_mut(a.clone()).insert(b.clone().into());
        self.node_mut(b).insert(a.into());
    }

    fn count_paths(&self, extra_visit: bool) -> usize {
        let mut paths = 0;
        let mut queue = VecDeque::default();
        queue.push_back((vec![Cave::START], false));

        while let Some((path, mut has_extra_visit)) = queue.pop_front() {
            if let Some(cave) = path.last() {
                match cave {
                    Cave::START => {
                        if path.len() > 1 {
                            continue;
                        }
                    }
                    Cave::END => {
                        paths += 1;
                        continue;
                    }
                    Cave::SMALL(_) => {
                        if let Some((_, prev_path)) = path.split_last() {
                            if prev_path.contains(cave) {
                                match (extra_visit, has_extra_visit) {
                                    (true, true) => continue,
                                    (true, false) => has_extra_visit = true,
                                    (false, _) => continue,
                                }
                            }
                        }
                    }
                    _ => {}
                }

                if let Some(neighbors) = self.0.get(cave) {
                    for adj in neighbors {
                        let mut new_path = path.clone();
                        new_path.push(adj.clone());
                        queue.push_back((new_path, has_extra_visit));
                    }
                }
            }
        }

        paths
    }
}

#[aoc(2021, 12)]
pub fn main() {
    let data = aoc_input!(2021, 12).unwrap();
    let mut system = CaveSystem::default();

    for line in data.trim().lines() {
        let (a, b) = line.split_once('-').unwrap();
        system.add_edge(a, b);
    }

    // Part I
    println!("{}", system.count_paths(false));

    // Part II
    println!("{}", system.count_paths(true));
}
