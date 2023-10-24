use std::collections::HashMap;

use aoc::{aoc_input, load_input};
use itertools::Itertools;
use petgraph::{data::Build, graph::Node, prelude::*, Graph};

#[derive(Clone)]
struct Edge {
    person: String,
    neighbor: String,
    delta: i32,
}

impl Edge {
    fn rev(&self) -> Self {
        Edge {
            person: self.neighbor.clone(),
            neighbor: self.person.clone(),
            delta: self.delta,
        }
    }
}

impl From<&str> for Edge {
    fn from(line: &str) -> Self {
        let words = line
            .trim_end_matches('.')
            .split(' ')
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        assert_eq!(words.len(), 11);
        let person: String = words[0].clone();
        let neighbor: String = words.last().unwrap().clone();
        let sign = match words[2].as_ref() {
            "gain" => 1,
            _ => -1,
        };
        let delta = sign * words[3].parse::<i32>().unwrap();

        Edge {
            person,
            neighbor,
            delta,
        }
    }
}

#[derive(Debug)]
struct Attendees {
    people: HashMap<String, NodeIndex>,
    graph: Graph<(), i32>,
}

impl Attendees {
    fn new() -> Self {
        Attendees {
            people: HashMap::new(),
            graph: Graph::new(),
        }
    }

    fn get_index_mut(&mut self, person: &String) -> NodeIndex {
        *self
            .people
            .entry(person.clone())
            .or_insert_with(|| self.graph.add_node(()))
    }

    fn get_index(&self, person: &String) -> Option<&NodeIndex> {
        self.people.get(person)
    }

    fn add_edge(&mut self, edge: &Edge) {
        let from = self.get_index_mut(&edge.person);
        let to = self.get_index_mut(&edge.neighbor);
        self.graph.add_edge(from, to, edge.delta);
    }

    fn happiness(&self, arrangement: &Vec<&String>) -> i32 {
        let mut cost = 0;

        for (&person, &neighbor) in arrangement.iter().zip(arrangement.iter().cycle().skip(1)) {
            let p1 = self.get_index(person).unwrap();
            let p2 = self.get_index(neighbor).unwrap();
            let e1 = self.graph.find_edge(*p1, *p2).unwrap();
            let e2 = self.graph.find_edge(*p2, *p1).unwrap();
            cost += self.graph.edge_weight(e1).unwrap() + self.graph.edge_weight(e2).unwrap();
        }

        cost
    }

    fn optimal_happiness(&self) -> i32 {
        let mut people = self.people.keys().collect::<Vec<_>>();
        let n = people.len();
        let person = people.pop().unwrap();

        people
            .into_iter()
            .permutations(n - 1)
            .map(|cycle| {
                let mut arr = cycle.clone();
                arr.push(&person);
                self.happiness(&arr)
            })
            .max()
            .unwrap()
    }

    fn add_myself(&mut self) {
        let me = "me".to_string();
        let my_idx = self.get_index_mut(&me);
        let people = self.people.clone();

        for person in people.keys() {
            let edge = Edge {
                person: me.clone(),
                neighbor: person.clone(),
                delta: 0,
            };
            self.add_edge(&edge);
            self.add_edge(&edge.rev());
        }
    }
}

fn main() {
    let data = aoc_input!(2015, 13).unwrap();
    let mut attendees = Attendees::new();

    for line in data.split('\n').filter(|x| !x.is_empty()) {
        let edge = Edge::from(line);
        attendees.add_edge(&edge);
    }

    // Part I
    println!("{}", attendees.optimal_happiness());

    // Part II
    attendees.add_myself();
    println!("{}", attendees.optimal_happiness());
}
