use aoc::{aoc, aoc_input};
use std::str;
use std::str::FromStr;

// Grammar:
//     list    : LPAREN node (COMMA node)* RPAREN
//     node    : number | list
//     number  : [0-9]*

type List = Vec<Node>;

#[derive(Debug, Eq, Ord)]
enum Node {
    Number(u8),
    List(List),
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Node::Number(x), &Node::Number(y)) => x == y,
            (&Node::List(ref x), &Node::List(ref y)) => x == y,
            _ => false,
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (&Node::Number(ref x), &Node::Number(ref y)) => x.partial_cmp(y),
            (&Node::List(ref x), &Node::List(ref y)) => x.partial_cmp(y),
            (&Node::Number(ref x), &Node::List(_)) => {
                Node::List(vec![Node::Number(*x)]).partial_cmp(other)
            }
            (&Node::List(ref x), &Node::Number(ref y)) => x.partial_cmp(&vec![Node::Number(*y)]),
        }
    }
}

impl FromStr for Node {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_node(s).map(|(node, _)| node)
    }
}

fn parse_list(input: &str) -> Result<(Node, &str), ()> {
    let mut items = vec![];
    let mut input = &input[1..];

    while !input.starts_with(']') {
        let (item, rem) = parse_node(input)?;
        items.push(item);
        input = rem;

        if input.starts_with(',') {
            input = &input[1..];
        }
    }
    input = &input[1..];

    Ok((Node::List(items), input))
}

fn parse_node(input: &str) -> Result<(Node, &str), ()> {
    if input.starts_with('[') {
        parse_list(input)
    } else {
        parse_number(input)
    }
}

fn parse_number(input: &str) -> Result<(Node, &str), ()> {
    let mut end = 0;

    while input.as_bytes()[end].is_ascii_digit() {
        end += 1;
    }

    let (num, rem) = input.split_at(end);
    let num = num.parse::<u8>().map_err(|_| ())?;

    Ok((Node::Number(num), rem))
}

#[aoc(2022, 13)]
pub fn main() {
    let data = aoc_input!(2022, 13).unwrap();
    let node_pairs: Vec<_> = data
        .trim()
        .split("\n\n")
        .map(|pair| {
            let (left, right) = pair.split_once('\n').unwrap();
            let left = Node::from_str(left).unwrap();
            let right = Node::from_str(right).unwrap();
            (left, right)
        })
        .collect();

    // Part I
    let sum = node_pairs
        .iter()
        .enumerate()
        .filter(|(_, (left, right))| left < right)
        .map(|(i, _)| i + 1)
        .sum::<usize>();
    println!("{sum}");

    // Part II
    let mut nodes = vec![];
    let left: Node = "[[2]]".parse().unwrap();
    let right: Node = "[[6]]".parse().unwrap();
    nodes.push(&left);
    nodes.push(&right);

    for node in &node_pairs {
        nodes.push(&node.0);
        nodes.push(&node.1);
    }

    nodes.sort_unstable();

    let i = nodes.iter().position(|x| **x == left).unwrap();
    let j = nodes.iter().position(|x| **x == right).unwrap();

    println!("{}", (i + 1) * (j + 1));
}
