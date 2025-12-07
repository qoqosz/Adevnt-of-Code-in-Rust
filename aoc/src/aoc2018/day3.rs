use aoc::{aoc, aoc_input};
use rustc_hash::FxHashMap;

struct Claim {
    id: usize,
    pos: (usize, usize),
    shape: (usize, usize),
}

impl Claim {
    fn area(&self) -> usize {
        self.shape.0 * self.shape.1
    }
}

fn parse(data: &str) -> Vec<Claim> {
    data.trim()
        .lines()
        .map(|line| {
            let (id, l) = line.split_once(" @ ").unwrap();
            let (a, b) = l.split_once(": ").unwrap();
            let (x, y) = a.split_once(',').unwrap();
            let (w, h) = b.split_once('x').unwrap();

            Claim {
                id: id[1..].parse().unwrap(),
                pos: (x.parse().unwrap(), y.parse().unwrap()),
                shape: (w.parse().unwrap(), h.parse().unwrap()),
            }
        })
        .collect()
}

fn claim(grid: &mut FxHashMap<(usize, usize), Vec<usize>>, claim: &Claim) {
    for i in 0..claim.shape.0 {
        for j in 0..claim.shape.1 {
            let p = (claim.pos.0 + i, claim.pos.1 + j);
            grid.entry(p).or_insert(vec![]).push(claim.id);
        }
    }
}

#[aoc(2018, 3)]
pub fn main() {
    let data = aoc_input!(2018, 3).unwrap();
    let claims = parse(&data);
    let mut grid = FxHashMap::default();

    // Part I
    for c in &claims {
        claim(&mut grid, c);
    }
    let area = grid.values().map(|c| c.len()).filter(|v| *v >= 2).count();
    println!("{area}");

    // Part II
    let mut mapping = FxHashMap::default();

    for (pos, ids) in grid.iter().filter(|(_, v)| v.len() == 1) {
        for id in ids {
            mapping.entry(id).or_insert(vec![]).push(pos);
        }
    }

    let id = mapping
        .iter()
        .find(|(id, ps)| claims[**id - 1].area() == ps.len())
        .map(|x| x.0)
        .unwrap();
    println!("{id}");
}
