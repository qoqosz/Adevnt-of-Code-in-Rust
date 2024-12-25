use aoc::{aoc, aoc_input};
use itertools::Itertools;
use petgraph::visit::{GetAdjacencyMatrix, IntoNeighbors, IntoNodeIdentifiers};
use petgraph::{prelude::GraphMap, Undirected};
use rustc_hash::FxHashSet;
use std::hash::Hash;
use std::iter::FromIterator;

/// Finds maximal cliques containing all the vertices in r, some of the
/// vertices in p, and none of the vertices in x.
fn bron_kerbosch_pivot<G>(
    g: G,
    adj_mat: &G::AdjMatrix,
    r: FxHashSet<G::NodeId>,
    mut p: FxHashSet<G::NodeId>,
    mut x: FxHashSet<G::NodeId>,
) -> Vec<FxHashSet<G::NodeId>>
where
    G: GetAdjacencyMatrix + IntoNeighbors,
    G::NodeId: Eq + Hash,
{
    let mut cliques = Vec::with_capacity(1);
    if p.is_empty() {
        if x.is_empty() {
            cliques.push(r);
        }
        return cliques;
    }
    // pick the pivot u to be the vertex with max degree
    let u = p.iter().max_by_key(|&v| g.neighbors(*v).count()).unwrap();
    let mut todo = p
        .iter()
        .filter(|&v| *u == *v || !g.is_adjacent(adj_mat, *u, *v) || !g.is_adjacent(adj_mat, *v, *u)) //skip neighbors of pivot
        .cloned()
        .collect::<Vec<G::NodeId>>();
    while let Some(v) = todo.pop() {
        let neighbors = FxHashSet::from_iter(g.neighbors(v));
        p.remove(&v);
        let mut next_r = r.clone();
        next_r.insert(v);

        let next_p = p
            .intersection(&neighbors)
            .cloned()
            .collect::<FxHashSet<G::NodeId>>();
        let next_x = x
            .intersection(&neighbors)
            .cloned()
            .collect::<FxHashSet<G::NodeId>>();

        cliques.extend(bron_kerbosch_pivot(g, adj_mat, next_r, next_p, next_x));

        x.insert(v);
    }

    cliques
}

fn maximal_cliques<G>(g: G) -> Vec<FxHashSet<G::NodeId>>
where
    G: GetAdjacencyMatrix + IntoNodeIdentifiers + IntoNeighbors,
    G::NodeId: Eq + Hash,
{
    let adj_mat = g.adjacency_matrix();
    let r = FxHashSet::default();
    let p = g.node_identifiers().collect::<FxHashSet<G::NodeId>>();
    let x = FxHashSet::default();
    return bron_kerbosch_pivot(g, &adj_mat, r, p, x);
}

fn parse(data: &str) -> GraphMap<&str, (), Undirected> {
    GraphMap::from_edges(data.trim().lines().flat_map(|line| line.split_once('-')))
}

#[aoc(2024, 23)]
pub fn main() {
    let data = aoc_input!(2024, 23).unwrap();
    let graph = parse(&data);

    // Part I
    let mut computers = FxHashSet::default();

    for node in graph.nodes().filter(|node| node.starts_with('t')) {
        for adjs in graph.neighbors(node).permutations(2) {
            let (n1, n2) = (adjs[0], adjs[1]);
            if graph.contains_edge(n1, n2) {
                let mut set = [node, n1, n2];
                set.sort_unstable();
                computers.insert(set);
            }
        }
    }

    println!("{}", computers.len());

    // Part II
    let password = maximal_cliques(&graph)
        .iter()
        .max_by_key(|x| x.len())
        .unwrap()
        .iter()
        .sorted_unstable()
        .join(",");
    println!("{password}");
}
