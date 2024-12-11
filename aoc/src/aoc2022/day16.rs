use aoc::{aoc, aoc_input};
use itertools::{iproduct, Itertools};
use regex_lite::Regex;
use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp::{max, min};

#[derive(Clone, PartialEq, Eq, Hash)]
struct State<'a> {
    node: &'a str,
    flow: i32,
    opened: usize,
    t: i32,
}

#[derive(Default)]
struct Heap<'a> {
    i: usize,
    heap: Vec<State<'a>>,
}

impl<'a> Heap<'a> {
    fn next(&mut self) -> Option<&State<'a>> {
        self.heap.get(self.i).map(|x| {
            self.i += 1;
            x
        })
    }

    fn push(&mut self, item: State<'a>) {
        self.heap.push(item);
    }

    fn shrink(&mut self, max_size: usize) -> Self {
        let n = min(max_size, self.heap.len());
        let sorted = self
            .heap
            .iter()
            .sorted_by_key(|s| -s.flow)
            .take(n)
            .cloned()
            .collect::<Vec<_>>();

        Self { i: 0, heap: sorted }
    }
}

fn parse(data: &str) -> (FxHashMap<&str, i32>, FxHashMap<&str, Vec<&str>>) {
    let re = Regex::new(r"(.*?)([A-Z]{2})(.*?)rate=(\d+)(.*?)valve(s)* (.*?)\n").unwrap();
    let mut flow_rates: FxHashMap<&str, i32> = FxHashMap::default();
    let mut graph: FxHashMap<&str, Vec<&str>> = FxHashMap::default();

    for caps in re.captures_iter(data) {
        let valve = caps.get(2).unwrap().as_str();
        let flow = caps.get(4).unwrap().as_str().parse::<i32>().unwrap();
        let conns = caps
            .get(caps.len() - 1)
            .unwrap()
            .as_str()
            .split(", ")
            .collect::<Vec<_>>();

        flow_rates.insert(valve, flow);
        graph.insert(valve, conns);
    }

    (flow_rates, graph)
}

fn find_max_flow<'a>(
    flow_rates: &FxHashMap<&'a str, i32>,
    graph: &FxHashMap<&'a str, Vec<&'a str>>,
    start: &'a str,
    time: usize,
) -> FxHashMap<usize, i32> {
    let valve_index = flow_rates
        .keys()
        .enumerate()
        .map(|(i, valve)| (*valve, 1 << i))
        .collect::<FxHashMap<_, _>>();
    let init = State {
        node: start,
        flow: 0,
        opened: 0,
        t: time as i32,
    };
    let mut states = Heap {
        i: 0,
        heap: vec![init],
    };
    let mut visited = FxHashSet::default();
    let mut max_flow = FxHashMap::default();

    for _ in 0..time {
        let mut queue = Heap::default();

        while let Some(state) = states.next() {
            if !visited.insert(state.clone()) {
                continue;
            }

            if state.t < 0 {
                continue;
            }

            // move
            for adj in &graph[state.node] {
                queue.push(State {
                    node: *adj,
                    flow: state.flow,
                    opened: state.opened.clone(),
                    t: state.t - 1,
                });
            }

            // open valve
            let node_flow_rate = flow_rates[state.node];

            if node_flow_rate > 0 && (state.opened & valve_index[state.node]) == 0 {
                let opened = state.opened | valve_index[state.node];
                let new_flow = state.flow + node_flow_rate * (state.t - 1);
                max_flow
                    .entry(opened)
                    .and_modify(|x| *x = max(*x, new_flow))
                    .or_insert(new_flow);
                queue.push(State {
                    node: state.node,
                    flow: new_flow,
                    opened,
                    t: state.t - 1,
                });
            }

            // do nothing
            queue.push(State {
                t: state.t - 1,
                ..*state
            });
        }

        states = queue.shrink(6000);
    }

    max_flow
}

#[aoc(2022, 16)]
pub fn main() {
    let data = aoc_input!(2022, 16).unwrap();
    let (flow_rates, graph) = parse(&data);

    // Part I
    let max_flow = find_max_flow(&flow_rates, &graph, "AA", 30)
        .values()
        .copied()
        .max()
        .unwrap();
    println!("{max_flow}");

    // Part II
    let flows = find_max_flow(&flow_rates, &graph, "AA", 26);
    let max_flow = iproduct!(&flows, &flows)
        .filter(|(f, g)| f.0 & g.0 == 0)
        .map(|(f, g)| f.1 + g.1)
        .max()
        .unwrap();
    println!("{max_flow}");
}
