use aoc::{aoc, aoc_input};
use glam::I16Vec2 as Point;
use rustc_hash::{FxHashMap, FxHashSet};

fn parse(data: &str) -> (Vec<&str>, Vec<&str>) {
    let (left, right) = data.trim().split_once('\n').unwrap();
    (left.split(',').collect(), right.split(',').collect())
}

fn make_wire(path: &Vec<&str>) -> FxHashMap<Point, usize> {
    let (mut i, mut c) = (1, Point::new(0, 0));
    let mut covered = FxHashMap::default();

    for step in path {
        let (dir, len) = step.split_at(1);
        let n: usize = len.parse().unwrap();
        let ds = match dir {
            "R" => Point::new(1, 0),
            "L" => Point::new(-1, 0),
            "U" => Point::new(0, 1),
            "D" => Point::new(0, -1),
            _ => unreachable!(),
        };

        for _ in 0..n {
            c += ds;

            if !covered.contains_key(&c) {
                covered.insert(c, i);
            }

            i += 1;
        }
    }

    covered
}

#[aoc(2019, 3)]
pub fn main() {
    let data = aoc_input!(2019, 3).unwrap();
    let (left, right) = parse(&data);
    let (wire1, wire2) = (make_wire(&left), make_wire(&right));
    let crossings = wire1
        .keys()
        .filter(|k| wire2.contains_key(k))
        .collect::<FxHashSet<_>>();

    // Part I
    let dist = crossings
        .iter()
        .map(|c| c.x.abs() + c.y.abs())
        .min()
        .unwrap();
    println!("{dist}");

    // Part II
    let steps = crossings.iter().map(|c| wire1[c] + wire2[c]).min().unwrap();
    println!("{steps}");
}
