use petgraph::algo::dijkstra;
use petgraph::data::Build;
use petgraph::graph::DiGraph;
use petgraph::visit::EdgeRef;

fn main() {
    let mut g: DiGraph<u32, u32> = DiGraph::new();
    let a = g.add_node(1);
    let b = g.add_node(2);
    let c = g.add_node(3);
    g.add_edge(a, b, 1);
    g.add_edge(a, c, 3);
    g.add_edge(b, c, 2);

    let scores = dijkstra(&g, a, None, |e| e.target().index());
    println!("{:?}", scores);
}
