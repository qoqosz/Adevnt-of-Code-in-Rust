use aoc::{aoc, aoc_input};
use num::Integer;
use rustc_hash::FxHashMap;
use std::hash::Hash;
use std::{collections::VecDeque, fmt::Debug};

const BROADCASTER: &str = "broadcaster";

#[derive(Debug, Default, Clone)]
struct Broadcast<K> {
    outputs: Vec<K>,
}

impl<K> Broadcast<K>
where
    K: Debug + Eq + PartialEq + Hash,
{
    fn new<I>(outputs: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<K>,
    {
        Self {
            outputs: outputs.into_iter().map(|x| x.into()).collect(),
        }
    }
}

#[derive(Debug, Clone)]
struct FlipFlop<K> {
    state: bool,
    outputs: Vec<K>,
}

impl<K> FlipFlop<K>
where
    K: Debug + Eq + PartialEq + Hash,
{
    fn new<I>(outputs: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<K>,
    {
        Self {
            state: false,
            outputs: outputs.into_iter().map(|x| x.into()).collect(),
        }
    }
}

#[derive(Debug, Clone)]
struct Conjunction<K> {
    inputs: FxHashMap<K, bool>,
    outputs: Vec<K>,
}

impl<K> Conjunction<K>
where
    K: Debug + Eq + PartialEq + Hash,
{
    fn new<I, J>(inputs: I, outputs: J) -> Self
    where
        I: IntoIterator,
        I::Item: Into<K>,
        J: IntoIterator,
        J::Item: Into<K>,
    {
        Self {
            inputs: inputs
                .into_iter()
                .map(|k| (k.into(), false))
                .collect::<FxHashMap<_, _>>(),
            outputs: outputs.into_iter().map(|x| x.into()).collect(),
        }
    }
}

trait Module<K>: Debug
where
    K: Debug + Eq + PartialEq + Hash,
{
    fn process(&mut self, source: K, signal: bool) -> Option<bool>;
    fn outputs(&self) -> &[K];
    fn reset(&mut self) {}
}

impl<K> Module<K> for Broadcast<K>
where
    K: Debug + Eq + PartialEq + Hash,
{
    #[allow(unused_variables)]
    fn process(&mut self, source: K, signal: bool) -> Option<bool> {
        Some(signal)
    }

    fn outputs(&self) -> &[K] {
        &self.outputs
    }
}

impl<K> Module<K> for FlipFlop<K>
where
    K: Debug + Eq + PartialEq + Hash,
{
    #[allow(unused_variables)]
    fn process(&mut self, source: K, signal: bool) -> Option<bool> {
        match signal {
            true => None,
            false => {
                self.state = !self.state;
                Some(self.state)
            }
        }
    }

    fn outputs(&self) -> &[K] {
        &self.outputs
    }

    fn reset(&mut self) {
        self.state = false;
    }
}

impl<K> Module<K> for Conjunction<K>
where
    K: Debug + Eq + PartialEq + Hash,
{
    fn process(&mut self, source: K, signal: bool) -> Option<bool> {
        *self.inputs.get_mut(&source).unwrap() = signal;
        Some(!self.inputs.values().all(|v| *v))
    }

    fn outputs(&self) -> &[K] {
        &self.outputs
    }

    fn reset(&mut self) {
        self.inputs.iter_mut().for_each(|(_, v)| *v = false);
    }
}

#[derive(Debug, Default)]
struct Machine<K> {
    modules: FxHashMap<K, Box<dyn Module<K>>>,
}

impl<K> Machine<K>
where
    K: Debug + Eq + PartialEq + Hash + Clone,
{
    fn reset(&mut self) {
        self.modules.iter_mut().for_each(|(_, m)| m.reset());
    }

    fn insert(&mut self, key: impl Into<K>, module: Box<dyn Module<K>>) {
        self.modules.insert(key.into(), module);
    }

    fn push_button(&mut self, broadcaster: impl Into<K>) -> (usize, usize) {
        let broadcaster = broadcaster.into();
        let mut queue = VecDeque::new();
        queue.push_back((broadcaster.clone(), false, broadcaster));
        let (mut n_low, mut n_high) = (1, 0);

        while let Some((source, signal, dest)) = queue.pop_front() {
            if let Some(module) = self.modules.get_mut(&dest) {
                if let Some(output) = module.process(source, signal) {
                    for rec in module.outputs() {
                        match output {
                            false => n_low += 1,
                            true => n_high += 1,
                        }
                        queue.push_back((dest.clone(), output, rec.clone()));
                    }
                }
            }
        }
        (n_low, n_high)
    }

    fn push_button_watch(&mut self, broadcaster: impl Into<K>, watch_list: &[K]) -> Option<K> {
        let broadcaster = broadcaster.into();
        let mut queue = VecDeque::new();
        queue.push_back((broadcaster.clone(), false, broadcaster));

        while let Some((source, signal, dest)) = queue.pop_front() {
            if watch_list.contains(&source) && signal {
                return Some(source.clone());
            }

            if let Some(module) = self.modules.get_mut(&dest) {
                if let Some(output) = module.process(source, signal) {
                    for rec in module.outputs() {
                        queue.push_back((dest.clone(), output, rec.clone()));
                    }
                }
            }
        }
        None
    }
}

fn parse_edges(data: &str) -> Vec<(&str, &str)> {
    let mut edges = Vec::new();

    for line in data.trim().lines() {
        let (module, outputs) = line.split_once(" -> ").unwrap();
        let outputs = outputs.split(", ");

        for output in outputs {
            edges.push((module.trim_start_matches(['%', '&']), output));
        }
    }

    edges
}

fn get_inputs<'a>(node: &'a str, edges: &'a [(&'a str, &'a str)]) -> Vec<&'a str> {
    edges.iter().filter(|e| e.1 == node).map(|e| e.0).collect()
}

fn parse(data: &str) -> Machine<String> {
    let mut machine = Machine::default();
    let edges = parse_edges(data);

    for line in data.trim().lines() {
        let (module, outputs) = line.split_once(" -> ").unwrap();
        let outputs = outputs.split(", ");

        match module.chars().next().unwrap() {
            'b' => machine.insert(module, Box::new(Broadcast::new(outputs))),
            '%' => machine.insert(
                module.trim_start_matches('%'),
                Box::new(FlipFlop::new(outputs)),
            ),
            '&' => {
                let key = module.trim_start_matches('&');
                let inputs = get_inputs(key, &edges);
                machine.insert(key, Box::new(Conjunction::new(inputs, outputs)));
            }
            _ => unreachable!(),
        }
    }

    machine
}

#[aoc(2023, 20)]
pub fn main() {
    let data = aoc_input!(2023, 20).unwrap();
    let mut machine = parse(&data);

    // Part I
    let (mut n_low, mut n_high) = (0, 0);

    for _ in 0..1000 {
        let out = machine.push_button(BROADCASTER);
        n_low += out.0;
        n_high += out.1;
    }
    println!("{}", n_low * n_high);

    // Part II
    machine.reset();
    // rx <- zp <- sb, nd, ds, hf
    let mut watch_list = vec![
        "sb".to_string(),
        "nd".to_string(),
        "ds".to_string(),
        "hf".to_string(),
    ];
    let (mut i, mut rx): (u64, u64) = (1, 1);

    while !watch_list.is_empty() {
        if let Some(module) = machine.push_button_watch(BROADCASTER, &watch_list) {
            rx = rx.lcm(&i);
            watch_list.retain(|w| *w != module);
        }
        i += 1;
    }
    println!("{rx}");
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE1: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    static EXAMPLE2: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    // #[ignore]
    #[test]
    fn test_example1_manual() {
        let mut machine: Machine<&str> = Machine::default();
        machine.insert("broadcaster", Box::new(Broadcast::new(["a", "b", "c"])));
        machine.insert("a", Box::new(FlipFlop::new(["b"])));
        machine.insert("b", Box::new(FlipFlop::new(["c"])));
        machine.insert("c", Box::new(FlipFlop::new(["inv"])));
        machine.insert("inv", Box::new(Conjunction::new(["c"], ["a"])));

        assert_eq!((8, 4), machine.push_button("broadcaster"));
        assert_eq!((8, 4), machine.push_button("broadcaster"));
    }

    // #[ignore]
    #[test]
    fn test_example2_manual() {
        let mut machine: Machine<&str> = Machine::default();
        machine.insert("broadcaster", Box::new(Broadcast::new(["a"])));
        machine.insert("a", Box::new(FlipFlop::new(["inv", "con"])));
        machine.insert("inv", Box::new(Conjunction::new(["a"], ["b"])));
        machine.insert("b", Box::new(FlipFlop::new(["con"])));
        machine.insert("con", Box::new(Conjunction::new(["a", "b"], ["output"])));

        assert_eq!((4, 4), machine.push_button("broadcaster"));
        assert_eq!((4, 2), machine.push_button("broadcaster"));
        assert_eq!((5, 3), machine.push_button("broadcaster"));
        assert_eq!((4, 2), machine.push_button("broadcaster"));
    }

    #[test]
    fn test_with_parse1() {
        let mut machine = parse(EXAMPLE1);

        assert_eq!((8, 4), machine.push_button("broadcaster"));
        assert_eq!((8, 4), machine.push_button("broadcaster"));
    }

    #[test]
    fn test_with_parse2() {
        let mut machine = parse(EXAMPLE2);

        assert_eq!((4, 4), machine.push_button("broadcaster"));
        assert_eq!((4, 2), machine.push_button("broadcaster"));
        assert_eq!((5, 3), machine.push_button("broadcaster"));
        assert_eq!((4, 2), machine.push_button("broadcaster"));
    }
}
