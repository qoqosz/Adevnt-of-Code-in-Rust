use aoc::{aoc, aoc_input};

#[inline(always)]
fn reduce_fuel(x: &u32) -> u32 {
    (x / 3).saturating_sub(2)
}

#[inline(always)]
fn reduce_fuel_total(x: &u32) -> u32 {
    let mut x = *x;
    let mut total = 0;

    while x > 0 {
        x = reduce_fuel(&x);
        total += x;
    }

    total
}

#[aoc(2019, 1)]
pub fn main() {
    let data = aoc_input!(2019, 1).unwrap();
    let fuels = data
        .lines()
        .flat_map(|x| x.parse::<u32>())
        .collect::<Vec<_>>();

    // Part I
    let p1 = fuels.iter().map(reduce_fuel).sum::<u32>();
    println!("{p1}");

    // Part  II
    let p2 = fuels.iter().map(reduce_fuel_total).sum::<u32>();
    println!("{p2}");
}
