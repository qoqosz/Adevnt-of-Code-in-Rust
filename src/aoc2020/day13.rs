use aoc::aoc_input;

pub fn main() {
    let data = aoc_input!(2020, 13).unwrap();
    let timestamp0 = data
        .lines()
        .next()
        .map(|x| x.parse::<i64>().unwrap())
        .unwrap();

    println!("{timestamp0:?}");
}
