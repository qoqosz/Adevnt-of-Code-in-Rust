use aoc::{aoc, aoc_input};
use rustc_hash::{FxHashMap, FxHashSet};
use std::{cmp::Reverse, collections::BinaryHeap};

#[derive(Default)]
struct Graph {
    adj: FxHashMap<u8, FxHashSet<u8>>,
    pred: FxHashMap<u8, FxHashSet<u8>>,
}

impl Graph {
    fn nodes(&self) -> impl Iterator<Item = &u8> {
        self.adj.keys()
    }

    fn remove_node(&mut self, u: &u8) {
        if let Some(adj) = self.adj.remove(u) {
            for v in &adj {
                if let Some(pred) = self.pred.get_mut(v) {
                    pred.remove(u);
                }
            }
        }
        if let Some(pred) = self.pred.remove(u) {
            for v in &pred {
                if let Some(adj) = self.adj.get_mut(v) {
                    adj.remove(u);
                }
            }
        }
    }

    fn remove_nodes_from<I>(&mut self, nodes: I)
    where
        I: IntoIterator<Item = u8>,
    {
        nodes.into_iter().for_each(|n| self.remove_node(&n));
    }

    fn add_edge(&mut self, u: u8, v: u8) {
        self.adj.entry(u).or_default().insert(v);
        self.adj.entry(v).or_default();
        self.pred.entry(v).or_default().insert(u);
        self.pred.entry(u).or_default();
    }

    fn add_edges_from<T>(&mut self, edges: T)
    where
        T: IntoIterator<Item = (u8, u8)>,
    {
        for edge in edges.into_iter() {
            self.add_edge(edge.0, edge.1);
        }
    }

    #[allow(dead_code)]
    fn edges(&self) -> impl Iterator<Item = (u8, u8)> {
        self.adj
            .iter()
            .flat_map(|(u, adj)| adj.iter().map(|v| (*u, *v)))
    }

    #[allow(dead_code)]
    fn to_dot(&self) -> String {
        format!(
            "digraph G {{\n{}}}",
            self.edges()
                .map(|(u, v)| format!("{} -> {};\n", u as char, v as char))
                .collect::<String>()
        )
    }

    fn in_degree(&self, u: &u8) -> usize {
        match self.pred.get(u) {
            Some(v) => v.len(),
            _ => 0,
        }
    }

    fn is_empty(&self) -> bool {
        self.adj.is_empty()
    }
}

impl TryFrom<&str> for Graph {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut graph = Graph::default();

        graph.add_edges_from(value.lines().map(|line| {
            let chars = line.as_bytes();
            (chars[5], chars[36])
        }));

        Ok(graph)
    }
}

fn lexicographic_sort(g: &Graph) -> Vec<u8> {
    let mut res = vec![];
    let mut in_degree_map = g
        .nodes()
        .map(|n| (*n, g.in_degree(n)))
        .collect::<FxHashMap<_, _>>();
    let mut queue = BinaryHeap::from_iter(
        in_degree_map
            .iter()
            .filter(|(_, deg)| **deg == 0)
            .map(|(n, _)| Reverse(*n)),
    );

    while let Some(Reverse(u)) = queue.pop() {
        res.push(u);

        if let Some(neighbors) = g.adj.get(&u) {
            for v in neighbors {
                if let Some(deg) = in_degree_map.get_mut(v) {
                    *deg = deg.saturating_sub(1);

                    if *deg == 0 {
                        queue.push(Reverse(*v))
                    }
                }
            }
        }
    }

    res
}

fn time_to_complete(g: &mut Graph) -> usize {
    let mut task_times: Vec<u8> = vec![];
    let mut tasks: Vec<u8> = vec![];
    let mut time = 0;

    while !g.is_empty() {
        if let Some(min_task) = g
            .nodes()
            .filter(|n| !tasks.contains(n) && g.in_degree(n) == 0)
            .min()
            && task_times.len() < 5
        {
            task_times.push(*min_task - 4);
            tasks.push(*min_task);
        } else if let Some(&min_time) = task_times.iter().min() {
            let completed = task_times
                .iter()
                .zip(tasks.iter())
                .filter(|(t, _)| **t == min_time)
                .map(|(_, task)| *task)
                .collect::<Vec<_>>();
            task_times.retain_mut(|v| match *v > min_time {
                true => {
                    *v -= min_time;
                    true
                }
                false => false,
            });
            tasks.retain(|t| !completed.contains(t));
            time += min_time as usize;
            g.remove_nodes_from(completed);
        }
    }

    time
}

#[aoc(2018, 7)]
pub fn main() {
    let data = aoc_input!(2018, 7).unwrap();
    let mut graph = Graph::try_from(data.as_str()).unwrap();

    // Part I
    println!("{}", str::from_utf8(&lexicographic_sort(&graph)).unwrap());

    // Part II
    println!("{}", time_to_complete(&mut graph));
}
