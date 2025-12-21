use aoc::{aoc, aoc_input};
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

struct Graph {
    adj: FxHashMap<String, FxHashSet<String>>,
}

impl Graph {
    fn new() -> Self {
        Self {
            adj: Default::default(),
        }
    }

    fn add_edge<S: Into<String>>(&mut self, u: S, v: S) {
        self.adj.entry(u.into()).or_default().insert(v.into());
    }
}

fn parse(data: &str) -> Graph {
    let mut g = Graph::new();

    for line in data.trim().lines() {
        if let Some((key, vals)) = line.split_once(": ") {
            for val in vals.split(' ') {
                g.add_edge(key, val);
            }
        }
    }

    g
}

fn count_paths<S>(g: &Graph, src: S, dst: S) -> usize
where
    S: Into<String>,
{
    let (src, dst) = (src.into(), dst.into());
    let mut cache = FxHashMap::default();

    fn dfs(g: &Graph, src: &String, dst: &String, cache: &mut FxHashMap<String, usize>) -> usize {
        if src == dst {
            return 1;
        }

        if let Some(&cached) = cache.get(src) {
            return cached;
        }

        let count = {
            if let Some(nodes) = g.adj.get(src) {
                nodes.iter().map(|n| dfs(g, n, dst, cache)).sum()
            } else {
                0
            }
        };

        cache.insert(src.clone(), count);

        count
    }

    dfs(g, &src, &dst, &mut cache)
}

#[aoc(2025, 11)]
pub fn main() {
    let data = aoc_input!(2025, 11).unwrap();
    let g = parse(&data);

    // Part I
    println!("{}", count_paths(&g, "you", "out"));

    // Part I
    let paths = [["svr", "dac", "fft", "out"], ["svr", "fft", "dac", "out"]];
    let n_paths = paths
        .iter()
        .map(|path| {
            path.iter()
                .tuple_windows()
                .map(|(u, v)| count_paths(&g, *u, *v))
                .product::<usize>()
        })
        .sum::<usize>();
    println!("{n_paths}");
}
