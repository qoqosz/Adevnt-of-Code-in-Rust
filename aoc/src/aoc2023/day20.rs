use aoc::{aoc, aoc_input};
use rustc_hash::FxHashMap;
use std::{collections::VecDeque, fmt::Debug};

const BROADCASTER: &str = "broadcaster";

type NodeId = usize;

#[derive(Debug)]
struct Broadcast {
    outputs: Vec<NodeId>,
}

#[derive(Debug)]
struct FlipFlop {
    state: bool,
    outputs: Vec<NodeId>,
}

#[derive(Debug)]
struct Conjunction {
    inputs: FxHashMap<NodeId, bool>,
    outputs: Vec<NodeId>,
}

trait Module: Debug {
    fn process(&mut self, source: NodeId, signal: bool) -> (Option<bool>, &[NodeId]);
}

/// When it receives a pulse, it sends the same pulse to all of its destination modules.
impl Module for Broadcast {
    fn process(&mut self, source: NodeId, signal: bool) -> (Option<bool>, &[NodeId]) {
        (Some(signal), &self.outputs)
    }
}

impl Module for FlipFlop {
    fn process(&mut self, source: NodeId, signal: bool) -> (Option<bool>, &[NodeId]) {
        if signal {
            (None, &[])
        } else {
            match self.state {
                false => {
                    self.state = true;
                    (Some(true), &self.outputs)
                }
                true => {
                    self.state = false;
                    (Some(false), &self.outputs)
                }
            }
        }
    }
}

impl Module for Conjunction {
    /// TODO: should add source `NodeId` if not in self.inputs
    fn process(&mut self, source: NodeId, signal: bool) -> (Option<bool>, &[NodeId]) {
        // update memory for the input source
        self.inputs.entry(source).and_modify(|e| *e = signal);
        (Some(!self.inputs.values().all(|v| *v)), &self.outputs)
    }
}

#[derive(Debug, Default)]
struct Machine {
    modules: Vec<Box<dyn Module>>,
}

impl Machine {
    fn insert(&mut self, module: Box<dyn Module>) -> NodeId {
        let idx = self.modules.len();
        self.modules.push(module);
        idx
    }

    // TODO: parametrize where is button and broadcaster
    fn push_button(&mut self, tr: &FxHashMap<NodeId, &str>) -> (usize, usize) {
        let mut queue = VecDeque::new();
        let (mut n_low, mut n_high) = (1, 0);

        queue.push_back((0, 0, false));

        while let Some((src, dest, signal)) = queue.pop_front() {
            let module = self.modules.get_mut(dest).unwrap();
            let (out, receivers) = module.process(src, signal);

            if let Some(sig) = out {
                for rec in receivers {
                    // let dest_name = tr.get(&dest).unwrap();
                    // let rec_name = tr.get(rec).unwrap();
                    // let sig_name = match sig {
                    //     false => "low",
                    //     true => "high",
                    // };
                    // println!("{} -{} => {}", dest_name, sig_name, rec_name);
                    match sig {
                        false => n_low += 1,
                        true => n_high += 1,
                    }
                    queue.push_back((dest, *rec, sig));
                }
            }
        }

        (n_low, n_high)
    }
}

fn get_pos<'a>(tr: &mut Vec<&'a str>, key: &'a str) -> usize {
    let idx = tr.iter().position(|&y| y == key);
    match idx {
        Some(val) => val,
        None => {
            tr.push(key);
            tr.len() - 1
        }
    }
}

fn parse(data: &str) {
    let mut tr: Vec<&str> = vec![]; //: FxHashMap<&str, NodeId> = FxHashMap::default();
    let mut modules: FxHashMap<usize, Box<dyn Module>> = FxHashMap::default();

    for line in data.trim().lines() {
        let (in_, out) = line.split_once(" -> ").unwrap();

        if in_.starts_with('b') {
            // get index of broadcaster
            let idx = get_pos(&mut tr, in_);

            let broadcaster = Broadcast {
                outputs: out
                    .split(", ")
                    .map(|c| get_pos(&mut tr, c))
                    .collect::<Vec<_>>(),
            };
            modules.insert(idx, Box::new(broadcaster));
        }
        if in_.starts_with('%') {
            let idx = get_pos(&mut tr, in_);

            let flip_flop = FlipFlop {
                state: false,
                outputs: out
                    .split(", ")
                    .map(|c| get_pos(&mut tr, c))
                    .collect::<Vec<_>>(),
            };
            modules.insert(idx, Box::new(flip_flop));
        }
        if in_.starts_with('&') {
            let idx = get_pos(&mut tr, in_);
            let conj = Conjunction {
                inputs: FxHashMap::default(),
                outputs: out
                    .split(", ")
                    .map(|c| get_pos(&mut tr, c))
                    .collect::<Vec<_>>(),
            };
            modules.insert(idx, Box::new(conj));
        }
    }
}

#[aoc(2023, 20)]
pub fn main() {
    let data = aoc_input!(2023, 20).unwrap();
    parse(&data);

    // Part I

    // Part II
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    // #[ignore]
    #[test]
    fn test_example1() {
        let broadcaster = Broadcast {
            outputs: vec![1, 2, 3],
        };
        let a = FlipFlop {
            state: false,
            outputs: vec![2],
        };
        let b = FlipFlop {
            state: false,
            outputs: vec![3],
        };
        let c = FlipFlop {
            state: false,
            outputs: vec![4],
        };
        let inv = Conjunction {
            inputs: FxHashMap::from_iter([(3, false)]),
            outputs: vec![1],
        };

        let mut machine = Machine::default();
        machine.insert(Box::new(broadcaster));
        machine.insert(Box::new(a));
        machine.insert(Box::new(b));
        machine.insert(Box::new(c));
        machine.insert(Box::new(inv));
        // println!("{:?}", machine);

        let tr =
            FxHashMap::from_iter([(0, "broadcaster"), (1, "a"), (2, "b"), (3, "c"), (4, "inv")]);

        let (a, b) = machine.push_button(&tr);
        println!("{}, {}", a, b);
    }

    #[ignore]
    #[test]
    // broadcaster -> a
    // %a -> inv, con
    // &inv -> b
    // %b -> con
    // &con -> output
    // a=1, inv=2, con=4, b=3
    fn test_example2() {
        let broadcaster = Broadcast { outputs: vec![1] };
        let a = FlipFlop {
            state: false,
            outputs: vec![2, 4],
        };
        let b = FlipFlop {
            state: false,
            outputs: vec![4],
        };
        let inv = Conjunction {
            inputs: FxHashMap::from_iter([(1, false)]),
            outputs: vec![3],
        };
        let con = Conjunction {
            inputs: FxHashMap::from_iter([(1, false), (3, false)]),
            outputs: vec![5],
        };
        let out = Broadcast { outputs: vec![] };

        let mut machine = Machine::default();
        machine.insert(Box::new(broadcaster));
        machine.insert(Box::new(a));
        machine.insert(Box::new(inv));
        machine.insert(Box::new(b));
        machine.insert(Box::new(con));
        machine.insert(Box::new(out));
        println!("{:?}", machine);

        let tr = FxHashMap::from_iter([
            (0, "broadcaster"),
            (1, "a"),
            (2, "inv"),
            (3, "b"),
            (4, "con"),
            (5, "output"),
        ]);

        machine.push_button(&tr);
        println!();
        machine.push_button(&tr);
        println!();
        machine.push_button(&tr);
        println!();
        machine.push_button(&tr);
    }
}
