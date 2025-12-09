use aoc::{aoc, aoc_input};
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, VecDeque};
use std::num::ParseIntError;

#[derive(PartialEq, Eq, Clone, Hash)]
struct JunctionBox {
    x: u64,
    y: u64,
    z: u64,
}

impl TryFrom<&str> for JunctionBox {
    type Error = ParseIntError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts = value.split(',').collect::<Vec<_>>();
        match &parts[..] {
            [x, y, z] => Ok(Self {
                x: x.parse()?,
                y: y.parse()?,
                z: z.parse()?,
            }),
            _ => unreachable!(),
        }
    }
}

struct Pair {
    u: JunctionBox,
    v: JunctionBox,
    dist2: u64,
}

impl Pair {
    const MAX: Pair = Pair {
        u: JunctionBox { x: 0, y: 0, z: 0 },
        v: JunctionBox { x: 0, y: 0, z: 0 },
        dist2: u64::MAX,
    };

    fn create(u: &JunctionBox, v: &JunctionBox) -> Self {
        let p = (u.x.abs_diff(v.x), u.y.abs_diff(v.y), u.z.abs_diff(v.z));
        let dist2 = p.0 * p.0 + p.1 * p.1 + p.2 * p.2;

        Self {
            u: u.clone(),
            v: v.clone(),
            dist2,
        }
    }
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        if self.dist2 == other.dist2 {
            let (a, b) = (self.u.clone(), self.v.clone());
            let (p, q) = (other.u.clone(), other.v.clone());

            (a == p && b == q) || (a == q && b == p)
        } else {
            false
        }
    }
}

impl Eq for Pair {}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.dist2.cmp(&other.dist2))
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist2.cmp(&other.dist2)
    }
}

struct Graph {
    adj: FxHashMap<JunctionBox, FxHashSet<JunctionBox>>,
}

impl Graph {
    fn new() -> Self {
        Self {
            adj: Default::default(),
        }
    }

    fn add_node(&mut self, u: JunctionBox) {
        self.adj.entry(u).or_default();
    }

    fn add_edge(&mut self, u: JunctionBox, v: JunctionBox) {
        self.adj.entry(u.clone()).or_default().insert(v.clone());
        self.adj.entry(v).or_default().insert(u);
    }

    fn connected_components(&self) -> Vec<FxHashSet<JunctionBox>> {
        let mut seen = FxHashSet::default();
        let mut components = vec![];

        for v in self.adj.keys() {
            if !seen.contains(v) {
                let c = self._bfs(v);
                seen.extend(c.iter().cloned());
                components.push(c);
            }
        }

        components
    }

    fn _bfs(&self, source: &JunctionBox) -> FxHashSet<JunctionBox> {
        let mut seen = FxHashSet::from_iter([source.clone()]);
        let mut queue = VecDeque::from_iter([source.clone()]);

        while let Some(u) = queue.pop_front() {
            for v in self.adj.get(&u).unwrap() {
                if seen.insert(v.clone()) {
                    queue.push_back(v.clone());
                }
            }
        }

        seen
    }
}

fn parse(data: &str) -> Vec<JunctionBox> {
    data.lines()
        .flat_map(JunctionBox::try_from)
        .collect::<Vec<_>>()
}

fn build_pairs(nodes: &[JunctionBox], n: usize) -> Vec<Pair> {
    let mut heap = BinaryHeap::with_capacity(n);
    let k = nodes.len();

    for i in 0..(k - 1) {
        for j in (i + 1)..k {
            let pair = Pair::create(&nodes[i], &nodes[j]);
            heap.push(pair);

            while heap.len() > n {
                heap.pop();
            }
        }
    }

    heap.into_iter().sorted_unstable().collect()
}

fn build_graph(nodes: &[JunctionBox], edges: &[Pair]) -> Graph {
    let mut g = Graph::new();

    for node in nodes {
        g.add_node(node.clone());
    }

    for pair in edges {
        g.add_edge(pair.u.clone(), pair.v.clone());
    }

    g
}

/// Solution - Part I
fn three_largest_circuits(g: &Graph) -> usize {
    g.connected_components()
        .iter()
        .map(|c| c.len())
        .sorted_unstable()
        .rev()
        .take(3)
        .product()
}

/// Solution - Part II
fn prod_x_last_2_junctions(g: &mut Graph) -> u64 {
    let mut min_pair = Pair::MAX;

    loop {
        let components = g.connected_components();

        if components.len() == 1 {
            return min_pair.u.x * min_pair.v.x;
        }

        min_pair = components
            .iter()
            .tuple_combinations()
            .flat_map(|(c1, c2)| {
                c1.iter()
                    .cartesian_product(c2.iter())
                    .map(|(u, v)| Pair::create(u, v))
                    .min()
            })
            .min()
            .unwrap();

        g.add_edge(min_pair.u.clone(), min_pair.v.clone());
    }
}

#[aoc(2025, 8)]
pub fn main() {
    let data = aoc_input!(2025, 8).unwrap();
    let nodes = parse(&data);
    let edges = build_pairs(&nodes, 1000);
    let mut g = build_graph(&nodes, &edges);

    // Part I
    println!("{}", three_largest_circuits(&g));

    // Part II
    println!("{}", prod_x_last_2_junctions(&mut g));
}
