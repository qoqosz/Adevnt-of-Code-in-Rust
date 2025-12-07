use aoc::{aoc, aoc_input};
use rand::seq::SliceRandom;
use rustc_hash::{FxHashMap, FxHashSet};
use std::fmt::{Debug, Display};
use std::hash::Hash;

type Graph<T> = FxHashMap<T, Vec<T>>;

trait UnGraph<T> {
    fn nodes(&self) -> FxHashSet<T>;
    fn edges(&self) -> Vec<(T, T)>;
}

impl<T> UnGraph<T> for Graph<T>
where
    T: Eq + PartialEq + Hash + Copy,
{
    fn nodes(&self) -> FxHashSet<T> {
        self.iter().fold(FxHashSet::default(), |mut acc, x| {
            acc.insert(*x.0);
            acc.extend(x.1);
            acc
        })
    }

    fn edges(&self) -> Vec<(T, T)> {
        self.iter()
            .flat_map(|(k, v)| v.iter().map(|x| (*k, *x)))
            .collect::<Vec<_>>()
    }
}

fn contract<T>(graph: &mut Graph<T>, u: T, v: T)
where
    T: Eq + PartialEq + Hash + Copy,
{
    if let Some(vals) = graph.remove(&v) {
        graph.entry(u).and_modify(|e| e.extend(vals));
    }

    for (k, w) in graph.iter_mut() {
        w.iter_mut().for_each(|node| {
            if *node == v {
                *node = u
            }
        });
        w.retain(|node| *node != *k);
    }

    graph.retain(|_, w| !w.is_empty());
}

fn karger<T>(graph: &mut Graph<T>) -> (usize, usize, usize)
where
    T: Eq + PartialEq + Hash + Copy,
{
    let mut rng = &mut rand::thread_rng();
    let mut weights: FxHashMap<T, usize> = FxHashMap::default();
    let n = graph.nodes().len();

    while graph.nodes().len() > 2 {
        let (u, v) = *graph.edges().choose(&mut rng).unwrap();
        contract(graph, u, v);

        let weight = weights.remove(&v).unwrap_or(1);
        *weights.entry(u).or_insert(1) += weight;
    }

    let i = *weights.iter().next().unwrap().1;
    let n_cuts: usize = graph.values().map(|v| v.len()).sum();
    (n_cuts, n - i, i)
}

fn solve<T>(graph: &Graph<T>) -> usize
where
    T: Eq + PartialEq + Hash + Copy + Debug + Display,
{
    loop {
        let (n_cuts, a, b) = karger(&mut graph.clone());

        if n_cuts == 3 {
            return a * b;
        }
    }
}

fn parse(data: &str) -> Graph<&str> {
    let mut graph: Graph<&str> = FxHashMap::default();

    for line in data.trim().lines() {
        let (key, vals) = line.rsplit_once(": ").unwrap();
        graph.insert(key, vals.split(' ').collect::<Vec<_>>());
    }

    graph
}

#[aoc(2023, 25)]
pub fn main() {
    let data = aoc_input!(2023, 25).unwrap();
    let graph = parse(&data);

    // Part I
    println!("{}", solve(&graph));
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

    #[test]
    fn test_part1() {
        let graph = parse(EXAMPLE);
        assert_eq!(solve(&graph), 54);
    }
}
