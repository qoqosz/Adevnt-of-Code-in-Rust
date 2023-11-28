use aoc::aoc_input;

fn count_trees<'a>(mut map: impl Iterator<Item = &'a str>, shift: usize) -> usize {
    let n = map.next().unwrap().len();
    map.enumerate()
        .filter(|(i, line)| line.chars().nth((i + 1) * shift % n).unwrap() == '#')
        .count()
}

pub fn main() {
    let data = aoc_input!(2020, 3).unwrap();
    let iter = data.lines().filter(|x| !x.is_empty());

    // Part I
    println!("{}", count_trees(iter.clone(), 3));

    // Part II
    let dirs: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let res: usize = dirs
        .iter()
        .map(|(right, down)| count_trees(iter.clone().step_by(*down), *right))
        .product();
    println!("{}", res);
}
